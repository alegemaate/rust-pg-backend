use actix_web::{middleware, App, HttpServer};
use rust_backend::app_config::config_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Hello world");

  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .configure(config_app)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
