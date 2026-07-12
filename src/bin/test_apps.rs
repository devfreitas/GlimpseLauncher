use windows::core::ComInterface;
use windows::Win32::System::Com::{CoInitializeEx, CoTaskMemFree, COINIT_MULTITHREADED};
use windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY;
use windows::Win32::UI::Shell::{
    BHID_EnumItems, FOLDERID_AppsFolder, IEnumShellItems, IShellItem, IShellItem2,
    SHGetKnownFolderItem, SIGDN_NORMALDISPLAY,
};

const PKEY_APP_USER_MODEL_ID: PROPERTYKEY = PROPERTYKEY {
    fmtid: windows::core::GUID::from_u128(0x9F4C2855_9F79_4B39_A8D0_E1D42DE1D5F3),
    pid: 5,
};

fn main() {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let apps_folder: IShellItem = match SHGetKnownFolderItem(
            &FOLDERID_AppsFolder,
            windows::Win32::UI::Shell::KF_FLAG_DEFAULT,
            None,
        ) {
            Ok(f) => f,
            Err(e) => {
                println!("Error getting apps folder: {:?}", e);
                return;
            }
        };

        let enum_items: IEnumShellItems = apps_folder.BindToHandler(None, &BHID_EnumItems).unwrap();

        let mut fetched = 0;
        let mut items: [Option<IShellItem>; 1] = [None; 1];
        while enum_items.Next(&mut items, Some(&mut fetched)).is_ok() && fetched == 1 {
            if let Some(item) = &items[0] {
                if let Ok(name_pwstr) = item.GetDisplayName(SIGDN_NORMALDISPLAY) {
                    let name = name_pwstr.to_string().unwrap_or_default();
                    if let Ok(item2) = item.cast::<IShellItem2>() {
                        if let Ok(aumid_pwstr) = item2.GetString(&PKEY_APP_USER_MODEL_ID) {
                            let aumid = aumid_pwstr.to_string().unwrap_or_default();
                            println!("Name: {}, AUMID: {}", name, aumid);
                            CoTaskMemFree(Some(aumid_pwstr.0 as _));
                        }
                    }
                    CoTaskMemFree(Some(name_pwstr.0 as _));
                }
            }
        }
    }
}