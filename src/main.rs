mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    warp::serve(routes::routes())
        .run(([127, 0, 0, 1], 3030))
        .await;
}
