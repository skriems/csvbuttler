use actix_cors::Cors;

pub fn cors() -> Cors {
    // TODO make this configurable
    Cors::new().allowed_methods(vec!["GET"]).max_age(3600)
}
