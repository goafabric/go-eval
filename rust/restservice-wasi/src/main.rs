use wstd::http::body::Body;
use wstd::http::{Request, Response, StatusCode};

#[wstd::http_server]
async fn main(req: Request<Body>) -> Result<Response<Body>, wstd::http::Error> {
    println!("serving on http://localhost:8080/hello/yo");
    match req.uri().path().strip_prefix("/hello/") {
        Some(name) if !name.is_empty() && !name.contains('/') => {
            Ok(Response::new(format!("Hello, {}!", name).into()))
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not Found".into())?),
    }
}
