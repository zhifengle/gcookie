use windows::{
    core::{HSTRING, PCWSTR, PWSTR},
    Win32::{
        Foundation::{ERROR_MORE_DATA, ERROR_SUCCESS},
        System::RestartManager::{
            RmEndSession, RmForceShutdown, RmGetList, RmRegisterResources, RmShutdown,
            RmStartSession, CCH_RM_SESSION_KEY, RM_PROCESS_INFO,
        },
    },
};

// https://github.com/thewh1teagle/rookie/blob/main/rookie-rs/src/windows/restart_manager.rs
pub unsafe fn release_file_lock(file_path: &str) -> bool {
    let file_path = HSTRING::from(file_path);
    let mut session: u32 = 0;
    let mut session_key_buffer = [0_u16; (CCH_RM_SESSION_KEY as usize) + 1];
    let session_key = PWSTR(session_key_buffer.as_mut_ptr());
    let result = RmStartSession(&mut session, 0, session_key);
    if result == ERROR_SUCCESS {
        let result = RmRegisterResources(session, Some(&[PCWSTR(file_path.as_ptr())]), None, None);
        if result == ERROR_SUCCESS {
            let mut pnprocinfoneeded: u32 = 0;
            let mut rgaffectedapps: [RM_PROCESS_INFO; 1] = [RM_PROCESS_INFO {
                ..Default::default()
            }];
            let mut lpdwrebootreasons: u32 = 0;
            let mut pnprocinfo: u32 = 0;
            let result = RmGetList(
                session,
                &mut pnprocinfoneeded,
                &mut pnprocinfo,
                Some(rgaffectedapps.as_mut_ptr()),
                &mut lpdwrebootreasons,
            );
            if result == ERROR_SUCCESS || result == ERROR_MORE_DATA {
                if pnprocinfoneeded > 0 {
                    // If current process does not have enough privileges to close one of
                    // the "offending" processes, you'll get ERROR_FAIL_NOACTION_REBOOT
                    let result = RmShutdown(session, RmForceShutdown.0 as u32, None);
                    if result == ERROR_SUCCESS {
                        // success
                        let _ = RmEndSession(session);
                        return true;
                    }
                } else {
                    // success
                    let _ = RmEndSession(session);
                    return true;
                }
            }
        }
        let _ = RmEndSession(session);
        return false;
    }
    return false;
}
