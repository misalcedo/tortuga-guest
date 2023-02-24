#[link(wasm_import_module = "response")]
extern "C" {
    pub fn status() -> u32;
    pub fn set_status(status: u32);
    pub fn write(buffer: *const u8, length: u32, end: u32) -> u32;
}
