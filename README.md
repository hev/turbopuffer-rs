# turbopuffer-rs

Rust client for the [Turbopuffer](https://turbopuffer.com/) vector database. Forked from [ragkit/turbopuffer-client](https://github.com/ragkit/turbopuffer-client) with per-vector delete support and typed request structs.

## What's different from upstream

- **`delete_vectors(ids)`** - delete individual vectors by ID (POSTs to the write endpoint with a `deletes` array)
- **Typed request structs** - `UpsertRequest`, `QueryRequest`, `DeleteVectorsRequest` with corresponding `_typed` methods
- **Custom base URL** - `Client::with_base_url()` for pointing at proxies or non-default endpoints
- **Renamed `delete()` to `delete_namespace()`** - the old name was ambiguous now that per-vector delete exists

## Install

```toml
[dependencies]
turbopuffer-rs = { git = "https://github.com/hev/turbopuffer-rs" }
```

## Usage

```rust
use turbopuffer_rs::Client;

let client = Client::new("your-api-key");
let ns = client.namespace("my-namespace");
```

### Upsert

```rust
use serde_json::json;

let body = json!({
    "ids": [1, 2, 3],
    "vectors": [[0.1, 0.2], [0.3, 0.4], [0.5, 0.6]],
    "attributes": {
        "title": ["one", "two", "three"]
    }
});

let res = ns.upsert(&body).await?;
```

Or with the typed API:

```rust
use turbopuffer_rs::request::UpsertRequest;
use serde_json::json;
use std::collections::HashMap;

let req = UpsertRequest {
    ids: vec![json!(1), json!(2), json!(3)],
    vectors: vec![vec![0.1, 0.2], vec![0.3, 0.4], vec![0.5, 0.6]],
    attributes: Some(HashMap::from([
        ("title".into(), vec![json!("one"), json!("two"), json!("three")]),
    ])),
};

let res = ns.upsert_typed(&req).await?;
```

### Query

```rust
let query = json!({
    "vector": [0.1, 0.2],
    "distance_metric": "cosine_distance",
    "top_k": 5,
    "include_attributes": ["title"],
});

let res = ns.query(&query).await?;
for v in &res.vectors {
    println!("{}: dist={}", v.id, v.dist);
}
```

### Delete vectors by ID

```rust
let res = ns.delete_vectors(&[json!("doc-1"), json!("doc-2")]).await?;
```

### Delete namespace

```rust
let res = ns.delete_namespace().await?;
```

## License

MIT
