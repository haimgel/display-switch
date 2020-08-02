//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{HBRUSH, HCURSOR, HICON, HWND};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, GetWindowLongPtrW,
    PostQuitMessage, RegisterClassW, SetWindowLongPtrW, TranslateMessage, GWLP_USERDATA, MSG,
    WM_CREATE, WM_DESTROY, WM_DEVICECHANGE, WNDCLASSW,
};

pub struct PnPDetect<F>
where
    F: FnMut(),
{
    hwnd: HWND,
    callback: F,
}

impl<F> PnPDetect<F>
where
    F: FnMut(),
{
    pub fn new(callback: F) -> Self {
        let mut pnp_detect = PnPDetect {
            hwnd: std::ptr::null_mut(),
            callback,
        };
        pnp_detect.create_window();
        return pnp_detect;
    }

    pub fn detect(&self) {
        unsafe {
            let mut msg: MSG = std::mem::MaybeUninit::zeroed().assume_init();
            loop {
                let val = GetMessageW(&mut msg, self.hwnd, 0, 0);
                if val == 0 {
                    break;
                } else {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
    }

    // Window procedure function to handle events
    pub unsafe extern "system" fn window_proc(
        hwnd: HWND,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_CREATE => {
                let create_struct = lparam as *mut winapi::um::winuser::CREATESTRUCTW;
                let window_state_ptr = create_struct.as_ref().unwrap().lpCreateParams;
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, window_state_ptr as isize);
            }
            WM_DESTROY => {
                PostQuitMessage(0);
            }
            WM_DEVICECHANGE => {
                let self_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Self;
                let window_state: &mut Self = self_ptr.as_mut().unwrap();
                (window_state.callback)();
            }
            _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
        }
        return 0;
    }

    fn create_window(&mut self) {
        unsafe {
            let winapi_class_name: Vec<u16> = OsStr::new("Sample Window Class 22")
                .encode_wide()
                .chain(once(0))
                .collect();
            let hinstance = GetModuleHandleW(std::ptr::null());

            let wc = WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(Self::window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: hinstance,
                hIcon: 0 as HICON,
                hCursor: 0 as HCURSOR,
                hbrBackground: 0 as HBRUSH,
                lpszMenuName: 0 as LPCWSTR,
                lpszClassName: winapi_class_name.as_ptr(),
            };

            let error_code = RegisterClassW(&wc);
            assert_ne!(error_code, 0, "failed to register the window class");

            let window_name: Vec<u16> = OsStr::new("Rust Win32 window 22")
                .encode_wide()
                .chain(once(0))
                .collect();

            let hwnd = CreateWindowExW(
                0,
                winapi_class_name.as_ptr(),
                window_name.as_ptr(),
                0,
                0,
                0,
                0,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                hinstance,
                self as *mut Self as *mut winapi::ctypes::c_void,
                //std::ptr::null_mut(),
            );

            if hwnd.is_null() {
                panic!("Something went wrong while creating a window");
            }
            self.hwnd = hwnd;
        }
    }
}
