/// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
#[repr(u16)]
pub enum Status {
    Continue = 100,
    #[default]
    Ok = 200,
    Created = 201,
    MultipleChoices = 300,
    BadRequest = 400,
    InternalServerError = 500,
}

impl From<u32> for Status {
    fn from(status: u32) -> Self {
        match status {
            100..=199 => Status::Continue,
            200..=299 => match status {
                200 => Status::Ok,
                201 => Status::Created,
                _ => Status::Ok,
            },
            300..=399 => Status::MultipleChoices,
            400..=499 => Status::BadRequest,
            500..=599 => Status::InternalServerError,
            _ => Status::Ok,
        }
    }
}