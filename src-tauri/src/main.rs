// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(lazy_cell)]

mod macros;

use std::collections::HashMap;
use std::sync::LazyLock;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Arguments {
    method: String,
    target: String,
    body: Option<String>,
    headers: HashMap<String, String>,
}

pub(crate) const UA: &str = "User-Agent";
pub(crate) const UA_VALUE: &str = "tauri-fetch";

static CLIENT: LazyLock<ureq::Agent> =
    LazyLock::new(|| ureq::AgentBuilder::new().build());

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn fetch(args: Arguments) -> String {
    match request(args) {
        Ok(ok) => ok,
        Err(err) => err.to_string(),
    }
}

fn request(
    args: Arguments
) -> Result<String> {
    let mut request = CLIENT
        .request(&args.method, &args.target)
        .set(UA, UA_VALUE);

    for (key, value) in &args.headers {
        request = request.set(key, value);
    }

    let res =
        if let Some(body) = args.body {
            request.send_string(&body)
        } else {
            request.call()
        };

    res?.into_string().map_err(From::from)
}
