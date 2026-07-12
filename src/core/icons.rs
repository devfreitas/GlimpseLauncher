use bincode::{Decode, Encode};
use eframe::egui::{ColorImage, Context, TextureHandle};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use windows::core::HSTRING;

use windows::Win32::Graphics::Gdi::{
    DeleteObject, GetDC, GetDIBits, GetObjectW, ReleaseDC, BITMAP, BITMAPINFO, BITMAPINFOHEADER,
    BI_RGB, DIB_RGB_COLORS, HBITMAP,
};
use windows::Win32::UI::Shell::ExtractIconExW;
use windows::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON, ICONINFO};

#[derive(Encode, Decode)]
pub struct PersistedIcon {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

#[derive(Clone)]
pub struct IconManager {
    cache: Arc<Mutex<HashMap<String, TextureHandle>>>,
    disk_cache: Arc<Mutex<HashMap<String, PersistedIcon>>>,
}

impl IconManager {
    pub fn new() -> Self {
        let mut manager = Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            disk_cache: Arc::new(Mutex::new(HashMap::new())),
        };
        manager.load_disk_cache();
        manager
    }

    fn cache_path() -> PathBuf {
        let mut base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        base.push("GlimpseLauncher");
        base.push("icons_cache.bin");
        base
    }

    fn load_disk_cache(&mut self) {
        let path = Self::cache_path();
        if path.exists() {
            if let Ok(bytes) = fs::read(&path) {
                if let Ok((data, _)) = bincode::decode_from_slice::<HashMap<String, PersistedIcon>, _>(
                    &bytes,
                    bincode::config::standard(),
                ) {
                    *self.disk_cache.lock().unwrap() = data;
                }
            }
        }
    }

    pub fn save_disk_cache(&self) {
        let path = Self::cache_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let disk = self.disk_cache.lock().unwrap();
        if let Ok(encoded) = bincode::encode_to_vec(&*disk, bincode::config::standard()) {
            let _ = fs::write(path, encoded);
        }
    }

    pub fn get_icon(&self, ctx: &Context, path: &str) -> Option<TextureHandle> {
        let cache = self.cache.lock().unwrap();
        if let Some(handle) = cache.get(path) {
            return Some(handle.clone());
        }
        drop(cache);

        let disk_cache = self.disk_cache.lock().unwrap();
        if let Some(persisted) = disk_cache.get(path) {
            let image = ColorImage::from_rgba_unmultiplied(
                [persisted.width, persisted.height],
                &persisted.pixels,
            );
            let handle = ctx.load_texture(
                format!("icon_{}", path),
                image,
                eframe::egui::TextureOptions::LINEAR,
            );
            self.cache
                .lock()
                .unwrap()
                .insert(path.to_string(), handle.clone());
            return Some(handle);
        }
        drop(disk_cache);

        if let Some(image) = extract_icon_image(path) {
            let persisted = PersistedIcon {
                width: image.size[0],
                height: image.size[1],
                pixels: image
                    .pixels
                    .iter()
                    .flat_map(|c| vec![c.r(), c.g(), c.b(), c.a()])
                    .collect(),
            };
            self.disk_cache
                .lock()
                .unwrap()
                .insert(path.to_string(), persisted);
            self.save_disk_cache();

            let handle = ctx.load_texture(
                format!("icon_{}", path),
                image,
                eframe::egui::TextureOptions::LINEAR,
            );
            self.cache
                .lock()
                .unwrap()
                .insert(path.to_string(), handle.clone());
            return Some(handle);
        }

        None
    }
}

pub static ICON_MANAGER: Lazy<IconManager> = Lazy::new(IconManager::new);

fn extract_icon_image(path: &str) -> Option<ColorImage> {
    if let Some(aumid) = path.strip_prefix("UWP:") {
        use windows::core::ComInterface;
        use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
        use windows::Win32::UI::Shell::{IShellItemImageFactory, SHParseDisplayName};

        unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
            let shell_path = HSTRING::from(format!("shell:appsFolder\\{}", aumid));
            let mut pidl = std::ptr::null_mut();
            if SHParseDisplayName(&shell_path, None, &mut pidl, 0, None).is_ok() {
                use windows::Win32::UI::Shell::SHCreateItemFromIDList;
                if let Ok(item) =
                    SHCreateItemFromIDList::<windows::Win32::UI::Shell::IShellItem>(pidl)
                {
                    if let Ok(factory) = item.cast::<IShellItemImageFactory>() {
                        let size = windows::Win32::Foundation::SIZE { cx: 32, cy: 32 };
                        if let Ok(hbitmap) =
                            factory.GetImage(size, windows::Win32::UI::Shell::SIIGBF_RESIZETOFIT)
                        {
                            let image = hbitmap_to_color_image(hbitmap);
                            use windows::Win32::Graphics::Gdi::DeleteObject;
                            DeleteObject(hbitmap);
                            return image;
                        }
                    }
                }
                use windows::Win32::System::Com::CoTaskMemFree;
                CoTaskMemFree(Some(pidl as _));
            }
        }
        return None;
    }

    let path_hstr = HSTRING::from(path);
    let mut icon_large: [HICON; 1] = [HICON::default(); 1];

    unsafe {
        let extracted = ExtractIconExW(&path_hstr, 0, Some(icon_large.as_mut_ptr()), None, 1);

        if extracted > 0 && !icon_large[0].is_invalid() {
            let hicon = icon_large[0];
            let image = hicon_to_color_image(hicon);
            let _ = DestroyIcon(hicon);
            return image;
        }
    }

    None
}

unsafe fn hicon_to_color_image(hicon: HICON) -> Option<ColorImage> {
    let mut info: ICONINFO = std::mem::zeroed();
    if GetIconInfo(hicon, &mut info).is_err() {
        return None;
    }

    let result = hbitmap_to_color_image(info.hbmColor);

    if !info.hbmColor.is_invalid() {
        DeleteObject(info.hbmColor);
    }
    if !info.hbmMask.is_invalid() {
        DeleteObject(info.hbmMask);
    }

    result
}

unsafe fn hbitmap_to_color_image(hbm: HBITMAP) -> Option<ColorImage> {
    if hbm.is_invalid() {
        return None;
    }

    let hdc = GetDC(None);

    let mut bmp: BITMAP = std::mem::zeroed();
    if GetObjectW(
        hbm,
        std::mem::size_of::<BITMAP>() as i32,
        Some(&mut bmp as *mut _ as *mut _),
    ) == 0
    {
        ReleaseDC(None, hdc);
        return None;
    }

    let width = bmp.bmWidth;
    let height = bmp.bmHeight;
    let mut pixels: Vec<u32> = vec![0; (width * height) as usize];

    let mut bmi: BITMAPINFO = std::mem::zeroed();
    bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    bmi.bmiHeader.biWidth = width;
    bmi.bmiHeader.biHeight = -height;
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;
    bmi.bmiHeader.biCompression = BI_RGB.0;

    let success = GetDIBits(
        hdc,
        hbm,
        0,
        height as u32,
        Some(pixels.as_mut_ptr() as *mut _),
        &mut bmi,
        DIB_RGB_COLORS,
    );

    ReleaseDC(None, hdc);

    if success == 0 {
        return None;
    }

    let mut rgba_pixels = Vec::with_capacity((width * height * 4) as usize);
    for &p in &pixels {
        let a = ((p >> 24) & 0xFF) as u8;
        let r = ((p >> 16) & 0xFF) as u8;
        let g = ((p >> 8) & 0xFF) as u8;
        let b = (p & 0xFF) as u8;
        rgba_pixels.push(r);
        rgba_pixels.push(g);
        rgba_pixels.push(b);
        rgba_pixels.push(a);
    }

    Some(ColorImage::from_rgba_unmultiplied(
        [width as usize, height as usize],
        &rgba_pixels,
    ))
}