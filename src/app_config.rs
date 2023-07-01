use actix_web::{web, Error, HttpResponse};

use crate::{
  handlers::{parts, products},
  http_exception::HttpException,
};

async fn default_not_found() -> Result<HttpResponse, Error> {
  Err(
    HttpException::NotFoundException {
      message: String::from("Not Found"),
    }
    .into(),
  )
}

async fn default_method_not_allowed() -> Result<HttpResponse, Error> {
  Err(
    HttpException::MethodNotAllowedException {
      message: String::from("Method Not Allowed"),
    }
    .into(),
  )
}

pub fn config_app(cfg: &mut web::ServiceConfig) {
  // domain includes: /products/{product_id}/parts/{part_id}
  cfg
    .service(
      web::scope("/products")
        .service(
          web::resource("")
            .route(web::get().to(products::get_products))
            .route(web::post().to(products::add_product)),
        )
        .service(
          web::scope("/{product_id}")
            .service(
              web::resource("")
                .route(web::get().to(products::get_product_detail))
                .route(web::delete().to(products::remove_product)),
            )
            .service(
              web::scope("/parts")
                .service(
                  web::resource("")
                    .route(web::get().to(parts::get_parts))
                    .route(web::post().to(parts::add_part)),
                )
                .service(
                  web::resource("/{part_id}")
                    .route(web::get().to(parts::get_part_detail))
                    .route(web::delete().to(parts::remove_part)),
                ),
            ),
        ),
    )
    .default_service(web::to(default_not_found));
}
