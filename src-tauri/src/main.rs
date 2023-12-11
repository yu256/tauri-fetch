// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(lazy_cell)]

mod json;

use anyhow::Result;
use json::{read_json, write_json};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::sync::LazyLock;

#[derive(Serialize, Deserialize, Default)]
struct Arguments {
    method: String,
    target: String,
    body: String,
    // headers: HashMap<String, String>,
}

pub(crate) const APP_NAME: &str = "tauri-fetch";
const UA: &str = "User-Agent";

static CLIENT: LazyLock<ureq::Agent> = LazyLock::new(|| ureq::AgentBuilder::new().build());

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch, restore])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn fetch(args: Arguments) -> String {
    unwrap(request(args))
}

#[inline(always)]
fn unwrap<T, E>(result: Result<T, E>) -> String
where
    T: Into<String>,
    E: Display,
{
    match result {
        Ok(ok) => ok.into(),
        Err(err) => err.to_string(),
    }
}

fn request(args: Arguments) -> Result<String> {
    store(&args.method, &args.target, &args.body);
    let request = CLIENT.request(&args.method, &args.target).set(UA, APP_NAME);

    // for (key, value) in &args.headers {
    //     request = request.set(key, value);
    // }

    let res = if args.body.is_empty() {
        request.call()
    } else {
        request.send_string(&args.body)
    };

    res?.into_string().map_err(From::from)
}

const PATH: &'static str = "temp";

fn store(method: &str, target: &str, body: &str) {
    #[derive(Serialize)]
    struct ArgumentsRef<'a> {
        method: &'a str,
        target: &'a str,
        body: &'a str,
        // headers: HashMap<String, String>,
    }

    let data = ArgumentsRef {
        method,
        target,
        body,
    };
    let _ = write_json(&data, PATH);
}

#[tauri::command]
fn restore() -> Arguments {
    read_json(PATH).unwrap_or_default()
}
