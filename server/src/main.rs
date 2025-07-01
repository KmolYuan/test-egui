use actix_files::{Files, NamedFile};
use actix_web::{App, Either, HttpResponse, HttpServer, get, http::header, web::Path};

const ADDR: &str = "127.0.0.1:8000";

#[get("/{path:.*}")]
async fn index(path: Path<String>) -> impl actix_web::Responder {
    if path.is_empty() || path.ends_with('/') {
        Either::Left(NamedFile::open("docs/index.html").unwrap())
    } else {
        Either::Right(
            HttpResponse::PermanentRedirect()
                .append_header((header::LOCATION, format!("/{path}/")))
                .finish(),
        )
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "docs/static"))
            .service(index)
    })
    .bind(ADDR)?
    .run()
    .await
}
