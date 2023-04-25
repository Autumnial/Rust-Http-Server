use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

pub mod state;
pub use state::{ContentType, Request, Response, Status};

use crate::state::Method;

pub fn load_file(path: &str) -> Option<String> {
    let file = File::open(path);

    let file = match file {
        Ok(file) => file,
        Err(_) => return None,
    };

    let file = BufReader::new(file);

    Some(
        file.lines()
            .map(|l| l.unwrap())
            .collect::<Vec<String>>()
            .join("\n"),
    )
}

pub struct Server {
    address: String,
    listener: TcpListener,
    get: HashMap<String, fn() -> Response>,
    post: HashMap<String, fn(Request) -> Response>,
}

impl Server {
    pub fn run(self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut request_bytes: [u8; 2048] = [0; 2048];
        stream.read(&mut request_bytes).unwrap();
        let request = String::from_utf8_lossy(&request_bytes[..]);

        let request = Self::handle_request(request.to_string());

        let response = self.route(request);

        stream.write_all(response.send().as_bytes()).unwrap();
    }

    fn handle_request(request: String) -> Request {
        let mut lines = request.lines();
        let first_line = lines.next().unwrap();
        let parts: Vec<&str> = first_line.split(" ").collect();
        let method = parts[0];
        let path = parts[1];

        println!("{} for {}", method, path);

        let mut headers = Vec::new();
        for line in lines {
            if line == "" {
                break;
            }
            headers.push(line.to_string());
        }

        let method = match method {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "OPTIONS" => Method::OPTIONS,
            _ => Method::UNKNOWN,
        };

        Request {
            method,
            path: path.to_string(),
            headers,
            body: "".to_string(),
        }
    }

    fn route(&self, request: Request) -> Response {
        match request.method {
            Method::GET => {
                let handler = self.get.get(&request.path);

                match handler {
                    Some(handler) => {
                        return handler();
                    }
                    None => return not_found(),
                };
            }
            Method::POST => {
                let handler = self.post.get(&request.path);

                match handler {
                    Some(handler) => {
                        return handler(request);
                    },
                    None => {
                        return not_found();
                    }
                };
            }
            _ => {
                return not_found();
            }
        }
    }
}

pub struct ServerBuilder {
    address: String,
    get: HashMap<String, fn() -> Response>,
    post: HashMap<String, fn(Request) -> Response>,
}

impl ServerBuilder {
    pub fn launch(self) {
        let listener = TcpListener::bind(self.address.clone()).unwrap();
        let server = Server {
            address: self.address,
            get: self.get,
            post: self.post,
            listener,
        };

        server.run();
    }

    pub fn get(
        mut self,
        path: String,
        handler: fn() -> Response,
    ) -> Self {
        self.get.insert(path, handler);
        self
    }

    pub fn post(
        mut self,
        path: String,
        handler: fn(Request) -> Response,
        ) -> Self{
        self.post.insert(path, handler);
        self
    }
}

pub fn build(address: String) -> ServerBuilder {
    ServerBuilder {
        address,
        get: HashMap::new(),
        post: HashMap::new(),
    }
}

pub fn not_found() -> Response {
    Response::new("")
        .set_status_code(Status::NotFound)
        .set_content_type(ContentType::Text)
        .build()
}
