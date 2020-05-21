use winapi::{shared::windef::HWND, um::winuser::{GetActiveWindow, GetDC}};

pub fn find_device_context() -> Option<bootstrap_gl::DeviceContext> {
    let hwnd = unsafe { GetActiveWindow() };

    if hwnd.is_null() {
        return None;
    }

    device_context_from_window_handle(hwnd)
}

pub fn device_context_from_window_handle(window_handle: HWND) -> Option<bootstrap_gl::DeviceContext> {
    let device_context = unsafe { GetDC(window_handle) };
    if device_context.is_null() {
        None
    } else {
        Some(device_context)
    }
}
