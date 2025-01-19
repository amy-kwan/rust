//@ check-pass
//@ revisions: aix
//@[aix] compile-flags: --target powerpc64-ibm-aix
//@[aix] needs-llvm-components: powerpc

#![warn(uses_power_alignment)]

#[repr(C)]
pub struct Floats {
    a: f64,
    b: u8,
    c: f64, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
    d: f32,
}

pub struct Floats2 {
    a: f64,
    b: u32,
    c: f64,
}

#[repr(C)]
pub struct Floats3 {
    a: f32,
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
pub struct Floats5 {
    a: f32,
    b: f64, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
    c: f32,
}

#[repr(C)]
pub struct FloatAgg1 {
    x: Floats,
    y: f64, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
}

#[repr(C)]
pub struct FloatAgg2 {
    x: i64,
    y: Floats, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
}

#[repr(C)]
pub struct FloatAgg3 {
    x: FloatAgg1,
    y: FloatAgg2, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
    z: FloatAgg2, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
}

#[repr(C)]
pub struct FloatAgg4 {
    x: FloatAgg1,
    y: FloatAgg2, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
}

#[repr(C)]
pub struct FloatAgg5 {
    x: FloatAgg1,
    y: FloatAgg2, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
    z: FloatAgg3, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
}

#[repr(C)]
pub struct FloatAgg6 {
    x: i64,
    y: Floats, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
    z: u8,
}

#[repr(C)]
pub struct FloatAgg7 {
    x: i64,
    y: Floats, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
    z: u8,
    zz: f32,
}

#[repr(C)]
pub struct A {
  d: f64,
}
#[repr(C)]
pub struct B {
  a: A,
  f: f32,
  d: f64, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
}
#[repr(C)]
pub struct C {
  c: u8,
  b: B, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
}
#[repr(C)]
pub struct D {
  x: f64,
}
#[repr(C)]
pub struct E {
  x: i32,
  d: D, //~ WARNING repr(C) does not follow the power alignment rule. This may affect platform C ABI compatibility for this type
}

fn main() { }
