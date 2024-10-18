use protobuf::Message;
use std::io::Read;
use std::io::Write;

use crate::base::Library;

const LIBRARY_BIN: &str = "library.bin";

pub fn load_library() -> Library {
    let mut file = std::fs::File::open(LIBRARY_BIN).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    Library::parse_from_bytes(&buffer).unwrap()
}

pub fn save_library(library: &Library) {
    let mut file = std::fs::File::create(LIBRARY_BIN).unwrap();
    let bytes = library.write_to_bytes().unwrap();
    file.write_all(&bytes).unwrap();
}
