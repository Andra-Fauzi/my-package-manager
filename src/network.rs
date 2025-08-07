use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, path::Path};

use std::thread;

use crate::package::{convert_byte_to_zip, unzip};

// this function is Server that listening to all client it returns some result Ok or Err
pub fn server(mut data: Vec<u8>) -> std::io::Result<()> {
    // this are tcp that listen to some ip and port
    let listener = TcpListener::bind("127.0.0.1:8767")?;
    println!("Server listening on 127.0.0.1:8767");

    // this will capture all stream and accept them automatically
    // like listener.accept() but it will not stop with one stream
    for stream in listener.incoming() {

        match stream {
            Ok(mut stream) => {
                
                // get peer and client ip
                let peer_addr = stream.peer_addr().unwrap();
                let ip = peer_addr.ip();
                println!("get connection from, {}",ip.to_string());

                // read client message
                let mut buffer = [0u8;1024];
                stream.read(&mut buffer);
                let string_buffer = String::from_utf8_lossy(&buffer);
                println!("get data from client, {}", string_buffer);

                // this will handle client 
                let data_clone = data.clone();
                thread::spawn(move || handle_client(stream, &data_clone));

            }

            Err(e) => println!("Connection failed, {}", e)
        }
    }

    Ok(())
}


pub fn client() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8767")?;

    let data = "hello world!";

    stream.write_all(data.as_bytes())?;

    println!("sent messsage, {}", data);

    let mut len_data: usize = 0;

    let mut length_data_buffer = [0u8;2];

    stream.read_exact(&mut length_data_buffer);

    let length_data = u16::from_be_bytes(length_data_buffer);

    println!("how much data is: {}", length_data);

    let mut all_data: Vec<u8> = Vec::new();

    loop {
        let mut len_buf = [0u8;2];
        if stream.read_exact(&mut len_buf).is_err() {
            println!("connection closed error");
            break;
        }

        let len = u16::from_be_bytes(len_buf) as usize;

        let mut chunk_buf = vec![0u8;len];
        stream.read_exact(&mut chunk_buf).expect("failed to read chunk");

        all_data.append(&mut chunk_buf);

        // println!("received {} bytes", len);
        len_data += len;
    }
    println!("all lenght data, {}", len_data);

    stream.write_all(b"done");

    convert_byte_to_zip(all_data);

    unzip(Path::new("halo.zip"), Path::new("sigma"));

    Ok(())
}

fn handle_client(mut stream: TcpStream, data: &Vec<u8>) {
    println!("all length of data from server, {}", data.len());
    
    // sent to client how much data will sent
    let length_data = data.len() as u16;
    stream.write_all(&length_data.to_be_bytes());

    for chunk in data.chunks(1024) {
        let len = chunk.len() as u16;
        stream.write_all(&len.to_be_bytes()).expect("failed to write length");
        stream.write_all(chunk).expect("failed to write chunk");
    }

    // stream.flush().unwrap();
    // std::thread::sleep(std::time::Duration::from_millis(100));
    stream.shutdown(std::net::Shutdown::Write);
    
    let mut buffer = [0u8;1024];
    stream.read(&mut buffer);
    let strbuffer = String::from_utf8_lossy(&buffer);
    println!("get done from client, {}", strbuffer);
    
    // loop {
    //     println!("check if done");
    //     let mut buffer = [0u8;1024];
    //     stream.read(&mut buffer);
    //     let done = String::from_utf8_lossy(&buffer);
    //     if done.contains("done") {

    //         println!("done");
    //         println!("all data sent.");
    //         break;
    //     }
    //     else {
    //         continue;
    //     }
    // }

}