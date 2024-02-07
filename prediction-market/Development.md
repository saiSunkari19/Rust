

### To convert normal rust project to cosmwasm project 

- Create bin folder in /src directory with schema.rs file 
- run `cargo run --bin schema` 
- To build wasm locally `cargo build --release --lib --target wasm32-unknown-unknown`
    - this will produce a wasm build in ./target/wasm32-unknown-unknown/release/YOUR_NAME_HERE.wasm
    - `find . -name '*.wasm'` to get build
- To unit-test `cargo test --lib`
- To schema `cargo run --bin schema`
- Optimised build 

produce an extremely small build output in a consistent manner. The suggest way
to run it is this:

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.15.0
```

Or, If you're on an arm64 machine, you should use a docker image built with arm64.

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer-arm64:0.15.0

---
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
   cosmwasm/rust-optimizer-arm64:0.15.0
```
#### Add this lib in order to genererate wasm 
```rust 
[lib]
crate-type = ["cdylib", "rlib"]
```

## Delpoyment on SEI Testnet Atlantic-2
1. Store contract
```
seid tx wasm store artifacts/prediction_market-aarch64.wasm  --from=sei-testnet --chain-id=atlantic-2   --broadcast-mode=block  --node  https://sei-testnet-rpc.polkachu.com  --gas 10000000 --fees 1000000usei

- code id : 5848
```
2. Instaintiate contract 
```
seid tx wasm instantiate 5848 '{"price_feed_id":"53614f1cb0c031d4af66c04cb9c756234adad0e1cee85303795091499a4084eb","pyth_contract_addr":"sei1w2rxq6eckak47s25crxlhmq96fzjwdtjgdwavn56ggc0qvxvw7rqczxyfy"}' --label "fetch_price" --admin sei1kn2cp4n0cfg9063ny7my3qznte8ea07qh3qqcs --from=sei-testnet --chain-id=atlantic-2   --broadcast-mode=block  --node  https://sei-testnet-rpc.polkachu.com --fees 20000usei

```
3. Query prices 
```
seid q wasm    contract-state  smart  sei14kmf4y4dyrmtf7wyaywgy9vuxlfvc8nk82ytcvsc8gf3v3tcp6yqzrmzcr '{"fetch_price":{}}' --chain-id=atlantic-2   --node  https://sei-testnet-rpc.polkachu.com

```