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
use std::io::BufWriter;
use std::io::BufRead;
use std::io::Write;
use std::net::Shutdown;

fn read_request(stream: &mut BufReader<TcpStream>) -> Vec<String> {
    let mut headers: Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        let len = stream.read_line(&mut line).unwrap();
        if len == 2 {
            return headers;
        }
        line.pop().unwrap();
        line.pop().unwrap();
        headers.push(line);
    }
}

fn write_reply(headers: Vec<String>, stream: &mut BufWriter<TcpStream>) {
    let ref request = headers[0];
    let items: Vec<&str> = request.split_whitespace().collect();
    let mut content = String::new();
    let mut statuscode = String::new();
    let crlf: Vec<u8> = vec![0xd, 0xa];
    if items[0] == "GET" {
        content.push_str("<html><head><title>Ok</title></head><body>Request accepted</body></html>");
        statuscode.push_str("200 OK");
    } else {
        content.push_str("<html><head><title>Fail</title></head><body>Invalid request</body></html>");
        statuscode.push_str("404");
    }
    let binarycontent = content.into_bytes();
    let mut replyheaders: Vec<String> = Vec::new();
    replyheaders.push("HTTP/1.1 ".to_string() + &statuscode);
    replyheaders.push("Content-Type: text/html; charset=UTF-8".to_string());
    replyheaders.push(format!("Content-Length: {}", binarycontent.len()));
    replyheaders.push("Connection: close".to_string());
    replyheaders.push("".to_string());
    for h in replyheaders {
        stream.write(&h.into_bytes()).unwrap();
        stream.write(&crlf).unwrap();
    }
    stream.write(&binarycontent).unwrap();
}

fn handle_client(stream: TcpStream) {
    let mut br = BufReader::new(stream);
    let headers = read_request(&mut br);
    let mut bw = BufWriter::new(br.into_inner());
    write_reply(headers, &mut bw);
    bw.into_inner().unwrap().shutdown(Shutdown::Both).unwrap();
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

