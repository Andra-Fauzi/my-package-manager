use serde::{Serialize, Deserialize};
use serde_json::Result;

use std::{fs::File, io::{Read, Write}, path::Path};
use std::collections::HashMap;

// #[derive(Serialize)]
// struct Halo {
//     nama: String,
//     umur: u32
// }

// pub fn test() {
//     let selamat = Halo {
//         nama: "halo".to_owned(),
//         umur: 123
//     };

//     let json = serde_json::to_string_pretty(&selamat).unwrap();
//     println!("the json, {}", json);

//     let mut file = File::create("manifest.json").unwrap();

//     file.write(json.as_bytes()).unwrap();
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub name: String,
    pub version: f32,
    pub dependencies: Option<HashMap<String, f32>>
}

impl Manifest {
    pub fn new(name: String, version: f32) -> Self {
        Manifest { name: name, version: version , dependencies: None }
    }
    pub fn add_dependencies(&mut self, name: String, version: f32) {
        self.dependencies.get_or_insert_with(HashMap::new).insert(name, version);
        println!("{:#?}",self.dependencies);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lock {
    pub name: String,
    pub version: f32,
    pub source: String,
    pub checksum: String,
    pub dependecies: Option<HashMap<String, f32>>
}

impl Lock {
    pub fn new(name: String, version: f32, source: String, checksum: String, dependencies: Option<HashMap<String, f32>>) -> Self {
        Lock { name: name, version: version, source: source, checksum: checksum, dependecies: dependencies }
    }
}

pub fn read_manifest_package(path: &Path) -> (Manifest, Lock) {
    let mut manifest_file = File::open(path.join("manifest.json")).unwrap();
    let mut lock_file = File::open(path.join("lock.json")).unwrap();

    let mut manifest_str = String::new();
    let mut lock_str = String::new();

    manifest_file.read_to_string(&mut manifest_str).unwrap();
    lock_file.read_to_string(&mut lock_str).unwrap();

    let manifest_struct: Manifest = serde_json::from_str(&manifest_str).expect("cannot translate manifest");
    let lock_struct: Lock = serde_json::from_str(&lock_str).expect("cannot translate lock");

    (manifest_struct, lock_struct)
}

pub fn create_manifest_package(path: &Path, manifest: &Manifest) {
    println!("creating manifest file");
    let mut manifest_file = File::create(path.join("manifest.json")).unwrap();

    let manifest_string = serde_json::to_string_pretty(manifest).unwrap();

    println!("writing manifest file");
    manifest_file.write_all(manifest_string.as_bytes()).unwrap();
}

pub fn add_lock_package(path: &Path, lock: &Lock) {
    println!("creating lock file");
    let mut lock_file = File::create(path.join("lock.json")).unwrap();

    let lock_string = serde_json::to_string_pretty(lock).unwrap();

    println!("writing lock file");
    lock_file.write_all(lock_string.as_bytes()).unwrap();
}