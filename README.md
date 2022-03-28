# rust_hello

- dfx deploy rust_hello

 

- dfx canister call rust_hello uploadData '(300:nat)' //Add 300MB of data


- dfx canister status rust_hello
```
Memory Size: Nat(664519607)

```

- dfx build rust_hello; dfx canister install rust_hello --mode upgrade


- dfx canister status rust_hello

``` 
Canister status call result for rust_hello.
Status: Running
Controllers: 3on3e-ygfl4-v7kf6-t5aci-fhsyd-y2myv-2p27h-mygdp-4avu2-priww-jqe renrk-eyaaa-aaaaa-aaada-cai
Memory allocation: 0
Compute allocation: 0
Freezing threshold: 2_592_000
Memory Size: Nat(4329489343)
Balance: 4_000_000_000_000 Cycles
Module hash: 0xb520703d9026af2c00351fba34242179f5a3ca72db1e0afb5c69eeec02d1463c
```

- dfx canister call rust_hello uploadData '(10:nat)'


- dfx build rust_hello; dfx canister install rust_hello --mode upgrade
```
Building canisters...
Executing: cargo build --target wasm32-unknown-unknown --release -p rust_hello
Compiling rust_hello v0.1.0 (/Users/ipfs/work/test_rust/tmp/rust_hello/src/rust_hello)
Finished release [optimized] target(s) in 3.32s
Executing: ic-cdk-optimizer -o target/wasm32-unknown-unknown/release/rust_hello.wasm target/wasm32-unknown-unknown/release/rust_hello.wasm
Upgrading code for canister rust_hello, with canister_id rdmx6-jaaaa-aaaaa-aaadq-cai
Error: The Replica returned an error: code 5, message: "Canister rdmx6-jaaaa-aaaaa-aaadq-cai trapped: unreachable"

```

