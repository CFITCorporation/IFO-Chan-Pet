use tauri::Manager;
use tauri::WindowEvent;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HWND;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongW, SetWindowLongW, SetWindowLongPtrW, CallWindowProcW,
    GWL_STYLE, GWLP_WNDPROC, WS_POPUP, WS_OVERLAPPEDWINDOW, WNDPROC,
};
use tauri_plugin_polygon::PolygonExt;
#[cfg(target_os = "windows")]
use std::ffi::c_void;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_polygon::init(|_app, event| {
            match event {
                tauri_plugin_polygon::Event::LeftClick { x, y } => {
                    println!("Clicked outside polygon at ({}, {})", x, y);
                }
                _ => {}
            }
        }))
        .setup(|app| {
            let windows = app.webview_windows();
            let window = windows.values().next().unwrap().clone();

            window.set_ignore_cursor_events(true).unwrap();
            // 修复win11💩山逻辑
            #[cfg(target_os = "windows")]
            {
                const WM_NCCALCSIZE: u32 = 0x0083;
                const WM_NCACTIVATE: u32 = 0x0086;

                // 保存原始窗口过程，用于处理其他消息
                static mut ORIG_WND_PROC: Option<WNDPROC> = None;

                unsafe extern "system" fn sub_wnd_proc(
                    hwnd: HWND,
                    msg: u32,
                    wparam: windows::Win32::Foundation::WPARAM,
                    lparam: windows::Win32::Foundation::LPARAM,
                ) -> windows::Win32::Foundation::LRESULT {
                    match msg {
                        WM_NCCALCSIZE => windows::Win32::Foundation::LRESULT(0),
                        WM_NCACTIVATE => windows::Win32::Foundation::LRESULT(1),
                        _ => {
                            if let Some(orig_proc) = ORIG_WND_PROC {
                                return CallWindowProcW(orig_proc, hwnd, msg, wparam, lparam);
                            }
                            windows::Win32::Foundation::LRESULT(0)
                        }
                    }
                }

                let hwnd = window.hwnd().unwrap() as HWND;
                unsafe {
                    // 无边框弹出式
                    let style = GetWindowLongW(hwnd, GWL_STYLE);
                    let new_style = (style as u32 & !(WS_OVERLAPPEDWINDOW.0 as u32)) | (WS_POPUP.0 as u32);
                    SetWindowLongW(hwnd, GWL_STYLE, new_style as i32);

                    // 替换窗口过程
                    let new_proc = sub_wnd_proc as *mut c_void;
                    let original = SetWindowLongPtrW(hwnd, GWLP_WNDPROC, new_proc as isize);
                    ORIG_WND_PROC = Some(std::mem::transmute(original));
                }
            }

            app.polygon().register("ball-area").unwrap();

            let monitor = window.current_monitor().unwrap().unwrap();
            let screen_size = monitor.size();
            let window_size = window.outer_size().unwrap();
            let x = screen_size.width as i32 - window_size.width as i32;
            let y = screen_size.height as i32 - window_size.height as i32;
            let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));
            //deprecated below
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::Moved(pos) = event {
                    if let Ok(Some(monitor)) = window_clone.current_monitor() {
                        let screen_rect = monitor.size();
                        let window_size = match window_clone.outer_size() {
                            Ok(size) => size,
                            Err(_) => return,
                        };
                        let mut new_x = pos.x;
                        let mut new_y = pos.y;
                        let mut changed = false;

                        if new_x < 0 {
                            new_x = 0;
                            changed = true;
                        }
                        if new_x + (window_size.width as i32) > screen_rect.width as i32 {
                            new_x = screen_rect.width as i32 - window_size.width as i32;
                            changed = true;
                        }
                        if new_y < 0 {
                            new_y = 0;
                            changed = true;
                        }
                        if new_y + (window_size.height as i32) > screen_rect.height as i32 {
                            new_y = screen_rect.height as i32 - window_size.height as i32;
                            changed = true;
                        }

                        if changed {
                            let _ = window_clone.set_position(tauri::Position::Physical(
                                tauri::PhysicalPosition { x: new_x, y: new_y },
                            ));
                        }
                    }
                }
            });


            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}