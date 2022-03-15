use std::collections::HashSet;
use windows::core::PSTR;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

static mut IDS: Option<HashSet<u32>> = None;

unsafe extern "system" fn count_windows(handle: HWND, _: LPARAM) -> BOOL {
    let styles = WINDOW_EX_STYLE(GetWindowLongPtrA(handle, GWL_EXSTYLE) as u32);
    if IsWindowVisible(handle).as_bool()
        && ((styles & WS_EX_APPWINDOW) == WS_EX_APPWINDOW
            || (styles & WS_EX_TOOLWINDOW) == WINDOW_EX_STYLE(0))
    {
        let mut buffer = [0; 50];
        GetWindowTextA(handle, PSTR(&mut buffer as *mut u8), 50);
        let mut end = 50;
        for (i, byte) in buffer.iter().enumerate() {
            if *byte == 0 {
                end = i;
                break;
            }
        }
        let title = String::from_utf8_lossy(&buffer[..end]);
        if !(title == "" || title == "Microsoft Text Input Application") {
            println!("{}", title);
            if let Some(ids) = &mut IDS {
                ids.insert(GetWindowThreadProcessId(handle, 0 as *mut u32));
            }
        }
    }
    BOOL(1)
}

fn main() {
    unsafe {
        let handle = windows::Win32::System::Console::GetConsoleWindow();
        loop {
            IDS = Some(HashSet::new());
            EnumWindows(Some(count_windows), LPARAM(0));
            println!();
            let count = IDS.iter().next().unwrap().len();
            println!("{:?}", count);
            if count % 2 == 1 {
                let styles = WINDOW_EX_STYLE(GetWindowLongPtrA(handle, GWL_EXSTYLE) as u32);
                if (styles & WS_EX_TOOLWINDOW) == WS_EX_TOOLWINDOW {
                    SetWindowLongPtrA(handle, GWL_EXSTYLE, 0);
                } else {
                    SetWindowLongPtrA(handle, GWL_EXSTYLE, WS_EX_TOOLWINDOW.0 as isize);
                }
            }
        }
    }
}
