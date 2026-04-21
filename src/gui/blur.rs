use eframe::CreationContext;

#[cfg(windows)]
pub fn enable_blur_for_window(cc: &CreationContext<'_>) {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Graphics::Dwm::{
        DwmSetWindowAttribute, DWMSBT_MAINWINDOW, DWMWA_SYSTEMBACKDROP_TYPE,
        DWMWA_USE_IMMERSIVE_DARK_MODE, DWM_SYSTEMBACKDROP_TYPE,
    };

    let handle = match cc.window_handle() {
        Ok(h) => h,
        Err(_) => {
            log::warn!("No window handle available; skipping DWM blur setup");
            return;
        }
    };
    let raw: RawWindowHandle = handle.as_raw();
    let hwnd = match raw {
        RawWindowHandle::Win32(h) => HWND(h.hwnd.get() as *mut _),
        _ => {
            log::warn!("Unexpected window handle variant; skipping DWM blur");
            return;
        }
    };

    unsafe {
        let dark: i32 = 1;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE,
            &dark as *const _ as *const _,
            std::mem::size_of::<i32>() as u32,
        );

        let backdrop: DWM_SYSTEMBACKDROP_TYPE = DWMSBT_MAINWINDOW;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_SYSTEMBACKDROP_TYPE,
            &backdrop as *const _ as *const _,
            std::mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32,
        );
    }

    enable_accent_blur(hwnd);
    log::info!("DWM blur / Mica backdrop requested");
}

#[cfg(windows)]
fn enable_accent_blur(hwnd: windows::Win32::Foundation::HWND) {
    use std::ffi::c_void;
    use windows::core::s;
    use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};

    #[repr(C)]
    struct AccentPolicy {
        accent_state: u32,
        accent_flags: u32,
        gradient_color: u32,
        animation_id: u32,
    }

    #[repr(C)]
    struct WindowCompositionAttribData {
        attribute: u32,
        data: *mut c_void,
        data_size: usize,
    }

    const ACCENT_ENABLE_ACRYLICBLURBEHIND: u32 = 4;
    const WCA_ACCENT_POLICY: u32 = 19;

    type PfnSetWindowCompositionAttribute = unsafe extern "system" fn(
        hwnd: windows::Win32::Foundation::HWND,
        data: *mut WindowCompositionAttribData,
    ) -> i32;

    unsafe {
        let user32 = match GetModuleHandleA(s!("user32.dll")) {
            Ok(m) => m,
            Err(_) => return,
        };
        let proc = match GetProcAddress(user32, s!("SetWindowCompositionAttribute")) {
            Some(p) => p,
            None => return,
        };
        let set_comp: PfnSetWindowCompositionAttribute = std::mem::transmute(proc);

        let mut accent = AccentPolicy {
            accent_state: ACCENT_ENABLE_ACRYLICBLURBEHIND,
            accent_flags: 0,
            gradient_color: 0x20_0A_0E_1C,
            animation_id: 0,
        };
        let mut data = WindowCompositionAttribData {
            attribute: WCA_ACCENT_POLICY,
            data: &mut accent as *mut _ as *mut c_void,
            data_size: std::mem::size_of::<AccentPolicy>(),
        };
        set_comp(hwnd, &mut data);
    }
}

#[cfg(not(windows))]
pub fn enable_blur_for_window(_cc: &CreationContext<'_>) {}
