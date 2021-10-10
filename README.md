# serde-validation-with-try-from

Validate types and fields in serde with `TryFrom` trait and [`#[serde(try_from = "FromType")]`](https://serde.rs/container-attrs.html#try_from).

See [`src/validate_field.rs`](src/validate_field.rs) and [`src/validate_struct.rs`](src/validate_struct.rs) for the full code.

Use `cargo test` to test it out.
