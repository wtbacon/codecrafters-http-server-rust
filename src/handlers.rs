use std::collections::HashMap;

use crate::http::{
    request::Request,
    response::{Parts, Response},
    status::StatusCode,
};

pub fn root_handler(req: &Request, _params: HashMap<String, String>) -> Response {
    Response::new(Parts::new(StatusCode::OK, req.head.version), None)
}

pub fn echo_handler(req: &Request, params: HashMap<String, String>) -> Response {
    let mut head = Parts::new(StatusCode::OK, req.head.version);

    let echo_part = params.get("msg").unwrap_or(&"".to_string()).clone();
    let content_length = echo_part.len();
    head.headers
        .insert("Content-Length".to_string(), content_length.to_string());
    head.headers
        .insert("Content-Type".to_string(), "text/plain".to_string());

    Response::new(head, Some(echo_part))
}

pub fn user_agent_handler(req: &Request, _params: HashMap<String, String>) -> Response {
    let mut head = Parts::new(StatusCode::OK, req.head.version);

    let user_agent = req
        .head
        .headers
        .get("User-Agent")
        .unwrap_or(&"".to_string())
        .clone();
    let content_length = user_agent.len();
    head.headers
        .insert("Content-Length".to_string(), content_length.to_string());
    head.headers
        .insert("Content-Type".to_string(), "text/plain".to_string());

    Response::new(head, Some(user_agent))
}
