//! Network Sockets example.
//!
//! This example showcases the use of network sockets via the `Soc` service and the standard library's implementations.

use ctru::prelude::*;

use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener};
use std::time::Duration;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().unwrap();
    let _console = Console::new(gfx.top_screen.borrow_mut());
    let mut hid = Hid::new().unwrap();
    let apt = Apt::new().unwrap();

    println!("\nlibctru sockets demo\n");

    // Owning a living handle to the `Soc` service is required to use network functionalities.
    let soc = Soc::new().unwrap();

    // Listen on the standard HTTP port (80).
    let server = TcpListener::bind("0.0.0.0:80").unwrap();
    server.set_nonblocking(true).unwrap();

    println!("Point your browser to http://{}/\n", soc.host_address());
    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        };

        match server.accept() {
            Ok((mut stream, socket_addr)) => {
                println!("Got connection from {socket_addr}");

                // Print the HTTP request sent by the client (most likely, a web browser).
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

                // Return a HTML page with a simple "Hello World!".
                let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";

                if let Err(e) = stream.write(response) {
                    println!("Error writing http response: {e}");
                }

                // Shutdown the stream (depending on the web browser used to view the page, this might cause some issues).
                stream.shutdown(Shutdown::Both).unwrap();
            }
            Err(e) => match e.kind() {
                // If the TCP socket would block execution, just try again.
                std::io::ErrorKind::WouldBlock => {}
                _ => {
                    println!("Error accepting connection: {e}");
                    std::thread::sleep(Duration::from_secs(2));
                }
            },
        }

        gfx.wait_for_vblank();
    }
}
