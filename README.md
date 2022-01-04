# Protobuf in Rust

### Running

```sh
cargo run assets/message.proto
```

## Input to output steps

- Read file input
- Tokenisation of syntax
- Translate tokenised syntax into Rust
- Output rendered Rust

## Available tokens

### Kind

- service
- message

### Config

- syntax
- package

### Scalar

- int32
- bool
- string
