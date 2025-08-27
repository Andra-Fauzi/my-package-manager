
use std::{io, path::Path};

use crate::{manifest::{create_manifest_package, read_manifest_package}, package::{convert_zip_to_byte, convert_zip_to_hash}};

mod network;
mod package;
mod manager;
mod manifest;

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


    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("cannot read line");

    // println!("{}",input);

    // if input.contains("client") {
    //     println!("on client");
    //     let client = network::client();
    //     if let Err(e) = client {
    //         println!("error on server, {}", e);
    //     }
    // }
    // else {
    //     println!("on server");
    //     let data = vec![255; 102400];
    //     let server = network::server(data);
    //     if let Err(e) = server {
    //         println!("error on server, {}", e);
    //     }
    // }

    // convert_zip_to_hash();

    // manifest::test();

    // let result = read_manifest_package(Path::new("packages/script-linux"));

    // create_manifest_package(Path::new("."), &result.0, &result.1);

    manager::start();

}