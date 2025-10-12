#[derive(Debug)]
pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
}

impl StatusCode {
    fn code(&self) -> u16 {
        match self {
            StatusCode::Ok => 200,
            StatusCode::NotFound => 404,
        }
    }

    fn reason_phrase(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::NotFound => "Not Found",
        }
    }

    pub fn as_str(&self) -> String {
        format!("{} {}", self.code(), self.reason_phrase())
    }
}
