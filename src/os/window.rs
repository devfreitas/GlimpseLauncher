use std::path::Path;
use windows::core::{ComInterface, HSTRING, PWSTR};
use windows::Win32::Foundation::{CloseHandle, BOOL, HWND, LPARAM, MAX_PATH};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, IPersistFile, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED,
    STGM,
};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::Shell::{IShellLinkW, ShellLink};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowThreadProcessId, IsWindowVisible, SetForegroundWindow, ShowWindow,
    SW_RESTORE,
};

struct EnumState {
    target_path: String,
    found: bool,
}

fn resolve_lnk(path: &str) -> String {
    if !path.to_lowercase().ends_with(".lnk") {
        return path.to_string();
    }
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        if let Ok(shell_link) =
            CoCreateInstance::<_, IShellLinkW>(&ShellLink, None, CLSCTX_INPROC_SERVER)
        {
            if let Ok(persist_file) = shell_link.cast::<IPersistFile>() {
                let path_hstring = HSTRING::from(path);
                if persist_file.Load(&path_hstring, STGM(0)).is_ok() {
                    let mut buffer: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
                    if shell_link
                        .GetPath(&mut buffer, std::ptr::null_mut(), 0)
                        .is_ok()
                    {
                        let len = buffer
                            .iter()
                            .position(|&c| c == 0)
                            .unwrap_or(MAX_PATH as usize);
                        if len > 0 {
                            return String::from_utf16_lossy(&buffer[..len]);
                        }
                    }
                }
            }
        }
    }
    path.to_string()
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let state = &mut *(lparam.0 as *mut EnumState);

    if !IsWindowVisible(hwnd).as_bool() {
        return BOOL(1);
    }

    let mut process_id: u32 = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut process_id));

    if process_id == 0 {
        return BOOL(1);
    }

    if let Ok(process_handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) {
        let mut buffer: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
        let mut size = MAX_PATH;

        let success = QueryFullProcessImageNameW(
            process_handle,
            PROCESS_NAME_WIN32,
            PWSTR(buffer.as_mut_ptr()),
            &mut size,
        );

        CloseHandle(process_handle).ok();

        if success.is_ok() && size > 0 {
            let exe_path = String::from_utf16_lossy(&buffer[..size as usize]);

            // Compare full paths
            if exe_path.eq_ignore_ascii_case(&state.target_path) {
                ShowWindow(hwnd, SW_RESTORE);
                SetForegroundWindow(hwnd);
                state.found = true;
                return BOOL(0);
            }

            // Fallback: If target path has no path (just exe name) or we want to match by stem
            let exe_stem = Path::new(&exe_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            let target_stem = Path::new(&state.target_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");

            if !exe_stem.is_empty() && exe_stem.eq_ignore_ascii_case(target_stem) {
                ShowWindow(hwnd, SW_RESTORE);
                SetForegroundWindow(hwnd);
                state.found = true;
                return BOOL(0);
            }
        }
    }

    BOOL(1)
}

pub fn focus_window_if_running(target_path: &str) -> bool {
    let resolved_path = resolve_lnk(target_path);
    let mut state = EnumState {
        target_path: resolved_path,
        found: false,
    };

    unsafe {
        let lparam = LPARAM(&mut state as *mut EnumState as isize);
        let _ = EnumWindows(Some(enum_windows_proc), lparam);
    }

    state.found
}
