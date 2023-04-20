use rusttp::ContentType;

fn main(){
    rusttp::build("127.0.0.1:8000".to_owned())
    .add_route("/".to_owned(), root)
    .add_route("/test".to_owned(), test)
    .launch(); 
}


fn root() -> (String, ContentType){
    (String::from(r#"
                <!DOCTYPE HTML>
                <html>
                    <head>
                        <title>RustTP</title>
                    </head>
                    <body>
                        <h1>Hello World</h1>
                    </body>"#.to_owned()), 
                    ContentType::Html)
}


fn test () -> (String, ContentType){
    (String::from(r#"
                <!DOCTYPE HTML>
                <html>
                    <head>
                        <title>RustTP</title>
                    </head>
                    <body>
                        <h1>Test</h1>
                    </body>"#.to_owned()), 
                    ContentType::Html)
}

