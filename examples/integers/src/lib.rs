#[no_mangle]
pub extern fn addition(a: u32, b: u32) -> u32 {
    a + b
}

#[allow(dead_code)]
pub extern fn fix_linking_when_not_using_stdlib() { panic!() }
