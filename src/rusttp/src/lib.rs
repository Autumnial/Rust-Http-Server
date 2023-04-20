use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, collections::HashMap};

pub mod state;
pub use state::{ContentType, Request, Response, Status};

pub struct Server {
    address: String,
    listener: TcpListener,
    routes: HashMap<String, fn() -> (String, ContentType)>
}

pub struct ServerBuilder {
    address: String,
    tcp_listener: Option<TcpListener>,
    routes: HashMap<String, fn() -> (String, ContentType)>
}

impl Server {
    pub fn run(mut self) {
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
    
       let response = self.route(&request);

        
        stream.write_all(response.send().as_bytes()).unwrap();
    }

    fn handle_request(request: String) -> Request{
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

        Request {
            method: method.to_string(),
            path: path.to_string(),
            headers,
            body: None
        }
    }


    /// only handles GET requests for now.
    fn route(&self, request: &Request) -> Response {
        let content; 
        let content_type;

        for route in &self.routes {
            if route.0 == &request.path {
                let out = route.1();
                content = out.0; 
                content_type = out.1;

                return Response::new(content)
                    .set_status_code(Status::Ok)
                    .set_content_type(content_type)
                    .build();
            }
        }
        Response::new("404".to_string())
            .set_status_code(Status::NotFound)
            .build()
    }
}

impl ServerBuilder {
    pub fn launch(self) {
        let listener = TcpListener::bind(self.address.clone()).unwrap();
        let server = Server {
            address: self.address,
            routes: self.routes,
            listener,
        };

        server.run();
    }

    pub fn add_route(mut self, path: String, handler: fn() -> (String, ContentType)) -> Self {
        self.routes.insert(path, handler);
        self
    }
}

pub fn build(address: String) -> ServerBuilder {
    ServerBuilder {
        address,
        routes: HashMap::new(),
        tcp_listener: None,
    }
}
