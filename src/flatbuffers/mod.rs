mod generated;

use std::io::{Error, ErrorKind, Read};

use flate2::bufread::ZlibDecoder;
pub use generated::*;

const SIG_FORMAT_MAGIC: &[u8] = b"BNSG";
const SIG_FORMAT_VERSION: u8 = 1;

pub fn decompress_signature_library(buf: &[u8]) -> Result<Vec<u8>, Error> {
    if &buf[0..4] != SIG_FORMAT_MAGIC {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    if buf[4] != SIG_FORMAT_VERSION {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    let mut decoder = ZlibDecoder::new(&buf[5..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}
