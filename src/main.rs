use std::env;
use std::io::prelude::*;
use std::net::TcpStream;

fn connect(host: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(host.to_string() + ":80")?;

    let request = format!(
        "\
         GET / HTTP/1.1\r\n\
         Host: {0}:{1}\r\n\
         Accept: */*\r\n\
         Connection: close\r\n\
         \r\n\
         ",
        host, 80
    );
    println!("{}", request);

    stream.write(request.as_bytes())?;

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    println!("{}", buffer);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut args = Vec::new();
    for arg in env::args() {
        args.push(arg);
    }

    println!("{}", args[0]);
    connect(&args[1])?;

    Ok(())
}
