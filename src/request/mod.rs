use std::io::Read;

mod host;

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Request {
    cursor: u32
}

impl Read for Request {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes = unsafe { host::read_body(buf.as_mut_ptr(), buf.len() as u32, self.cursor) };

        self.cursor += bytes;

        Ok(bytes as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_request() {
        let mut buffer = vec![0; 64];
        let mut request = Request::default();

        let bytes = request.read(buffer.as_mut_slice()).unwrap();
        let expected = Vec::from("Hello, World!");

        assert_eq!(expected.len(), bytes);
        assert_eq!(&buffer[..bytes], &expected[..]);

    }
}