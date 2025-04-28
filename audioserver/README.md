# Audioserver

## Prerequisites

```
sudo apt install libasound2-dev
```

## Run

```
cargo run -p audioserver
```

## LMDB Example

```
# in Cargo.toml
[dependencies]
lmdb-zero = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

```
use lmdb_zero as lmdb;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Params {
    temperature: f64,
    mode: String,
}

const DB_NAME: &str = "params";

fn open_env<P: AsRef<Path>>(path: P) -> lmdb::Result<lmdb::Env> {
    fs::create_dir_all(&path).unwrap();
    let mut builder = lmdb::EnvBuilder::new()?;
    builder.set_max_dbs(1)?;
    builder.open(path.as_ref(), lmdb::open::Flags::empty(), 0o600)
}

fn save_params(env: &lmdb::Env, params: &Params) -> lmdb::Result<()> {
    let db = env.open_db(Some(DB_NAME), lmdb::db::CREATE)?;
    let txn = lmdb::WriteTransaction::new(env)?;
    {
        let access = txn.access();
        let serialized = serde_json::to_vec(params).unwrap();
        access.put(&db, "params", &serialized, lmdb::put::Flags::empty())?;
    }
    txn.commit()
}

fn load_params(env: &lmdb::Env) -> lmdb::Result<Option<Params>> {
    let db = env.open_db(Some(DB_NAME), lmdb::db::CREATE)?;
    let txn = lmdb::ReadTransaction::new(env)?;
    let access = txn.access();
    if let Ok(bytes) = access.get::<str, [u8]>(&db, "params") {
        let params: Params = serde_json::from_slice(bytes).unwrap();
        Ok(Some(params))
    } else {
        Ok(None)
    }
}

fn main() {
    let env = open_env("./lmdb_data").expect("Failed to open LMDB environment");

    // Engine: write parameters
    let params = Params {
        temperature: 42.5,
        mode: "auto".to_string(),
    };
    save_params(&env, &params).expect("Failed to save params");

    // Web UI: read parameters
    if let Ok(Some(loaded)) = load_params(&env) {
        println!("Loaded params: {:?}", loaded);
    }
}
```