use std::{mem, ptr};
use winapi::{
    shared::minwindef::TRUE,
    um::{errhandlingapi::GetLastError,
        libloaderapi::{GetProcAddress, LoadLibraryA},
        wingdi::{SwapBuffers, wglCreateContext, wglGetProcAddress, wglMakeCurrent, wglGetCurrentDC, wglDeleteContext, wglGetCurrentContext},
        winuser::GetActiveWindow}
};

pub type DeviceContext = winapi::shared::windef::HDC;
pub type Context = (winapi::shared::windef::HDC, winapi::shared::windef::HGLRC);

pub unsafe fn create_context(device_context: DeviceContext) -> Option<Context> {
    let tmp_context = wglCreateContext(device_context);
    if tmp_context.is_null() {
        return None;
    }

    make_current((device_context, tmp_context));

    let render_context = create_context_attribs(device_context, ptr::null_mut(), ptr::null());

    clear_current();
    wglDeleteContext(tmp_context);

    if render_context.is_null() {
        let error = GetLastError();
        println!("WARNING: Failed to created OpenGL context, last error: {:#x}", error);
        None
    } else {
        make_current((device_context, render_context));

        // TODO: Don't do this in context creation.
        if set_swap_interval(0) != crate::types::Boolean::True {
            println!("WARNING: Failed to set swap interval of setting swap interval");
        }

        clear_current();

        Some((device_context, render_context))
    }
}

pub unsafe fn destroy_context(context: Context) {
    let (_, render_context) = context;
    clear_current();

    let result = wglDeleteContext(render_context);

    assert!(result == 1, "Failed to delete context: {:?}", render_context);
}

pub unsafe fn load_proc(proc_name: &str) -> Option<extern "system" fn()> {
    let string = proc_name.as_bytes();
    debug_assert!(
        string[string.len() - 1] == 0,
        "Proc name \"{}\" is not null terminated",
        proc_name,
    );

    let mut ptr = wglGetProcAddress(string.as_ptr() as *const _);

    if ptr.is_null() {
        let module = LoadLibraryA(b"opengl32.dll\0".as_ptr() as *const _);

        // TODO: What do we want to do in this case? Probably just return `None`, right?
        assert!(!module.is_null(), "Failed to load opengl32.dll");

        ptr = GetProcAddress(module, string.as_ptr() as *const _);
    }

    if ptr.is_null() {
        let actual_dc = wglGetCurrentDC();
        let actual_context = wglGetCurrentContext();
        println!(
            "pointer for {} was null, last error: 0x{:X}, active dc: {:?}, active context: {:?}",
            proc_name,
            GetLastError(),
            actual_dc,
            actual_context,
        );

        return None;
    }

    Some(mem::transmute(ptr))
}

pub unsafe fn swap_buffers(context: Context) {
    let (device_context, _) = context;
    if SwapBuffers(device_context) != TRUE {
        let (device_context, render_context) = context;
        let hwnd = GetActiveWindow();
        panic!(
            "Swap buffers failed, dc: {:?}, context: {:?} last error: 0x:{:X}, hwnd: {:?}",
            device_context,
            render_context,
            GetLastError(),
            hwnd,
        );
    }
}

pub unsafe fn make_current(context: Context) -> Context {
    let old_device_context = wglGetCurrentDC();
    let old_render_context = wglGetCurrentContext();

    let (device_context, render_context) = context;
    let result = wglMakeCurrent(device_context, render_context);
    if result != TRUE {
        let hwnd = GetActiveWindow();
        panic!(
            "Failed to make context current, dc: {:?}, context: {:?} last error: 0x:{:X}, actual dc and context: {:?} and {:?}, hwnd: {:?}",
            device_context,
            render_context,
            GetLastError(),
            old_device_context,
            old_render_context,
            hwnd,
        );
    }

    (old_device_context, old_render_context)
}

pub unsafe fn clear_current() {
    make_current((ptr::null_mut(), ptr::null_mut()));
}

gl_proc!(wglGetExtensionsStringARB:
    fn get_extension_string(hdc: winapi::shared::windef::HDC) -> *const u8);

gl_proc!(wglCreateContextAttribsARB:
    fn create_context_attribs(
        hdc: winapi::shared::windef::HDC,
        share_context: winapi::shared::windef::HGLRC,
        attrib_list: *const i32
    ) -> winapi::shared::windef::HGLRC);

gl_proc!(wglGetSwapIntervalEXT:
    fn get_swap_interval() -> i32);

gl_proc!(wglSwapIntervalEXT:
    fn set_swap_interval(interval: i32) -> crate::types::Boolean);
