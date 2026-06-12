use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use winreg::RegKey;
use winreg::enums::{HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER, KEY_READ};

fn scan_uninstall_registry(index: &mut Vec<AppEntry>) {
    let roots = [
        (RegKey::predef(HKEY_LOCAL_MACHINE), "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall"),
        (RegKey::predef(HKEY_CURRENT_USER), "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall"),
    ];
    for (root, sub_path) in &roots {
        if let Ok(key) = root.open_subkey_with_flags(sub_path, KEY_READ) {
            for sub_name in key.enum_keys().filter_map(Result::ok) {
                if let Ok(app_key) = key.open_subkey(&sub_name) {
                    if let Ok(name) = app_key.get_value::<String, _>("DisplayName") {
                        if name.trim().is_empty() { continue; }
                        let path_str: Result<String, _> = app_key.get_value("DisplayIcon")
                            .or_else(|_| app_key.get_value("UninstallString"));
                        let exe_path = path_str.ok().and_then(|s| {
                            let first = s.split_whitespace().next()?;
                            Some(PathBuf::from(first.trim_matches('"')))
                        }).unwrap_or_else(|| PathBuf::new());
                        index.push(AppEntry {
                            name,
                            path: exe_path,
                            priority: 80,
                            is_dir: false,
                        });
                    }
                }
            }
        }
    }
}

use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use bincode::{Encode, Decode};

#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct AppEntry {
    pub name: String,
    pub path: PathBuf,
    pub priority: u8,
    pub is_dir: bool,
}

pub const BLACKLIST: &[&str] = &[
    "unins",
    "uninstall",
    "desinstalar",
    "setup",
    "installer",
    "install",
    "msiexec",
    "vcredist",
    "dotnet-runtime",
    "bootstrapper",
    "clicktorun",
    "dxwebsetup",
    "update",
    "updater",
    "autoupdate",
    "patcher",
    "maint",
    "fix",
    "helper",
    "broker",
    "host",
    "agent",
    "service",
    "background",
    "proxy",
    "watchdog",
    "daemon",
    "bridge",
    "overlay",
    "telemetry",
    "monitor",
    "commandline",
    "headless",
    "launcher_helper",
    "driver",
    "vulkan",
    "physx",
    "notification_helper",
    "crash_handler",
    "crash",
    "diagnostics",
    "troubleshoot",
    "error",
    "crashreporter",
    "crashhandler",
    "dump",
    "report",
    "log",
    "feedback",
    "msinfo",
    "systemsettings",
    "toastnotification",
    "microsoft.windows.",
    "softwarelogo",
    "adminflows",
    "sysinfo",
    "coretools",
    "runtimebroker",
    "sihost",
    "ctfmon",
    "dllhost",
    "rundll",
    "conhost",
    "csrss",
    "svchost",
    "wininit",
    "winlogon",
    "lsass",
    "smss",
    "fontview",
    "atbroker",
    "systemreset",
    "isoburn",
    "magnify",
    "narrator",
    "osk",
    "sysprep",
    "wsreset",
    "taskhost",
    "notification_helper",
    "nacl",
    "swiftshader",
    "widevine",
    "clearkey",
    "srl",
    "squirrel",
    "nuget",
    "chocolatey",
    "elevation_service",
    "readme",
    "license",
    "changelog",
    "credits",
    "copyright",
    "legal",
    "manifest",
    "metadata",
    "config",
    "settings",
];

const DIR_BLACKLIST: &[&str] = &[
    "node_modules",
    "target",
    ".git",
    ".svn",
    "dist",
    "build",
    "temp",
    "tmp",
    "cache",
    "logs",
    "appdata\\local\\temp",
    "windows\\winsxs",
    "windows\\servicing",
    "windows\\softwaredistribution",
    "common files",
    "microsoft shared",
    "steamapps\\common",
    ".vs",
    ".idea",
    ".vscode",
    "vendor",
    "obj",
    "bin",
    "packages",
    "package cache",
    "microsoft\\windowsapps",
    "appdata\\local\\packages",
];

const DOCS_TERMS: &[&str] = &[
    "documentation",
    "help",
    "readme",
    "manual",
    "license",
    "changelog",
    "credits",
    "legal",
    "faq",
];

fn calculate_priority(path: &Path, is_uwp: bool) -> u8 {
    if is_uwp {
        return 100;
    }

    let p = path.to_string_lossy().to_lowercase();
    let name = path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();

    if DOCS_TERMS
        .iter()
        .any(|term| name.contains(term) || p.contains(term))
    {
        return 1;
    }

    if p.contains("start menu") && p.ends_with(".lnk") {
        return 100;
    } else if p.contains("desktop") {
        return 80;
    } else if p.contains("system32") {
        return 30;
    } else if p.contains("documents") || p.contains("documentos") {
        return 40;
    }

    10
}

fn is_blacklisted(name: &str) -> bool {
    let name_lower = name.to_lowercase();
    BLACKLIST.iter().any(|term| name_lower.contains(term))
}

fn is_dir_blacklisted(dir_name: &str) -> bool {
    let d = dir_name.to_lowercase();
    DIR_BLACKLIST.iter().any(|term| d.contains(term))
}

fn base_name(name: &str) -> String {
    let n = name.to_lowercase();
    let suffixes = [
        " setup",
        " installer",
        " uninstaller",
        " uninstall",
        " updater",
        " helper",
        " service",
        " launcher",
        " crash handler",
        " crashhandler",
        " crashreporter",
        " diagnostics",
        " troubleshooter",
        " compatibility",
    ];
    let mut result = n.clone();
    for suffix in &suffixes {
        if let Some(pos) = result.rfind(suffix) {
            result.truncate(pos);
        }
    }
    result.trim().to_string()
}

pub fn build_index() -> Vec<AppEntry> {
    if let Some(saved) = load_index() {
        return saved;
    }
    let mut index = Vec::new();

    if let Some(mut user_path) = dirs::data_dir() {
        user_path.push("Microsoft\\Windows\\Start Menu\\Programs");
        scan_directory(&user_path, &mut index, &["lnk"], 5, false);
    }
    let sys_start_menu = Path::new("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs");
    scan_directory(sys_start_menu, &mut index, &["lnk"], 5, false);

    if let Ok(output) = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "chcp 65001 >$null; Get-StartApps | ForEach-Object { \"$($_.Name)|$($_.AppID)\" }",
        ])
        .output()
    {
        let result_string = String::from_utf8_lossy(&output.stdout);
        for line in result_string.lines() {
            let parts: Vec<&str> = line.splitn(2, '|').collect();
            if parts.len() == 2 {
                let name = parts[0].trim().to_string();
                let app_id = parts[1].trim().to_string();

                if !name.is_empty() && !is_blacklisted(&name) && !app_id.contains("Internal") {
                    let path = PathBuf::from(format!("UWP:{}", app_id));
                    index.push(AppEntry {
                        name,
                        path,
                        priority: 100,
                        is_dir: false,
                    });
                }
            }
        }
    }

    scan_uninstall_registry(&mut index);

    let system_tools = [
        "cmd.exe",
        "powershell.exe",
        "control.exe",
        "taskmgr.exe",
        "regedit.exe",
        "notepad.exe",
    ];
    for tool in system_tools {
        let path = PathBuf::from("C:\\Windows\\System32").join(tool);
        if path.exists() {
            index.push(AppEntry {
                name: tool.replace(".exe", "").to_string(),
                path,
                priority: 9,
                is_dir: false,
            });
        }
    }

    if let Some(desktop) = dirs::desktop_dir() {
        scan_directory(&desktop, &mut index, &["lnk", "exe"], 1, true);
    }
    if let Some(docs) = dirs::document_dir() {
        scan_directory(&docs, &mut index, &["lnk"], 1, true);
    }
    if let Some(pics) = dirs::picture_dir() {
        scan_directory(&pics, &mut index, &["lnk"], 1, true);
    }
    if let Some(downloads) = dirs::download_dir() {
        scan_directory(&downloads, &mut index, &["lnk", "exe"], 1, true);
    }

    let mut groups: HashMap<String, AppEntry> = HashMap::with_capacity(index.len());
    for entry in index {
        let key = base_name(&entry.name);
        if key.is_empty() { continue; }
        match groups.get(&key) {
            Some(existing) if existing.priority >= entry.priority => {}
            _ => { groups.insert(key, entry); }
        }
    }

    let mut deduplicated: Vec<AppEntry> = groups.into_values().collect();
    deduplicated.sort_by(|a, b| {
        b.priority
            .cmp(&a.priority)
            .then_with(|| a.name.cmp(&b.name))
    });
    if let Err(e) = save_index(&deduplicated) {
        eprintln!("Failed to persist index: {e}");
    }
    deduplicated
}

fn scan_directory(
    dir: &Path,
    index: &mut Vec<AppEntry>,
    allowed_extensions: &[&str],
    max_depth: usize,
    include_dirs: bool,
) {
    if !dir.exists() {
        return;
    }

    let walker = WalkDir::new(dir)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !is_dir_blacklisted(&name)
        });

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if is_blacklisted(&name) {
            continue;
        }

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if allowed_extensions.contains(&ext.to_lowercase().as_str()) {
                    let priority = calculate_priority(path, false);
                    index.push(AppEntry {
                        name,
                        path: path.to_path_buf(),
                        priority,
                        is_dir: false,
                    });
                }
            }
        } else if include_dirs && path.is_dir() {
            if !name.starts_with('.') && !is_dir_blacklisted(&name) {
                let priority = calculate_priority(path, false);
                index.push(AppEntry {
                    name,
                    path: path.to_path_buf(),
                    priority,
                    is_dir: true,
                });
            }
        }
    }
}

fn persisted_index_path() -> PathBuf {
    let mut base = dirs::config_dir().unwrap_or_else(|| Path::new(".").to_path_buf());
    base.push("GlimpseLauncher");
    base.push("index.bin");
    base
}

fn save_index(index: &Vec<AppEntry>) -> Result<(), std::io::Error> {
    let path = persisted_index_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let encoded = bincode::encode_to_vec(index, bincode::config::standard())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    std::fs::write(path, encoded)
}

fn load_index() -> Option<Vec<AppEntry>> {
    let path = persisted_index_path();
    if !path.exists() {
        return None;
    }
    match std::fs::read(&path) {
        Ok(bytes) => match bincode::decode_from_slice(&bytes, bincode::config::standard()) {
            Ok((data, _)) => Some(data),
            Err(_) => None,
        },
        Err(_) => None,
    }
}