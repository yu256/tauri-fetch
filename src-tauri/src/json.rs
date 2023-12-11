use crate::APP_NAME;
use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
    sync::LazyLock,
};
use tauri::api::path::cache_dir;

static PATH: LazyLock<PathBuf> = LazyLock::new(|| cache_dir().unwrap().join(APP_NAME));

pub(crate) fn read_json<T>(path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut file = BufReader::new(File::open(PATH.join(path))?);

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    serde_json::from_str(&content).map_err(From::from)
}

pub(crate) fn write_json<T>(data: &T, name: &str) -> Result<()>
where
    T: Serialize,
{
    let Ok(file) = File::create(PATH.join(name)) else {
        create_dir_all(&*PATH)?;
        return write_json(data, name);
    };

    let json = serde_json::to_string(data)?;

    let mut file = BufWriter::new(file);

    file.write_all(json.as_bytes()).map_err(From::from)
}
