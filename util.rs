use ll::stylesheet::css_fixed;
use types::CssQName;
use core::libc::c_void;
use lwcstr_from_rust_str = wapcaplet::from_rust_string;

pub fn css_fixed_to_float(f: css_fixed) -> float {
    const before: i32 = 10;
    f as float * 1.0f / ((1i32 << before) as float)
}

pub fn float_to_css_fixed(f: float) -> css_fixed {
    const before: i32 = 10;
    (f * ((1 << before) as float)) as css_fixed
}

pub fn rust_str_to_net_qname(s: &str) -> CssQName {
    CssQName {
        ns: None,
        name: lwcstr_from_rust_str(s)
    }
}

pub fn net_qname_to_rust_str(qname: &a/CssQName) -> &a/str {
    qname.name.to_str_slice()
}


// FIXME: These methods should be unsafe
pub trait VoidPtrLike {
    static fn from_void_ptr(*c_void) -> Self;
    fn to_void_ptr(&self) -> *c_void;
}
