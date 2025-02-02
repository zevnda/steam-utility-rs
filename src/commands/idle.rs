use crate::commands::init_steam_client;
use image::GenericImageView;
use image::{load_from_memory, DynamicImage};
use reqwest::blocking::get;
use serde_json::Value;
use std::ffi::OsStr;
use std::io::Read;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HDC, HICON, HWND};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, SelectObject,
    SRCCOPY,
};
use winapi::um::winuser::{
    AdjustWindowRect, BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint,
    GetMessageW, LoadImageW, PostQuitMessage, RegisterClassW, SendMessageW, ShowWindow,
    TranslateMessage, UpdateWindow, CW_USEDEFAULT, IMAGE_ICON, LR_LOADFROMFILE, MSG, PAINTSTRUCT,
    SW_SHOW, WM_CLOSE, WM_DESTROY, WM_PAINT, WM_SETICON, WNDCLASSW, WS_CAPTION, WS_MINIMIZEBOX,
    WS_OVERLAPPED, WS_SYSMENU,
};

static mut IMAGE: Option<DynamicImage> = None;

// Handle window messages
extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CLOSE => {
            unsafe { PostQuitMessage(0) };
            0
        }
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            0
        }
        WM_PAINT => {
            unsafe {
                let mut ps: PAINTSTRUCT = std::mem::zeroed();
                let hdc: HDC = BeginPaint(hwnd, &mut ps);
                if let Some(ref img) = IMAGE {
                    let (width, height) = img.dimensions();
                    let hdc_mem = CreateCompatibleDC(hdc);
                    let bitmap = CreateCompatibleBitmap(hdc, width as i32, height as i32);
                    SelectObject(hdc_mem, bitmap as *mut _);
                    let raw_img = img.to_rgba8();
                    let data = raw_img.as_raw();
                    for y in 0..height {
                        for x in 0..width {
                            let idx = (y * width + x) as usize * 4;
                            let color =
                                winapi::um::wingdi::RGB(data[idx], data[idx + 1], data[idx + 2]);
                            winapi::um::wingdi::SetPixel(hdc_mem, x as i32, y as i32, color);
                        }
                    }
                    BitBlt(
                        hdc,
                        0,
                        0,
                        width as i32,
                        height as i32,
                        hdc_mem,
                        0,
                        0,
                        SRCCOPY,
                    );
                    DeleteObject(bitmap as *mut _);
                    DeleteDC(hdc_mem);
                }
                EndPaint(hwnd, &ps);
            }
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

// Idle the game and display its header image in the window
pub fn idle(app_id: u32) {
    init_steam_client(app_id).expect("Failed to initialize Steam client");

    // Download the game's header image
    let url = format!(
        "https://cdn.akamai.steamstatic.com/steam/apps/{}/header_292x136.jpg",
        app_id
    );
    let mut response = get(&url).expect("Failed to download image");
    let mut buf = Vec::new();
    response
        .read_to_end(&mut buf)
        .expect("Failed to read image");
    let img = load_from_memory(&buf);

    // Get the game name
    let app_name = get_app_name(app_id).unwrap_or_else(|_| "Idling".to_string());
    let app_name_wide: Vec<u16> = app_name.encode_utf16().chain(Some(0)).collect();

    unsafe {
        if let Ok(image) = img {
            IMAGE = Some(image);
        } else {
            IMAGE = None;
        }

        let h_instance: HINSTANCE = GetModuleHandleW(null_mut());
        let class_name = "my_window_class\0".encode_utf16().collect::<Vec<u16>>();

        let icon_path = "res/icon.ico";
        let icon_path_wide: Vec<u16> = OsStr::new(icon_path).encode_wide().chain(Some(0)).collect();
        let h_icon = LoadImageW(
            null_mut(),
            icon_path_wide.as_ptr(),
            IMAGE_ICON,
            32,
            32,
            LR_LOADFROMFILE,
        ) as HICON;

        let wnd_class = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: h_icon,
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
            lpszClassName: class_name.as_ptr(),
        };

        RegisterClassW(&wnd_class);

        let mut rect = winapi::shared::windef::RECT {
            left: 0,
            top: 0,
            right: 291,
            bottom: 136,
        };
        AdjustWindowRect(
            &mut rect,
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
            0,
        );

        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            app_name_wide.as_ptr(),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            null_mut(),
            null_mut(),
            h_instance,
            null_mut(),
        );

        if hwnd.is_null() {
            eprintln!("Failed to create window");
            return;
        }

        SendMessageW(hwnd, WM_SETICON, 0, h_icon as LPARAM);
        SendMessageW(hwnd, WM_SETICON, 1, h_icon as LPARAM);

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

// Get the game name from the Steam store API
fn get_app_name(app_id: u32) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://store.steampowered.com/api/appdetails?l=english&appids={}",
        app_id
    );
    let response = get(&url)?;
    let response_body = response.text()?;
    let json_data: Value = serde_json::from_str(&response_body)?;
    let app_data = &json_data[app_id.to_string()]["data"]["name"];
    let app_name = app_data.as_str().unwrap_or("Unknown Game").to_string();
    Ok(format!("{} [{}]", app_name, app_id))
}
