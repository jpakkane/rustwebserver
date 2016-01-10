/*
 * Copyright 2016 Jussi Pakkanen
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 */

use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use std::io::BufRead;
use std::net::Shutdown;

fn read_request(stream: &mut BufReader<TcpStream>) {
    loop {
        let mut line = String::new();
        let len = stream.read_line(&mut line).unwrap();
        if len == 2 {
            return;
        }
        println!("{}", line);
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut bs = BufReader::new(stream);
    read_request(&mut bs);
    bs.into_inner().shutdown(Shutdown::Both);
}

fn run_server(address: &str) {
    println!("Starting web server at {}.", address);
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Connection failed:\n{}\n", e);
            }
        }
    }
}

fn main() {
    let address = "127.0.0.1:1234";
    run_server(address);
}

