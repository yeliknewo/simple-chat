use std::io::Result as IoResult;

use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};

pub fn start_client<A: ToSocketAddrs>(addr: A) -> IoResult<()> {

    println!("Starting Client");

    let mut stream: TcpStream = TcpStream::connect(addr)?;

    write!(stream, "Hello World")?;

    Ok(())

}
