use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    }).bind(("127.0.0.1", 3000))?
        .run()
        .await
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test code
}
