use std::env;
use std::process;

use csvbuttler::data;
use csvbuttler::handler;
use csvbuttler::routes;

use actix_web::{middleware, web, App, HttpServer};
use env_logger;
use listenfd::ListenFd;

fn build_server_str(state: &data::StateType) -> String {
    let cfg = &state.lock().unwrap().cfg;
    format!("{}:{}", &cfg.interface, &cfg.port)
}

fn run() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=trace");
    env::set_var("RUST_BACKTRACE", "1"); // TODO set in dev
    env_logger::init();

    let log_fmt = "%a '%r' %s %b '%{Referer}i' '%{User-Agent}i' %D";

    let state = data::AppState::new()?;

    let server_str = build_server_str(&state);

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            // getting a reference to the data
            .data(state.clone())
            // configure logging
            .wrap(middleware::Logger::new(log_fmt))
            // enable compression if requested via headers
            .wrap(middleware::Compress::default())
            // this is our root handler
            .route("/", web::get().to(handler::index))
            // configure the rest of the app
            .configure(routes::config)
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
