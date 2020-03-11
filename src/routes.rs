use actix_web::web;

use crate::handler;
use crate::middleware::cors;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // TODO: paginated list of products
        // .service(web::resource("")
        //     .route(web::get().to_async(handler::tbc))
        // )
        // .route("/", web::get().to(handler::index))
        web::resource("/{id}")
            .name("product")
            .wrap(cors())
            .route(web::get().to_async(handler::product)),
    );
}
