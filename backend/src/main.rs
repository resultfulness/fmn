#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() {
    fmn_backend::run().await
}
