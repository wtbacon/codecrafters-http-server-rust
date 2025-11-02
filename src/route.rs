use std::{collections::HashMap, path};

use crate::http::{
    request::Request,
    response::{Parts, Response},
    status::StatusCode,
};

type Handler = fn(&Request, HashMap<String, String>) -> Response;

#[derive(Default)]
pub struct Router {
    routes: HashMap<(String, String), Handler>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, method: &str, path_pattern: &str, handler: Handler) {
        self.routes
            .insert((method.to_string(), path_pattern.to_string()), handler);
    }

    pub fn route(&self, request: &Request) -> Response {
        for ((method, path_pattern), handler) in &self.routes {
            if &request.head.method == method {
                if let Some(params) =
                    Self::match_and_extract_params(&request.head.path, path_pattern)
                {
                    return handler(request, params);
                }
            }
        }
        Self::not_found(request)
    }

    fn match_and_extract_params(path: &str, pattern: &str) -> Option<HashMap<String, String>> {
        let mut params = HashMap::new();
        let path_parts = path
            .trim_matches(path::MAIN_SEPARATOR)
            .split(path::MAIN_SEPARATOR)
            .collect::<Vec<&str>>();
        let pattern_parts = pattern
            .trim_matches(path::MAIN_SEPARATOR)
            .split(path::MAIN_SEPARATOR)
            .collect::<Vec<&str>>();

        if path_parts.len() != pattern_parts.len() {
            return None;
        }

        for (p_part, pat_part) in path_parts.iter().zip(pattern_parts.iter()) {
            if pat_part.starts_with(":") {
                let key = pat_part.trim_start_matches(":").to_string();
                params.insert(key, p_part.to_string());
            } else if p_part != pat_part {
                return None;
            }
        }
        Some(params)
    }

    fn not_found(request: &Request) -> Response {
        Response::new(
            Parts::new(StatusCode::NOT_FOUND, request.head.version),
            Some(StatusCode::NOT_FOUND.canonical_reason().into()),
        )
    }
}
