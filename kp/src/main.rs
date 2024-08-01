use std::process::Command;
use std::{env, str};

fn main() {
    let mut args = env::args().skip(1);
    let port = args.next().expect("Please provide a port to kill");

    let output = Command::new("lsof")
        .arg("-i")
        .arg(format!(":{}", port))
        .arg("-nP")
        .output()
        .expect("Failed to execute lsof command");

    if output.status.success() {
        let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 sequence");

        if output_str.trim().is_empty() {
            println!("No process is using port {}", port);
        } else {
            let processes_infos = output_str.lines().skip(1);

            for processes_info in processes_infos {
                if let Some(pid) = processes_info.split_whitespace().collect::<Vec<_>>().get(1) {
                    let status = Command::new("kill")
                        .arg("-9")
                        .arg(pid)
                        .status()
                        .expect("Failed to execute kill command");

                    if status.success() {
                        println!("Process with PID {} terminated", pid);
                    } else {
                        eprintln!("Failed to terminate process with PID {}.", pid);
                    }
                }
            }
        }
    } else {
        let error_message = str::from_utf8(&output.stderr).expect("Invalid UTF-8 sequence");
        eprintln!("Error: {}", error_message);
    }
}
