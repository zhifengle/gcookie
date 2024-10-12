use std::ffi::c_void;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

pub fn is_elevated() -> bool {
    let mut result = false;
    let mut handle: HANDLE = HANDLE(std::ptr::null_mut());
    unsafe {
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle).is_ok() {
            let elevation = TOKEN_ELEVATION::default();
            let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
            let mut ret_size = size;
            let raw_ptr = &elevation as *const _ as *mut c_void;
            if GetTokenInformation(
                handle,
                TokenElevation.into(),
                Some(raw_ptr),
                size,
                &mut ret_size,
            )
            .is_ok()
            {
                result = elevation.TokenIsElevated != 0;
            }
        }
        let _ = CloseHandle(handle);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_elevated_ok() {
        // run with admin
        println!("---- run with admin ----- {}", is_elevated());
        // assert!(is_elevated());
    }
}
