# Byte Fast Find

Faster byte pattern search using IDA style byte pattern matching

## Building

### git *(HEAD)*

```sh
git clone https://github.com/etke/bff && cd bff
cargo build --features=binja --release
cp target/release/libbff.so ~/.binaryninja/plugins/
```

## Usage

*tip* Use binaryninja keybindings to facilitate ease of use. eg. `Ctrl+Shift+F`