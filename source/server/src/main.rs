use axum::{routing::get, Router};
use shuttle_secrets::SecretStore;
use anyhow::anyhow;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}

// #[shuttle_runtime::route]
// async fn secrets(#[shuttle_secrets::Secrets] secret_store: SecretStore,) -> shuttle_axum::ShuttleAxum  {
//     // get secret defined in `Secrets.toml` file.
//     let secret = if let Some(secret) = secret_store.get("MY_API_KEY") {
//         secret
//     } else {
//         return Err(anyhow!("Oops, no secret found").into());
//     };
//     let router = Router::new().route("/secret", get(hello_world));
//     Ok(router.into())

// }