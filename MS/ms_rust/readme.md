```shell
# install rust and cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# create project folder
cargo init ms
cd ms/
# compile and run
cp ../rust_files/ms.rs src/main.rs
cp ../rust_files/Cargo.toml Cargo.toml
cargo build --release
time ./target/release/ms 100000000 4
```
