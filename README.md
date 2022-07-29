
# Tauri Yew Template

This is essentially a yew and tauri template but with a folder structure i prefer more.

## Installation

Have npm and rust installed and then install tauri-cli and cargo-commander:
```bash
cargo install cargo-commander
cargo install tauri-cli --locked --version ^1.0.0-rc 
```

Now run the setup script:
```bash
cargo cmd setup
```
This installs trunk and wasm-32 and installs node dependencies
## Run Locally

Follow the installation guide and then simply run:
```bash
cargo cmd dev
```
Your tauri app should launch in dev mode.
## Build

To build your app run:

```bash
cargo cmd build
```


## Stuff you should change 
```
root > cargo.toml
root > crates > [
    backend > [
        Cargo.toml, tauri.conf.json
    ],
    frontend > [
        index.html,
        src > main.rs
    ]
]
```
## Acknowledgements

 - [Original Template (how-do-you)](https://github.com/how-do-you/banan)
 - [Yew Tauri Guide](https://dev.to/stevepryde/create-a-desktop-app-in-rust-using-tauri-and-yew-2bhe)

## License

[MIT](https://choosealicense.com/licenses/mit/)
