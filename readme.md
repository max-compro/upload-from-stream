# Example of streaming files

This contains three different examples. The code for each example is located in
`src/bin/` directory and the mutual code in `src/request.rs`. 

There is precompiled binaries which can be run on Linux systems with x86_64 
architecture and uses glibc. The precompiled binaries can found in `bin/`.

## Installation

The example is written in Rust so to compile the example you have to install
Rust and its toolchain, this can be done from (here)[https://www.rust-lang.org/tools/install].

## Usage

After installation update the `.env` file with the url to use and your own api 
token. Then you can run each example by running the following command in
the terminal: `cargo run --bin [name of binary] -- --w [id] -f [id] [path]`.

For more help run: `cargo run --bin [name of binary] -- --help`.

## Configuration

All configurable values (2) are located in `.env`

```.env
MEDIABANK_URL="change this to your mediabank url"
API_KEY="change to your api key"
```
