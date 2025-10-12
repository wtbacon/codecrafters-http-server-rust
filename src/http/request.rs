use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn new(request_line: String, headers: Vec<String>, body: Option<String>) -> Self {
        let mut headers_map = HashMap::new();

        for header in headers {
            if let Some((key, value)) = header.split_once(": ") {
                headers_map.insert(key.to_string(), value.to_string());
            }
        }

        Self {
            request_line: RequestLine::new(request_line),
            headers: headers_map,
            body,
        }
    }
}

#[derive(Debug)]
pub struct RequestLine {
    pub method: String,
    pub path: String,
    pub version: String,
}

impl RequestLine {
    pub fn new(line: String) -> Self {
        let mut parts = line.splitn(3, ' ');
        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("").to_string();
        let version = parts.next().unwrap_or("").to_string();

        Self {
            method,
            path,
            version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_line_parsing() {
        let line = "GET /index.html HTTP/1.1".to_string();
        let request_line = RequestLine::new(line);

        assert_eq!(request_line.method, "GET");
        assert_eq!(request_line.path, "/index.html");
        assert_eq!(request_line.version, "HTTP/1.1");
    }

    #[test]
    fn test_request_line_parsing_with_query_params() {
        let line = "POST /api/users?limit=10&offset=20 HTTP/1.1".to_string();
        let request_line = RequestLine::new(line);

        assert_eq!(request_line.method, "POST");
        assert_eq!(request_line.path, "/api/users?limit=10&offset=20");
        assert_eq!(request_line.version, "HTTP/1.1");
    }

    #[test]
    fn test_request_line_parsing_incomplete() {
        let line = "GET /".to_string();
        let request_line = RequestLine::new(line);

        assert_eq!(request_line.method, "GET");
        assert_eq!(request_line.path, "/");
        assert_eq!(request_line.version, "");
    }

    #[test]
    fn test_request_line_parsing_empty() {
        let line = "".to_string();
        let request_line = RequestLine::new(line);

        assert_eq!(request_line.method, "");
        assert_eq!(request_line.path, "");
        assert_eq!(request_line.version, "");
    }

    #[test]
    fn test_request_creation() {
        let headers = vec![
            "Content-Type: application/json".to_string(),
            "Authorization: Bearer token123".to_string(),
            "User-Agent: TestClient/1.0".to_string(),
        ];

        let request = Request::new(
            "POST /api/users HTTP/1.1".to_string(),
            headers,
            Some(r#"{"name": "John"}"#.to_string()),
        );

        assert_eq!(request.request_line.method, "POST");
        assert_eq!(request.request_line.path, "/api/users");
        assert_eq!(request.request_line.version, "HTTP/1.1");

        assert_eq!(
            request.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(
            request.headers.get("Authorization"),
            Some(&"Bearer token123".to_string())
        );
        assert_eq!(
            request.headers.get("User-Agent"),
            Some(&"TestClient/1.0".to_string())
        );

        assert_eq!(request.body, Some(r#"{"name": "John"}"#.to_string()));
    }

    #[test]
    fn test_request_creation_no_body() {
        let headers = vec!["Accept: text/html".to_string()];

        let request = Request::new("GET / HTTP/1.1".to_string(), headers, None);

        assert_eq!(request.request_line.method, "GET");
        assert_eq!(request.request_line.path, "/");
        assert_eq!(
            request.headers.get("Accept"),
            Some(&"text/html".to_string())
        );
        assert_eq!(request.body, None);
    }

    #[test]
    fn test_request_headers_parsing_malformed() {
        let headers = vec![
            "Valid-Header: value".to_string(),
            "Invalid-Header-No-Colon".to_string(),
            "Another-Valid: another-value".to_string(),
        ];

        let request = Request::new("GET / HTTP/1.1".to_string(), headers, None);

        // Should only contain valid headers
        assert_eq!(request.headers.len(), 2);
        assert_eq!(
            request.headers.get("Valid-Header"),
            Some(&"value".to_string())
        );
        assert_eq!(
            request.headers.get("Another-Valid"),
            Some(&"another-value".to_string())
        );
        assert_eq!(request.headers.get("Invalid-Header-No-Colon"), None);
    }

    #[test]
    fn test_request_headers_with_colons_in_value() {
        let headers = vec![
            "Time: 12:34:56".to_string(),
            "URL: http://example.com:8080/path".to_string(),
        ];

        let request = Request::new("GET / HTTP/1.1".to_string(), headers, None);

        assert_eq!(request.headers.get("Time"), Some(&"12:34:56".to_string()));
        assert_eq!(
            request.headers.get("URL"),
            Some(&"http://example.com:8080/path".to_string())
        );
    }
}
