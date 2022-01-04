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

- `service` as `trait` and `struct`
- `message` as `struct`
- `package` as `mod`
- `syntax`

### Scalar

- `int32` as `i32`
- `bool` as `bool`
- `string` as `String`
