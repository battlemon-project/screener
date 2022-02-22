#[tokio::main]
async fn main() {
    battlemon_screener::run::run().await.expect("Couldn't run");
}
