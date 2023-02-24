#[cfg(not(test))]
#[link(wasm_import_module = "request")]
extern "C" {
    pub fn read_body(buffer: *mut u8, length: u32, start: u32) -> u32;
}

#[cfg(test)]
#[no_mangle]
pub unsafe extern "C" fn read_body(buffer: *mut u8, length: u32, start: u32) -> u32 {
    let start = start as usize;
    let body = Vec::from("Hello, World!");
    let remaining = &body[start..];
    let count = length.min(remaining.len() as u32);

    remaining.as_ptr().copy_to(buffer, count as usize);

    count
}
