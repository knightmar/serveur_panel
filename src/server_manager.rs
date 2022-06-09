use std::thread;
use std::thread::{JoinHandle, Thread};


pub mod server_manager {
    use std::io::Stdout;
    use std::process::{ChildStdout, Command};
    use std::{thread, time};
    use std::io::Write;

    pub fn start_server() {
        let command = Command::new("sh").args(&["-c", "screen -S mc_server -dm java -jar server.jar nogui"]).current_dir("./server").status().expect("");
        let mut last: Vec<u8> = Vec::new();
        let mut log = String::new();

        loop {
            thread::sleep(time::Duration::from_secs_f64(0.10));
            let out = Command::new("tail").args(&["-n 1", "latest.log"]).current_dir("./server/logs").output().expect("test").stdout;
            if out != last && out.len() > 0 {
                last = out.clone();
                log = String::from_utf8_lossy(&out[..out.len() - 1]).to_string();
                println!("{}", log);
            }
        }
    }
}

