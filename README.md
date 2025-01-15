# Sula-Alus CLI

A CLI tool to encode strings and files using Sula-Alus encoding

## Installation

```bash
cargo install sula-alus
# or using binstall
cargo binstall sula-alus
```

## Usage

### Using on string

```bash
sula-alus "Some Text" # output: "txeT emoS"
```

### Using on file

```bash
sula-alus -f input.txt # output: <encoded file content>
```

### Outputting to a file

```bash
sula-alus -f input.txt -o output.txt
```

## License

This project is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
