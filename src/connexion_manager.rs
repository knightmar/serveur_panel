pub mod connexion_manager {
    use std::io::{BufRead, BufReader, BufWriter, Write};
    use std::net::TcpStream;
    use std::{thread, time};
    use std::process::exit;
    use crate::server_manager::server_manager::*;

    pub fn gestion_connexion(flux: TcpStream) {
        println!("New connection from : {}", &flux.peer_addr().unwrap());

        let mut reader = BufReader::new(&flux);
        let mut writer = BufWriter::new(&flux);

        loop {
            let mut line = String::new();

            reader.read_line(&mut line).expect("TODO: panic message");

            line = line.trim().parse().unwrap();


            match line.as_str() {
                "exit" => {
                    println!("exiting");
                    thread::sleep(time::Duration::from_secs(3));
                    exit(0);
                }
                "start" => {
                    println!("start");
                    start_server(&flux);
                }
                "stop" => {
                    println!("stop");
                }
                _ => {
                    line.push_str("\n");
                    writer.write_all(line.as_ref()).unwrap();
                    writer.flush().unwrap();
                    println!(
                        "New line from {}  : \"{}\" ",
                        flux.peer_addr().unwrap(),
                        line
                    );
                }
            }
        }
    }
}