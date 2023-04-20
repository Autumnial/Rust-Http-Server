
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
    pub content: String,
    pub content_type: String,
    pub status_code: Status,
    pub headers: Vec<String> 
}

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<String>,
    pub body: Option<String>,
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
    pub fn set_content_type(mut self, content_type: ContentType) -> Self{
        self.content_type = Some(content_type);
        self
    }

    pub fn set_status_code(mut self, status_code: Status) -> Self{
        self.status_code = status_code;
        self
    }
    
    pub fn add_header(mut self, header: String) -> Self{
        self.headers.push(header);
        self
    }

    pub fn build(self) -> Response{
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
