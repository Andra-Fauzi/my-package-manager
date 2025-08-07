use std::{fs::File,io::{Read, Seek, Write}, path::{self, Path}};

use zip::{write::{FileOptions, ZipWriter}, ZipArchive};

pub fn write_zip() -> zip::result::ZipResult<()> {
    let file = File::create("archive.zip")?;
    let mut zip = ZipWriter::new(file);

    let opts = FileOptions::<()>::default();

    let mut some_file = File::open("hello.txt")?;
    let mut contents = String::new();
    some_file.read_to_string(&mut contents)?;

    zip.start_file("hello.txt", opts)?;
    zip.write_all(contents.as_bytes())?;
    zip.finish()?;
    Ok(())
}

pub fn unzip(path_zip: &Path, path_extract: &Path) -> zip::result::ZipResult<()> {
    let file = File::open(path_zip)?;
    let mut archive = ZipArchive::new(file)?;

    archive.extract(path_extract)?;
    Ok(())
}

pub fn convert_zip_to_byte(path: &Path) -> Vec<u8>{
    let mut some_file = File::open(path).unwrap();
    let metadata = some_file.metadata().unwrap();
    let mut some_buffer = vec![0; metadata.len() as usize];

    some_file.read(&mut some_buffer).unwrap();

    return some_buffer;
}

pub fn convert_byte_to_zip(data: Vec<u8>) {
    let mut some_file = File::create("halo.zip").unwrap();
    some_file.write_all(&data);
}