// Cargo.toml
// [dependencies]
// windows = "0.54"

use std::env;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Threading::{
    OpenProcess, TerminateProcess, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_TERMINATE,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let mut signal = "-TERM";
    let pid_str = if args[1].starts_with('-') {
        if args.len() != 3 {
            print_usage();
            std::process::exit(1);
        }
        signal = &args[1];
        &args[2]
    } else {
        if args.len() != 2 {
            print_usage();
            std::process::exit(1);
        }
        &args[1]
    };

    let pid: u32 = match pid_str.parse() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("PID must be a number");
            std::process::exit(1);
        }
    };

    match signal {
        "-0" => {
            // Check if process exists
            unsafe {
                // Using PROCESS_QUERY_LIMITED_INFORMATION is sufficient and requires fewer privileges.
                match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
                    Ok(handle) => {
                        let _ = CloseHandle(handle);
                        // Exit with 0 to indicate success (process exists)
                        std::process::exit(0);
                    }
                    Err(e) => {
                        eprintln!("Failed to access PID {}: {}", pid, e);
                        // Exit with 1 to indicate failure (process does not exist or access denied)
                        std::process::exit(1);
                    }
                }
            }
        }
        "-TERM" => {
            // Terminate the process
            unsafe {
                let handle = match OpenProcess(PROCESS_TERMINATE, false, pid) {
                    Ok(handle) => handle,
                    Err(e) => {
                        eprintln!("Failed to open PID {}: {}", pid, e);
                        std::process::exit(1);
                    }
                };

                if let Err(e) = TerminateProcess(handle, 1) {
                    eprintln!("Failed to terminate PID {}: {}", pid, e);
                    let _ = CloseHandle(handle);
                    std::process::exit(1);
                } else {
                    println!("Successfully terminated PID {}", pid);
                }

                let _ = CloseHandle(handle);
            }
        }
        _ => {
            eprintln!("Unsupported signal: {}", signal);
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("Usage: kill [-SIGNAL] <PID>");
    eprintln!("Supported signals: -0, -TERM");
}

