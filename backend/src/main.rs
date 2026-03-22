#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> Result<(), String> {
    fmn_backend::run().await
}
