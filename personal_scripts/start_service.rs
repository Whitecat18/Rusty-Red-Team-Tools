use std::ptr::null_mut;
use winapi::um::winsvc::{
    OpenSCManagerW, OpenServiceW, StartServiceW, ChangeServiceConfigW,
    QueryServiceStatusEx, SetServiceStatus, SERVICE_STATUS_PROCESS,
    SERVICE_NO_CHANGE, SERVICE_AUTO_START, SERVICE_QUERY_CONFIG,
    SERVICE_CHANGE_CONFIG, SERVICE_START, SERVICE_QUERY_STATUS,
    SERVICE_STOP, SERVICE_ALL_ACCESS, SC_MANAGER_ALL_ACCESS,
};
use winapi::shared::winerror::ERROR_SERVICE_ALREADY_RUNNING;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::CloseServiceHandle;
use winapi::um::synchapi::Sleep;

fn main() {
    let service_name = "AarSvc";
    let scm_handle = unsafe {
        OpenSCManagerW(
            null_mut(),
            null_mut(),
            SC_MANAGER_ALL_ACCESS,
        )
    };

    if scm_handle.is_null() {
        eprintln!("Failed to open Service Control Manager: {}", unsafe { GetLastError() });
        return;
    }

    let service_handle = unsafe {
        OpenServiceW(
            scm_handle,
            to_wide_string(service_name).as_ptr(),
            SERVICE_ALL_ACCESS,
        )
    };

    if service_handle.is_null() {
        eprintln!(
            "Failed to open service '{}': {}",
            service_name,
            unsafe { GetLastError() }
        );
        unsafe { CloseServiceHandle(scm_handle) };
        return;
    }

    let success = unsafe {
        ChangeServiceConfigW(
            service_handle,
            SERVICE_NO_CHANGE,
            SERVICE_AUTO_START,
            SERVICE_NO_CHANGE,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
        )
    };

    if success == 0 {
        eprintln!(
            "Failed to set '{}' as auto-start: {}",
            service_name,
            unsafe { GetLastError() }
        );
    } else {
        println!("Service '{}' is now set to auto-start.", service_name);
    }

    // Start the service if not already running
    let started = unsafe { StartServiceW(service_handle, 0, null_mut()) };

    if started == 0 {
        let error = unsafe { GetLastError() };
        if error == ERROR_SERVICE_ALREADY_RUNNING {
            println!("Service '{}' is already running.", service_name);
        } else {
            eprintln!("Failed to start service '{}': {}", service_name, error);
        }
    } else {
        println!("Service '{}' has been started successfully.", service_name);
    }

    // Prevent the service from being stopped (unless stopped manually)
    let mut service_status: SERVICE_STATUS_PROCESS = unsafe { std::mem::zeroed() };
    let status_size = std::mem::size_of::<SERVICE_STATUS_PROCESS>() as u32;

    let status_ok = unsafe {
        QueryServiceStatusEx(
            service_handle,
            0, // SC_STATUS_PROCESS_INFO
            &mut service_status as *mut _ as *mut u8,
            status_size,
            &mut 0,
        )
    };

    if status_ok != 0 {
        println!(
            "Service '{}' is running with process ID: {}",
            service_name, service_status.dwProcessId
        );

        loop {
            unsafe { Sleep(5000); }
            let running_ok = unsafe {
                QueryServiceStatusEx(
                    service_handle,
                    0,
                    &mut service_status as *mut _ as *mut u8,
                    status_size,
                    &mut 0,
                )
            };

            if running_ok == 0 || service_status.dwCurrentState != winapi::um::winsvc::SERVICE_RUNNING {
                println!("Service '{}' is not running. Attempting to restart...", service_name);
                unsafe { StartServiceW(service_handle, 0, null_mut()) };
            }
        }
    } else {
        eprintln!("Failed to query service status: {}", unsafe { GetLastError() });
    }

    unsafe {
        CloseServiceHandle(service_handle);
        CloseServiceHandle(scm_handle);
    }
}

fn to_wide_string(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(value).encode_wide().chain(Some(0)).collect()
}
