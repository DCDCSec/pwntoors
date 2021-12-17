pub mod util {
    use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

    // macro_rules! pack {
    //     ($n:expr) => {
    //         inner_pack!(
    //             concat_ident!(p, $n),
    //             concat_ident!(u, $n),
    //             concat_ident!(write_u, $n)
    //         );
    //     };
    // }

    macro_rules! pack {
        ($func_name:ident, $ty:ty, $inner_method:ident) => {
            pub fn $func_name(v: $ty) -> Result<Vec<u8>, std::io::Error> {
                let mut res = Vec::<u8>::new();
                res.$inner_method::<BigEndian>(v.into())?;
                Ok(res)
            }
        };
    }

    pub fn p8(v: u8) -> Result<Vec<u8>, std::io::Error> {
        let mut res = Vec::<u8>::new();
        res.write_u8(v)?;
        Ok(res)
    }

    pack!(p16, u16, write_u16);
    pack!(p32, u32, write_u32);
    pack!(p64, u64, write_u64);
}

#[test]
fn test_u8() {
    let r = util::p8(0xFF);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), vec![0xff]);
}

#[test]
fn test_u16() {
    let r = util::p16(0xFF16);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), vec![0xff, 0x16]);
}

#[test]
fn test_u32() {
    let r = util::p32(0xFF1682);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), vec![0x00, 0xff, 0x16, 0x82]);
}

#[test]
fn test_u64() {
    let r = util::p64(0xFF168219_20304050);
    assert!(r.is_ok());
    assert_eq!(
        r.unwrap(),
        vec![0xff, 0x16, 0x82, 0x19, 0x20, 0x30, 0x40, 0x50]
    );
}
