use clap::{Parser, Subcommand};
use std::{collections::HashMap, hash::Hash, path::Path};

use crate::manifest::{create_manifest_package, Lock, Manifest};

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
    Show,
    Init {
        name: String
    }
}

pub fn start() {
    let args = Args::parse();
    match args.command {
        Command::Install { name } => { println!("{}", name) },
        Command::Uninstall { name } => { println!("{}", name) },
        Command::Search { name } => { println!("{}", name) },
        Command::About => { println!("warung bakmi, warung bakmiiiii") },
        Command::Url { url } => { println!("{}", url) },
        Command::Show => { println!("Show what?") },
        Command::Init { name } => {init(name);}
    }
}

pub fn init(name: String) {
    println!("build manifest");
    let mut manifest = Manifest::new(name.clone(), 0.1);
    println!("build lock");
    let lock = Lock::new(name.clone(), 0.1, String::from(""), String::from(""), None);

    manifest.add_dependencies(String::from("halo"), 1.0);
    manifest.add_dependencies(String::from("sigma"), 1.0);
    manifest.add_dependencies(String::from("ohio"), 1.0);

    create_manifest_package(Path::new("."), &manifest);
}