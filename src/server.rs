use std::io::Read;

use std::io::Result as IoResult;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub fn start_server<A: ToSocketAddrs>(addr: A) -> IoResult<()> {

    let listener = TcpListener::bind(addr)?;

    for stream in listener.incoming() {

        handle_client(stream?)?;

        return Ok(());

    }

    Ok(())

}

pub fn handle_client(stream: TcpStream) -> IoResult<()> {

    let mut vec: Vec<u8> = Vec::new();

    for byte in stream.bytes() {

        vec.push(byte?);

    }

    println!("Message: {0}", String::from_utf8(vec).unwrap());

    Ok(())

}
