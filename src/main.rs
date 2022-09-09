use axum::{response::Json, routing::get, Router};
use reqwest::Client;
use serde_json::{json, Value};
use std::io::Result;
use std::net::SocketAddr;
use std::time::Duration;
use wakey;

const MAC_ADDR: &str = "70:54:b4:cb:14:fe";
const SMARTCENTER_URL: &str = "http://192.168.0.14:56789/apps/SmartCenter";
const TIMEOUT_SECS: u64 = 5;
const TIMEOUT_NANOS: u32 = 0;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/status", get(status))
        .route("/on", get(on))
        .route("/off", get(off));

    // run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Json<Value> {
    Json(json!({ "message": "Hello, from your TV" }))
}

async fn status() -> Json<Value> {
    // Use SmartCentre
    let client = reqwest::Client::builder()
        .timeout(Duration::new(TIMEOUT_SECS, TIMEOUT_NANOS))
        .build()
        .unwrap();

    let response = client
        .post(SMARTCENTER_URL)
        .body("<?xml version='1.0'?><remote><key code='9999'/></remote>")
        .header("Content-Type", "application/xml")
        .send()
        .await;

    match response {
        Ok(_body) => {
            println!("Sent the power keycode!");
            Json(json!({ "message": "running" }))
        }
        Err(err) => {
            println!("{:?}", err);
            Json(json!({ "message": "not-running" }))
        }
    }
}

async fn on() -> Json<Value> {
    // First, use Wake-on-LAN
    let wol = wakey::WolPacket::from_string(MAC_ADDR, ':');

    let wol_resp: Result<()> = wol.send_magic();

    // Second, use SmartCentre
    let client = reqwest::Client::builder()
        .timeout(Duration::new(TIMEOUT_SECS, TIMEOUT_NANOS))
        .build()
        .unwrap();

    let smartcentre_resp = client
        .post(SMARTCENTER_URL)
        .body("<?xml version='1.0'?><remote><key code='1012'/></remote>")
        .header("Content-Type", "application/xml")
        .send()
        .await;

    if wol_resp.is_ok() || smartcentre_resp.is_ok() {
        Json(json!({ "message": "on" }))
    } else {
        Json(json!({ "message": "on" }))
    }
}

async fn off() -> Json<Value> {
    // Use SmartCentre
    let client = smartcentre_client().unwrap();

    let response = client
        .post(SMARTCENTER_URL)
        .body("<?xml version='1.0'?><remote><key code='1012'/></remote>")
        .header("Content-Type", "application/xml")
        .send()
        .await;

    match response {
        Ok(_body) => println!("Sent the power keycode!"),
        Err(err) => println!("{:?}", err),
    }

    Json(json!({ "message": "off" }))
}

fn smartcentre_client() -> reqwest::Result<Client> {
    reqwest::Client::builder()
        .timeout(Duration::new(TIMEOUT_SECS, TIMEOUT_NANOS))
        .build()
}
