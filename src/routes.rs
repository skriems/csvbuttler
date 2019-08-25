use actix_web::middleware::DefaultHeaders;
use actix_web::web;

use crate::{handler, middleware::cors};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .wrap(DefaultHeaders::new().header("Cache-Control", "max-age=3600"))
            // TODO: paginated list of products
            // .service(web::resource("")
            //     .route(web::get().to_async(handler::tbc))
            // )
            // .route("/", web::get().to(handler::index))
            .service(
                web::resource("/{id}")
                    .name("product")
                    .wrap(cors())
                    .route(web::get().to_async(handler::product)),
            ),
    );
}
