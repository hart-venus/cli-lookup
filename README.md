# cli-lookup
A simple command line tool to simplify single-prompt lookups using the OpenAI API.

## Dependencies
- [Rust](https://www.rust-lang.org/tools/install)
- [OpenAI API Key](https://platform.openai.com/)

## Installation
- clone this repo
```bash
git clone git@github.com:hart-venus/cli-lookup.git
```
- compile the binary
```bash
cd cli-lookup
cargo build --release
```
- export your OpenAI API key
```bash
EXPORT OPENAI_API_KEY=<your-api-key>
```
Replace `<your-api-key>` with your actual API key. 

## Usage
```bash
./target/release/cli-lookup "What's the weather like in San Francisco?"
```
