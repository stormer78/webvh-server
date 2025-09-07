#[tokio::main]
async fn main() {
    // Serve files from the "static" directory
    let routes = warp::fs::dir("assets");

    println!("Serving static files from ./assets on http://0.0.0.0:8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
