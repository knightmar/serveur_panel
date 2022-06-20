use std::thread;
use std::thread::{JoinHandle, Thread};


pub mod server_manager {
    use std::io::{BufWriter, Stdout};
    use std::process::{ChildStdout, Command, exit};
    use std::{thread, time};
    use std::fs::copy;
    use std::io::Write;
    use std::net::TcpStream;

    pub fn start_server(flux: &TcpStream) {
        let mut writer = BufWriter::new(flux);
        let mut line = String::from("Ceci est un test\n\n");
        // writer.write_all(line.as_ref()).expect("");

        let command = Command::new("sh").args(&["-c", "screen -S mc_server -dm java -jar server.jar nogui"]).current_dir("./server").status().expect("");
        let mut last = String::new();
        let mut log = String::new();

        loop {
            thread::sleep(time::Duration::from_secs_f64(0.10));
            let mut out = String::from_utf8(Command::new("tail").args(&["-n 1", "latest.log"]).current_dir("./server/logs").output().expect("test").stdout).unwrap();
            out = out.trim().parse().unwrap();
            if out != last {
                last = out;
                last.push_str("\n\n");
                writer.write_all(last.as_ref()).expect("TODO: panic message");
                let flush = writer.flush();
                if !flush.is_ok() {
                    eprintln!("Connexion ended");
                    exit(0);
                }
            }
        }


        println!("new connexion from {}", flux.peer_addr().unwrap());

        let mut new_line = String::from("patate\n");

        let mut writer = BufWriter::new(flux);

        println!("line is : {}", new_line);

        writer.write_all(new_line.as_ref()).expect("TODO: panic message");
    }
}

