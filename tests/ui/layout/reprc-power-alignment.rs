//@ check-pass
//@ revisions: aix
//@[aix] compile-flags: --target powerpc64-ibm-aix
//@[aix] needs-llvm-components: powerpc

#![warn(uses_power_alignment)]

#[repr(C)]
pub struct Floats {
    a: f64,  //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
    b: u8,
    c: f64,
}

pub struct Floats2 {
    a: f64,
    b: u32,
    c: f64,
}

#[repr(C)]
pub struct Floats3 {
    a: f32,  //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
    b: f32,
    c: i64,
}

#[repr(C)]
pub struct Floats4 {
    a: u64,
    b: u32,
    c: f32,
}

#[repr(C)]
pub struct FloatAgg1 {
    x: Floats, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
    y: f64,
}

#[repr(C)]
pub struct FloatAgg2 {
    x: i64,
    y: Floats,
}

#[repr(C)]
pub struct FloatAgg3 {
    x: FloatAgg1, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
    y: FloatAgg2,
}

fn main() { }
