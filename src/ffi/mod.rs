#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod inner {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use self::inner::root::*;
pub use self::inner::root::std;
