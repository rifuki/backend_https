use actix_web::{
    HttpServer, 
    App, 
    web, 
    middleware
};

use rst04_jwt::{
    AppState, 
    establish_connection,
    auth::scoped_auth,
    user::scoped_user
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* * rust logging */
    std::env::set_var("RUST_LOG", "ERROR");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    /* * end rust logging*/

    /* * .env import */
    dotenv::dotenv().ok();
    /* * end .env import */
    /* * env var */
    let app_port = std::env::var("APP_PORT").unwrap_or("80".to_string());
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|e| {
        let error_message = "DATABASE URL must be set first.";
        eprintln!("{} [{}]", error_message, e);
        std::process::exit(1);
    });
    /* * end env var */

    /* database pool */
    let db_pool = establish_connection(&db_url).await;
    let app_state = web::Data::new( AppState { db_pool } );
    /* end database pool */

    /* * backbone server */
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::NormalizePath::trim())
            .configure(scoped_auth)
            .configure(scoped_user)
    })
    .bind(format!("0.0.0.0:{}", app_port))?
    .run()
    .await
    /* * end backbone server */
}