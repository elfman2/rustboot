#![no_std]
#![no_main]

use rt::entry;

entry!(main);
/// Struct that handles error codes
#[repr(C)]
pub enum CodecResponse {
    Ok = 0,
    EncodeError = -1,
    DecodeError = -2,
}

#[link(name = "a653")]
extern {
    fn GET_TIME() -> i64;
}

fn main() -> ! {
    let mut  _x = 42;
    let mut time;
    loop {_x+=1;
       time = unsafe {GET_TIME();}
	}
}

