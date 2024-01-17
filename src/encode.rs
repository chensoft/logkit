//! mod encode
pub trait Encode: Sync + Send { // todo should put in json library
fn encode(&self, buf: &mut Vec<u8>);
}

macro_rules! impl_encode_integer {
    ($t:ident) => {
        impl Encode for $t {
            fn encode(&self, buf: &mut Vec<u8>) {
                let _ = itoa::write(buf, *self); // todo can faster
            }
        }
    };
}

impl_encode_integer!(i8);
impl_encode_integer!(i16);
impl_encode_integer!(i32);
impl_encode_integer!(i64);
// todo impl_encode_integer!(i128);
impl_encode_integer!(isize);
impl_encode_integer!(u8);
impl_encode_integer!(u16);
impl_encode_integer!(u32);
impl_encode_integer!(u64);
// todo impl_encode_integer!(u128);
impl_encode_integer!(usize);

impl Encode for f32 { // todo pretty?
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.reserve(24);
        unsafe {
            let len = ryu::raw::f2s_buffered_n(*self, buf.as_mut_ptr().add(buf.len()));
            buf.set_len(buf.len() + len);
        }
    }
}

impl Encode for f64 { // todo pretty?
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.reserve(24);
        unsafe {
            let len = ryu::raw::d2s_buffered_n(*self, buf.as_mut_ptr().add(buf.len()));
            buf.set_len(buf.len() + len);
        }
    }
}

impl Encode for bool {
    fn encode(&self, buf: &mut Vec<u8>) {
        match *self {
            true => buf.extend_from_slice("true".as_bytes()), // todo make faster
            false => buf.extend_from_slice("false".as_bytes()),
        }
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

#[test]
fn test_simple() {
    fn assert(val: impl Encode, cmp: String) {
        let mut buf = vec![];
        val.encode(&mut buf);
        assert_eq!(String::from_utf8_lossy(&buf), cmp);
    }

    assert(0_i8, 0_i8.to_string());
    assert(i8::MIN, i8::MIN.to_string());
    assert(i8::MAX, i8::MAX.to_string());

    assert(0_i16, 0_i16.to_string());
    assert(i16::MIN, i16::MIN.to_string());
    assert(i16::MAX, i16::MAX.to_string());

    assert(0_i32, 0_i32.to_string());
    assert(i32::MIN, i32::MIN.to_string());
    assert(i32::MAX, i32::MAX.to_string());

    assert(0_i64, 0_i64.to_string());
    assert(i64::MIN, i64::MIN.to_string());
    assert(i64::MAX, i64::MAX.to_string());

    // todo
    // assert(0_i128, 0_i128.to_string());
    // assert(i128::MIN, i128::MIN.to_string());
    // assert(i128::MAX, i128::MAX.to_string());

    assert(0_isize, 0_isize.to_string());
    assert(isize::MIN, isize::MIN.to_string());
    assert(isize::MAX, isize::MAX.to_string());

    assert(u8::MIN, u8::MIN.to_string());
    assert(u8::MAX, u8::MAX.to_string());

    assert(u16::MIN, u16::MIN.to_string());
    assert(u16::MAX, u16::MAX.to_string());

    assert(u32::MIN, u32::MIN.to_string());
    assert(u32::MAX, u32::MAX.to_string());

    assert(u64::MIN, u64::MIN.to_string());
    assert(u64::MAX, u64::MAX.to_string());

    // assert(u128::MIN, u128::MIN.to_string());
    // assert(u128::MAX, u128::MAX.to_string());

    assert(usize::MIN, usize::MIN.to_string());
    assert(usize::MAX, usize::MAX.to_string());

    assert(true, true.to_string());
    assert(false, false.to_string());

    // todo add escape strings
    assert("", r#""""#.to_string());
    assert("Hello World", r#""Hello World""#.to_string());
}

#[test]
fn test_compose() {
    let mut buf = vec![];
    1_i32.encode(&mut buf);
    2_u32.encode(&mut buf);
    3_f32.encode(&mut buf);
    4_f64.encode(&mut buf);
    true.encode(&mut buf);
    "5".encode(&mut buf);
    assert_eq!(String::from_utf8_lossy(&buf), r#"123E04E0true"5""#);
}