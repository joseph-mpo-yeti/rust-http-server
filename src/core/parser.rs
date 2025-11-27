use crate::core::logging::Logging;

use crate::types::method::*;
use crate::types::request::*;
use crate::types::response::*;

use crate::types::method::*;
use crate::types::request::*;
use crate::types::response::*;

use std::{
    collections::HashMap,
    io::{Error, Read, Write},
    sync::{Arc, Mutex},
};

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub struct Parser {
    logging_enabled: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            logging_enabled: false,
        }
    }

    pub async fn parse_http_request(&self, socket: &mut TcpStream) -> Result<HttpRequest, Error> {
        let mut request_content = String::new();
        let mut buf = [0u8; 1024];
        loop {
            match socket.read(&mut buf).await {
                Ok(size) => {
                    if size > 0 {
                        let st = std::str::from_utf8(&buf[0..size]).unwrap();
                        request_content.push_str(st);
                        match request_content.find("\r\n\r\n") {
                            Some(_) => {
                                break;
                            }
                            _ => {}
                        }
                    } else {
                        break;
                    }
                }
                Err(err) => {
                    if self.logging_enabled {
                        log::info!("There was an error: {err:?}");
                    }
                    break;
                }
            }
        }

        let mut req: Vec<&str> = request_content
            .split("\r\n\r\n")
            .filter(|x| !x.is_empty())
            .collect();

        log::debug!("parsed-request: {req:?}");

        let req_headers = match req.get(0) {
            Some(st) => st,
            None => "",
        };

        let mut lines = req_headers.lines();
        let request_line = match lines.next() {
            Some(first_line) => Some(self.parse_request_line(first_line)),
            _ => None,
        };

        match request_line {
            Some(request_line) => {
                if request_line.0 == HttpRequestMethod::UNKNOWN {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid request method",
                    ));
                }

                log::debug!("parsed-headers: {req_headers:?}");

                let mut headers = self.parse_headers(lines);
                let body = self.parse_request_body(socket, req, &headers).await;

                Ok(HttpRequest::new(
                    request_line.0,
                    request_line.1,
                    request_line.2,
                    body,
                    headers,
                ))
            }
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid request",
            )),
        }
    }

    fn parse_request_line(&self, line: &str) -> RequestLine {
        let collect: Vec<&str> = line.split(' ').collect();
        let method = match *collect.get(0).unwrap() {
            "GET" => HttpRequestMethod::GET,
            "POST" => HttpRequestMethod::POST,
            "PUT" => HttpRequestMethod::PUT,
            "PATCH" => HttpRequestMethod::PATCH,
            "HEAD" => HttpRequestMethod::HEAD,
            "OPTIONS" => HttpRequestMethod::OPTIONS,
            "DELETE" => HttpRequestMethod::DELETE,
            "CONNECT" => HttpRequestMethod::CONNECT,
            "TRACE" => HttpRequestMethod::TRACE,
            _ => HttpRequestMethod::UNKNOWN,
        };

        RequestLine(
            method,
            String::from(*collect.get(1).unwrap_or(&"")),
            String::from(*collect.get(2).unwrap_or(&"")),
        )
    }

    fn parse_headers(&self, lines: std::str::Lines<'_>) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        for l in lines {
            let header: Vec<&str> = l.split(':').collect();
            let key = self.get_str(header.get(0));
            let value = self.get_str(header.get(1));
            if !key.is_empty() && !value.is_empty() {
                headers.insert(key, value);
            }
        }
        headers
    }

    async fn parse_request_body(
        &self,
        socket: &mut TcpStream,
        req: Vec<&str>,
        headers: &HashMap<String, String>,
    ) -> String {
        let content_length = match headers.get("Content-Length") {
            Some(v) => v.parse::<u32>().unwrap(),
            _ => 0,
        };

        if content_length > 0 {
            let mut req_body = req.get(1).unwrap_or(&"").to_string();
            if self.logging_enabled() {
                log::debug!(
                    "Content-Length is {content_length} and parsed body size is {}",
                    req_body.len()
                );
            }

            let mut buf = [0u8; 1024];

            while (req_body.len() as u32) < content_length {
                match socket.read(&mut buf).await {
                    Ok(c) => {
                        let st = std::str::from_utf8(&buf[0..c]).unwrap();
                        req_body.push_str(st);
                    }
                    Err(err) => {
                        log::error!(
                            "There was an error while parsing the request body! Error: {err:?}"
                        );
                    }
                }
            }
            req_body
        } else {
            String::new()
        }
    }

    fn get_str(&self, opt: Option<&&str>) -> String {
        match opt {
            Some(v) => String::from(v.trim()),
            None => String::new(),
        }
    }
}

impl Logging for Parser {
    fn enable_logging(&mut self) {
        self.logging_enabled = true;
    }
    fn disable_logging(&mut self) {
        self.logging_enabled = true;
    }

    fn logging_enabled(&self) -> bool {
        self.logging_enabled
    }
}
