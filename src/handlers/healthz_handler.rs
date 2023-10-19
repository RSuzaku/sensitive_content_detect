pub async fn healthz() -> String {
    println!("healthz request");
    String::from("server is healthy")
}