use std::{collections::HashMap, fs, path};

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

pub fn files_handler(req: &Request, params: HashMap<String, String>) -> Response {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 && args[1] != "--directory" {
        eprint!("Server must be started with --directory <dir>");
        eprint!("{:?}", args);
        return Response::new(Parts::new(StatusCode::NOT_FOUND, req.head.version), None);
    }
    let dir = &args[2];

    let filename = params.get("filename").unwrap_or(&"".to_string()).clone();
    if filename
        .split(path::MAIN_SEPARATOR)
        .any(|part| part == "..")
    {
        eprint!("Invalid file path");
        return Response::new(Parts::new(StatusCode::NOT_FOUND, req.head.version), None);
    }

    let full_path = format!("{}/{}", dir, filename);
    let file_contents = match fs::read_to_string(full_path) {
        Ok(contents) => contents,
        Err(_) => {
            eprint!("Failed to read file");
            return Response::new(Parts::new(StatusCode::NOT_FOUND, req.head.version), None);
        }
    };

    let mut head = Parts::new(StatusCode::OK, req.head.version);

    head.headers.insert(
        "Content-Length".to_string(),
        file_contents.len().to_string(),
    );
    head.headers.insert(
        "Content-Type".to_string(),
        "application/octet-stream".to_string(),
    );

    Response::new(head, Some(file_contents))
}
