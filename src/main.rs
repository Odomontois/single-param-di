use app::app::{app_life, AppSvc};

#[tokio::main]
async fn main() {
    let app = app_life();
    app.run().await;
}
