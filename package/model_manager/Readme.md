# build the manager

```bash
cargo build --release
```

Copy binary

```bash
cp target/release/pyano-model-manager .
```

Run the manager

```bash
./pyano-model-manager
```

Manager server will be available at `127.0.0.1:8090`

To connect with server:

## In rust code:

```rust
use pyano::model::{ModelManagerClient};

let model_manager = ModelManagerClient::new("http://127.0.0.1:8090");

```

## Rest API usage

APIs available:
/models/load (POST)
/models/unload (POST)
/models/status/:name (GET)
/models/list (GET)

example usage:

```bash
curl  -X POST \
  'http://127.0.0.1:8090/models/load' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "name": "coder",
  "model_path": "/Users/cj/.pyano/models/base-coder.gguf",
  "model_type": "0",
  "adapter_config": {
    "batch_size": 512,
    "ctx_size": 8192,
    "gpu_layers": -1,
    "server_port": 5006,
    "extra_args": {}
  }
}'

```
