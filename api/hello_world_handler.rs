use vercel_runtime::{run, Body, Error as VercelError, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), VercelError> {
    run(handler).await
}

pub async fn handler(_: Request) -> Result<Response<Body>, VercelError> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body("Whatevs homieee hemloo dude".into())
        .map_err(|e| e.into())
}
