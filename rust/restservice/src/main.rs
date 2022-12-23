use warp::Filter; // 1.

#[tokio::main] // 2.
pub async fn main() {
    let hello = warp::path!("hello" / String) // 3.
        .map(|name| format!("Hello, {}!", name)); // 4.

    println!("serving on http://localhost:8080/hello/yo");
    warp::serve(hello) // 5.
        .run(([0, 0, 0, 0], 8080)) // 6.
        .await;
}