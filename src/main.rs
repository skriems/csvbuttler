use std::env;
use std::process;

use csvbuttler::data;
use csvbuttler::error;
use csvbuttler::handler;
use csvbuttler::routes;
use csvbuttler::settings::Settings;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::{Compress, DefaultHeaders, Logger};
use actix_web::{web, App, HttpServer};
use chrono::Duration;
use csrf_token::CsrfTokenGenerator;
use env_logger;
use listenfd::ListenFd;

/// Utility funciton to build the server string based on the cli arguments.
/// Defaults to: 127.0.0.1:8000
fn build_server_str(settings: &Settings) -> String {
    format!("{}:{}", settings.default.interface, settings.default.port)
}

fn run() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=trace");
    env::set_var("RUST_BACKTRACE", "1"); // TODO set in dev
    env_logger::init();
    let log_fmt = "%a '%r' %s %b '%{Referer}i' '%{User-Agent}i' %D";
    let state = data::AppState::new()?;
    let settings = Settings::new().map_err(error::Error::ConfigError)?;
    let server_str = build_server_str(&settings);

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            // getting a reference to the data
            .data(state.clone())
            .data(CsrfTokenGenerator::new(
                settings.secrets.csrf.clone().as_bytes().to_vec(),
                Duration::hours(1),
            ))
            // configure logging
            .wrap(Logger::new(log_fmt))
            // enable compression if requested via headers
            .wrap(Compress::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&settings.secrets.app.as_bytes())
                    .domain(&settings.default.domain)
                    .name("jwt_token")
                    .path("/")
                    .max_age(Duration::days(1).num_seconds())
                    .secure(settings.default.https),
            ))
            // this is our root handler
            .route("/", web::get().to(handler::index))
            .service(
                web::scope("/products")
                    .wrap(DefaultHeaders::new().header("Cache-Control", "max-age=3600"))
                    .configure(routes::config),
            )
            .service(
                web::resource("/auth")
                    .route(web::post().to(handler::login))
                    .route(web::delete().to(handler::logout)),
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        println!("Listening on {}", &server_str);
        server.bind(server_str)?
    };

    server.run()
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
