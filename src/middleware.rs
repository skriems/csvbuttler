use actix_cors::Cors;
use actix_web::http::header;

use crate::config::get_config;

pub fn cors() -> Cors {
    // TODO: would be nice if we wouldn't `get_config` for all threads
    // convert the `Result` into an `Option`
    let config = get_config().ok();

    let allowed_headers = vec![
        header::ACCEPT,          // not respected
        header::ACCEPT_ENCODING, // gzip
        header::AUTHORIZATION,
    ];

    if let Some(cfg) = config {
        if let Some(origin) = cfg.allow_origin {
            // TODO: handle the "*"
            return Cors::new()
                .allowed_origin(&origin)
                .allowed_methods(vec!["GET"])
                .allowed_headers(allowed_headers)
                .max_age(3600);
        };
    };

    // default CORS rules if nothing is specified via cli or env.
    // `Origin` is simply echoed back
    Cors::new()
        .allowed_methods(vec!["GET"])
        .allowed_headers(allowed_headers)
        .max_age(3600)
}
