use actix_web::{
    HttpServer, 
    App, 
    web, 
    middleware
};

use rst04_jwt::{
    AppState, 
    establish_connection,
    scoped_auth,
    scoped_user
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
    let secret_access_token = std::env::var("SECRET_ACCESS_TOKEN").unwrap_or_else(|e| {
        let error_message = "SECRET_ACCESS_TOKEN must be set.";
        eprintln!("{} [{}]", error_message, e);
        std::process::exit(1);
    });
    let secret_refresh_token = std::env::var("SECRET_REFRESH_TOKEN").unwrap_or_else(|e| {
        let error_message = "SECRET_REFRESH_TOKEN must be set.";
        eprintln!("{} [{}]", error_message, e);
        std::process::exit(1);
    });
    /* * end env var */

    /* database pool */
    let db_pool = establish_connection(&db_url).await;
    /* end database pool */

    let app_state = web::Data::new( 
        AppState { db_pool, secret_access_token, secret_refresh_token } 
    );
    /* * backbone server */
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::NormalizePath::trim())
            .service(
                web::scope("/api")
                    .configure(scoped_auth)
                    .configure(scoped_user)
            )
    })
    .bind(format!("0.0.0.0:{}", app_port))?
    .run()
    .await
    /* * end backbone server */
}