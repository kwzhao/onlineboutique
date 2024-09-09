use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(no_op))
        .route("/*whatever", get(no_op).post(no_op));
    let listener = TcpListener::bind("0.0.0.0:60000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn no_op() {}
