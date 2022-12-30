# sigkit-rs

## How to use

```rust
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("<path to .sig file>").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let decompressed = sigkit::decompress_signature_library(&buf).unwrap();
    let signature_library = sigkit::flatbuffers::root_as_signature_library(&decompressed).unwrap();
    println!("{:#?}", signature_library);
}

```
