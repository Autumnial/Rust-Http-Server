use rusttp::{load_file, ContentType, Request, Response};

fn main() {
    println!("Listening at http://127.0.0.1:8000");

    rusttp::build("127.0.0.1:8000".to_owned())
        .get("/".to_owned(), root)
        .get("/greet".to_owned(), greet)
        .post("/post".to_owned(), posting)
        .launch();
}

fn root() -> Response {
    let content = "";

    Response::new(content).build()
}

fn greet() -> Response {
    let content = "Hello, World!";

    Response::new(content).build()
}

fn posting(req: Request) -> Response {
    let binding = req.body.clone();
    let content = binding.as_str();

    Response::new(content).build()
}
