use std::thread;
use std::thread::{JoinHandle, Thread};


pub mod server_manager {
    use std::io::Stdout;
    use std::process::ChildStdout;
    use std::thread;
    use tokio::io::AsyncReadExt;
    use tokio::process::Command;
    use std::io::Write;

    #[tokio::main]
    pub async fn start_server() {

        let server_thread = thread::spawn(move || {
            let mut child = Command::new("cmd")
                .current_dir("server")
                .args(&["/C", "java -jar server.jar --nogui"])
                .spawn()
                .expect("failed to spawn");

            let mut file = std::fs::File::create("data.txt").expect("create failed");
            file.write_all("Hello World".as_bytes()).expect("write failed");
            println!("data written to file" );

        });
        // The usage is similar as with the standard library's `Command` type

    }
}


// let server = thread::spawn(move || {
//     Command::new("cmd")
//         .current_dir("server")
//         .args(&["/C", "java -jar server.jar --nogui"])
//         .stdout(Stdio::inherit())
//         .spawn()
//         .expect("ls command failed to start");
// });
