use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, collections::HashMap};

pub enum ContentType{
    Text,
    Html,
    Css,
    Js,
    Json,
}

pub enum Status{
    Ok,
    NotFound,
    BadRequest,
    InternalServerError,
    NotImplemented,
}

pub struct Response {
    // idk 
    content: String,
    content_type: String,
    status_code: Status,
    headers: Vec<String> 
}

struct Request {
    method: String,
    path: String,
    headers: Vec<String>,
    body: Option<String>,
}

impl Response{
    
    pub fn new(content: String) -> ResponseBuilder{
        ResponseBuilder{
            content,
            content_type: None,
            status_code: Status::Ok,
            headers: Vec::new()
        }
    }

    pub fn send(self) -> String{
        let mut response = String::new();
        
        for header in self.headers{
            response.push_str(&header);
            response.push_str("\n");
        }

        response.push_str("\n");
        response.push_str(&self.content);


        return response;
    }
}


pub struct ResponseBuilder{ 
    content: String,
    content_type: Option<ContentType>,
    status_code: Status,
    headers: Vec<String> 
}

impl ResponseBuilder{
    fn set_content_type(mut self, content_type: ContentType) -> Self{
        self.content_type = Some(content_type);
        self
    }

    fn set_status_code(mut self, status_code: Status) -> Self{
        self.status_code = status_code;
        self
    }
    
    fn add_header(mut self, header: String) -> Self{
        self.headers.push(header);
        self
    }

    fn build(self) -> Response{
        let content_type = match self.content_type{
            Some(content_type) => match content_type{
                ContentType::Text => "text/plain",
                ContentType::Html => "text/html",
                ContentType::Css => "text/css",
                ContentType::Js => "text/javascript",
                ContentType::Json => "application/json"
            },
            None => "text/plain"
        };

        let status_code = match self.status_code{
            Status::Ok => "200 OK",
            Status::NotFound => "404 Not Found",
            Status::BadRequest => "400 Bad Request",
            Status::InternalServerError => "500 Internal Server Error",
            Status::NotImplemented => "501 Not Implemented",
        };
        
    
        
        let mut headers = self.headers;
        headers.insert(0, format!("HTTP/1.1 {}", status_code));
        headers.push(format!("Content-Type: {}", content_type));
        headers.push(format!("Content-Length: {}", self.content.len()));
    

        Response{
            content: self.content,
            content_type: content_type.to_string(),
            status_code: self.status_code,
            headers
        }
    }

}

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
