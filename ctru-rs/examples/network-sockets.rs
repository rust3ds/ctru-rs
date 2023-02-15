use ctru::prelude::*;

use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener};
use std::time::Duration;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::init().unwrap();
    let _console = Console::init(gfx.top_screen.borrow_mut());
    let hid = Hid::init().unwrap();
    let apt = Apt::init().unwrap();

    println!("\nlibctru sockets demo\n");

    let soc = Soc::init().unwrap();

    let server = TcpListener::bind("0.0.0.0:80").unwrap();
    server.set_nonblocking(true).unwrap();

    println!("Point your browser to http://{}/\n", soc.host_address());

    while apt.main_loop() {
        gfx.wait_for_vblank();

        match server.accept() {
            Ok((mut stream, socket_addr)) => {
                println!("Got connection from {socket_addr}");

                let mut buf = [0u8; 4096];
                match stream.read(&mut buf) {
                    Ok(_) => {
                        let req_str = String::from_utf8_lossy(&buf);
                        println!("{req_str}");
                    }
                    Err(e) => {
                        if e.kind() == io::ErrorKind::WouldBlock {
                            println!("Note: Reading the connection returned ErrorKind::WouldBlock.")
                        } else {
                            println!("Unable to read stream: {e}")
                        }
                    }
                }

                let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";

                if let Err(e) = stream.write(response) {
                    println!("Error writing http response: {e}");
                }

                stream.shutdown(Shutdown::Both).unwrap();
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::WouldBlock => {}
                _ => {
                    println!("Error accepting connection: {e}");
                    std::thread::sleep(Duration::from_secs(2));
                }
            },
        }

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        };
    }
}
