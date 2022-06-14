extern crate core;

mod connexion_manager;
mod server_manager;

use std::env;
use std::net::TcpListener;
use crate::connexion_manager::connexion_manager::*;
use std::{net, thread};

fn main() {

    let default_port: String = "7878".to_string();
    let mut port: String = "".to_string();

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 && args.len() != 1 {
        println!("Usage: -p <port>");
    } else {
        if args.len() == 1 {
            port = default_port.trim().parse().unwrap();
        } else {
            if &args[1] == "-p" {
                let arg_port: u16 = args[2].parse().unwrap();

                if arg_port < 1024 {
                    println!("Not a good value for port: {}", arg_port);
                } else {
                    port = arg_port.to_string();
                }
            } else {
                print!("Use -p to set the port number");
            }
        }
    }

    println!("Server is launching on {}", "0.0.0.0:".to_owned() + &port);

    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &port).unwrap();

    for flux in listener.incoming() {
        let mut flux = flux.unwrap();
        thread::spawn(move || {
            gestion_connexion(flux);
        });
    }
}