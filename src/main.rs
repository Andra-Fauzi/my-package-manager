use clap::{Parser, Subcommand};

use std::{io, path::Path};

use crate::package::convert_zip_to_byte;

mod network;
mod package;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Install {
        name: String,
    },
    Uninstall {
        name: String,
    },
    Search {
        name: String,
    },
    About,
    Url {
        url: String
    },
    Show
}

fn main() {
    // let args = Args::parse();
    // let write = package::write_zip();
    // if let Err(e) = write {
    //     println!("write error {}", e);
    // }
    // let unzip = package::unzip();
    // if let Err(e) = unzip {
    //     println!("write error {}", e);
    // }


    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("cannot read line");

    println!("{}",input);

    if input.contains("client") {
        println!("on client");
        let client = network::client();
        if let Err(e) = client {
            println!("error on server, {}", e);
        }
    }
    else {
        println!("on server");
        let data = convert_zip_to_byte(Path::new("archive.zip"));
        let server = network::server(data);
        if let Err(e) = server {
            println!("error on server, {}", e);
        }
    }
}