use rest_server::app::web::Server;

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Server::new("application.toml")
        .await;
    sqlx::migrate!()
        .run(&app.state.pool)
        .await
        .expect("Failed to migrate tables.");

    app.run().await;
}