use windows::{
    Win32::{UI::WindowsAndMessaging::*, Foundation::HWND},
    core::PCSTR
};
use winapi::{um::processthreadsapi::OpenProcess };
use winapi::vc::vadefs::uintptr_t;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::{PROCESS_ALL_ACCESS};
use winapi::um::memoryapi::ReadProcessMemory;
use std::{ptr};

fn get_application_process() -> (u32, HWND) {
    let class_name = "UnrealWindow";

    let window = unsafe { FindWindowA(PCSTR(class_name.as_ptr()), PCSTR::null())}; 

    let process_id: u32 = 0;
    (process_id, window)
}

fn main() {
    unsafe {
        let ( mut process_id, window) = get_application_process();
        println!("{:?}", window);
        GetWindowThreadProcessId(window, &mut process_id);
        println!("ProcessID: {}", process_id);

        let p_handle: HANDLE = OpenProcess(PROCESS_ALL_ACCESS, 0, process_id);

        if p_handle.is_null() {
            println!("Process not found...");
        }
        
        let address: uintptr_t = 0x2346EE9CE2C;
        let mut buffer = [0u8; 1];

        let result = ReadProcessMemory(p_handle, address as *const _, buffer.as_mut_ptr() as *mut _, buffer.len(), ptr::null_mut());

        println!("buf: {:?}", buffer)
    }
}
