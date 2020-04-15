#[macro_use]
extern crate afl;

use micro_http::{HttpServer, Response, StatusCode};
use std::io::prelude::*;

fn main() {
	fuzz!(|data: &[u8]| {
		let path_to_socket = "/tmp/test.sock";
		std::fs::remove_file(path_to_socket).unwrap_or_default();

		// Start the server.
		let mut server = HttpServer::new(path_to_socket).unwrap();
		server.start_server().unwrap();

		// Connect a client to the server so it doesn't block in our example.
		let mut socket = std::os::unix::net::UnixStream::connect(path_to_socket).unwrap();
		socket.write_all(data).unwrap();

		let mut i = 0;

		// Server loop processing requests.
		loop {
			for request in server.requests().unwrap() {
				let response = request.process(|request| {
					// Your code here.
					Response::new(request.http_version(), StatusCode::NoContent)
				});
				server.respond(response);
				std::process::exit(0);
			}

			i = i + 1;
			if i >= 2 {
				break;
			}
		}

		socket.shutdown(std::net::Shutdown::Both).unwrap();
		std::fs::remove_file(path_to_socket).unwrap_or_default();
	});
}
