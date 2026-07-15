use crossbeam_channel::Sender;
use windows::core::w;

use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, MOD_ALT, MOD_NOREPEAT};
use windows::Win32::UI::WindowsAndMessaging::{
    GetMessageW, MessageBoxW, MB_ICONERROR, MB_OK, MSG, WM_HOTKEY,
};

pub fn listen_for_hotkey(tx: Sender<crate::AppMsg>) {
    unsafe {
        let result = RegisterHotKey(None, 1, MOD_ALT | MOD_NOREPEAT, 0x53); // alt + s

        if result.is_ok() {
            let mut msg = MSG::default();
            loop {
                let ret = GetMessageW(&mut msg, None, 0, 0);
                if ret.0 == 0 || ret.0 == -1 {
                    break;
                }
                if msg.message == WM_HOTKEY {
                    let _ = tx.send(crate::AppMsg::ShowLauncher);
                }
            }
        } else {
            MessageBoxW(
                None,
                w!("Não foi possível registar o atalho de teclado! A tecla já está a ser usada por outro programa ou pelo sistema."),
                w!("Erro no Launcher"),
                MB_ICONERROR | MB_OK,
            );
        }
    }
}
