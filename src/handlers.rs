use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::{self, Path},
};

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
    head.headers
        .insert("Content-Type".to_string(), "text/plain".to_string());

    let echo_part = params.get("msg").unwrap_or(&"".to_string()).clone();
    match req.head.headers.get("Accept-Encoding") {
        Some(encoding) if encoding.contains("gzip") => {
            let mut encoder =
                flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
            encoder.write_all(echo_part.as_bytes()).unwrap();
            let compressed_data = encoder.finish().unwrap();
            head.headers
                .insert("Content-Encoding".to_string(), "gzip".to_string());
            head.headers.insert(
                "Content-Length".to_string(),
                compressed_data.len().to_string(),
            );
            Response::new(head, Some(compressed_data))
        }
        _ => {
            let content_length = echo_part.len();
            head.headers
                .insert("Content-Length".to_string(), content_length.to_string());

            Response::new(head, Some(echo_part.into_bytes()))
        }
    }
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

    Response::new(head, Some(user_agent.into_bytes()))
}

pub fn files_handler(req: &Request, params: HashMap<String, String>) -> Response {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 && args[1] != "--directory" {
        eprint!("Server must be started with --directory <dir>");
        eprint!("{:?}", args);
        return Response::new(Parts::new(StatusCode::NOT_FOUND, req.head.version), None);
    }
    let base_dir = Path::new(&args[2]);

    let filename = params.get("filename").unwrap_or(&"".to_string()).clone();
    if filename
        .split(path::MAIN_SEPARATOR)
        .any(|part| part == "..")
    {
        eprint!("Invalid file path");
        return Response::new(Parts::new(StatusCode::NOT_FOUND, req.head.version), None);
    }

    let full_path = base_dir.join(&filename);
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

    Response::new(head, Some(file_contents.into_bytes()))
}

pub fn post_file_handler(req: &Request, params: HashMap<String, String>) -> Response {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 && args[1] != "--directory" {
        eprint!("Server must be started with --directory <dir>");
        eprint!("{:?}", args);
        return Response::new(Parts::new(StatusCode::NOT_FOUND, req.head.version), None);
    }
    let base_dir = Path::new(&args[2]);

    let filename = match params.get("filename") {
        Some(name) => name.clone(),
        None => {
            eprint!("Filename parameter missing");
            return Response::new(Parts::new(StatusCode::NOT_FOUND, req.head.version), None);
        }
    };

    let file_path = base_dir.join(&filename);
    let mut file = match File::create(&file_path) {
        Ok(f) => f,
        Err(_) => {
            eprint!("Failed to create file");
            return Response::new(
                Parts::new(StatusCode::INTERNAL_SERVER_ERROR, req.head.version),
                None,
            );
        }
    };

    let contents = req.body.as_ref();
    match file.write_all(contents.unwrap_or(&"".to_string()).as_bytes()) {
        Ok(_) => (),
        Err(_) => {
            eprint!("Failed to write to file");
            return Response::new(
                Parts::new(StatusCode::INTERNAL_SERVER_ERROR, req.head.version),
                None,
            );
        }
    }

    let head = Parts::new(StatusCode::CREATED, req.head.version);
    Response::new(head, None)
}
