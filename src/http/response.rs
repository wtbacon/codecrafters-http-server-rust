use crate::http::status::StatusCode;

const DELIMITERS: &str = "\r\n";

pub fn build_response(
    status: StatusCode,
    headers: Option<Vec<(String, String)>>,
    body: Option<String>,
) -> String {
    let body = body.unwrap_or_default();
    let content_length = body.len();

    let mut response = format!("HTTP/1.1 {}{}", status.as_str(), DELIMITERS);

    if let Some(hdrs) = headers {
        for (key, value) in &hdrs {
            response.push_str(&format!("{}: {}{}", key, value, DELIMITERS));
        }
    }

    response.push_str(&format!("Content-Length: {}{}", content_length, DELIMITERS));
    response.push_str(&format!("Content-Type: text/plain{}", DELIMITERS));

    response.push_str(DELIMITERS);

    response.push_str(&body);

    response
}
