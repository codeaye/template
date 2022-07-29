
# Tauri React Template

This is essentially a react-ts and tauri template but with a folder structure i prefer more.

## Installation

Have npm and rust installed and then install tauri-cli and cargo-commander:
```bash
cargo install cargo-commander tauri-cli
```

Now run the setup script:
```bash
cargo cmd setup
```
This installs pnpm (your hardrive will thank me later) and installs node dependencies
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
        index.html, package.json
    ]
]
```
## Acknowledgements

 - [Original Template (how-do-you)](https://github.com/how-do-you/banan)


## License

[MIT](https://choosealicense.com/licenses/mit/)
