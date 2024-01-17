//! mod encode
// use super::define::*;

pub trait Encode: Sync + Send {
    fn encode(&self, buf: &mut Vec<u8>);
}

impl Encode for i32 {
    fn encode(&self, buf: &mut Vec<u8>) {
        let _ = itoa::write(buf, *self);
    }
}

impl Encode for &str {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(b'"');

        // todo encode json string
        buf.extend_from_slice(self.as_bytes());

        buf.push(b'"');
    }
}

impl Encode for String {
    fn encode(&self, buf: &mut Vec<u8>) {
        self.as_str().encode(buf)
    }
}