#[cfg(not(test))]
#[link(wasm_import_module = "response")]
extern "C" {
    pub fn status() -> u32;
    pub fn set_status(status: u32);
    pub fn write_body(buffer: *const u8, length: u32) -> u32;
}

#[cfg(test)]
extern "C" {
    pub fn status() -> u32;
    pub fn set_status(status: u32);
    pub fn write_body(buffer: *const u8, length: u32) -> u32;
}
