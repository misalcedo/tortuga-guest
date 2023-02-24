use std::io::Write;

mod host;
mod status;

pub use status::Status;

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Response {}

impl Response {
    pub fn status(&self) -> Status {
        let code = unsafe { host::status() };
        Status::from(code)
    }

    pub fn set_status(&self, status: Status) {
        unsafe { host::set_status(status as u32); }
    }
}

impl Write for Response {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let bytes = unsafe { host::write_body(buf.as_ptr(), buf.len() as u32) };

        Ok(bytes as usize)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
