#!/bin/bash

(
echo -e 'Code:\n```rust';
cat src/main.rs ;
echo -e '\n```\n\nResult of `cargo build`:\n```\n' ;
cargo build 2>&1 |cat;
echo -e '\n```'
 ) | gpt4io_fresh \
        'Act as Rust expert. Provide fixed code.' \
        'Here I provide `src/main.rs` and result of `cargo build`. Please provide fixed code'

