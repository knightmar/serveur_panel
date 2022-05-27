pub mod connexion_manager {
    use std::io::{BufRead, BufReader};
    use std::net::TcpStream;
    use std::{thread, time};
    use std::process::exit;

    pub fn gestion_connexion(flux: TcpStream) {
        println!("New connection from : {}", &flux.peer_addr().unwrap());

        let mut reader = BufReader::new(&flux);
        loop {
            let mut line = String::new();

            reader.read_line(&mut line).expect("TODO: panic message");

            line = line.trim().parse().unwrap();
            if line == "exit" {
                println!("exiting");
                thread::sleep(time::Duration::from_secs(3));
                exit(0);
            }
            println!(
                "New line from {}  : \"{}\" ",
                &flux.peer_addr().unwrap(),
                line
            );
        }
    }
}