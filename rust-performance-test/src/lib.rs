pub mod json_parser;
use std::collections::HashMap;
use rand::Rng;

#[derive(Debug)]
pub struct HttpRequest {
    _method: String,
    _path: String,
    _headers: HashMap<String, String>,
    _body: String,
}

#[inline]
pub fn parse_http_request(request: &str) -> HttpRequest {
    let mut headers = HashMap::new();

    // Locate the end of the request line
    let newline_pos = request.find('\n').unwrap_or(request.len());
    let mut request_line = &request[..newline_pos];
    // Trim a trailing '\r', if present
    if request_line.ends_with('\r') {
        request_line = &request_line[..request_line.len() - 1];
    }

    // Manually extract method and path from the request line.
    let method_end = request_line.find(' ').unwrap_or(request_line.len());
    let method = &request_line[..method_end];

    let path_start = method_end + 1;
    let path_end = request_line[path_start..]
        .find(' ')
        .map(|i| path_start + i)
        .unwrap_or(request_line.len());
    let path = &request_line[path_start..path_end];

    // Move position past the request line (including newline)
    let mut pos = newline_pos + 1;
    let len = request.len();

    // Parse headers: iterate until an empty line is encountered.
    while pos < len {
        let line_end = request[pos..].find('\n').map(|i| pos + i).unwrap_or(len);
        let mut line = &request[pos..line_end];
        // Remove trailing '\r' if it exists
        if line.ends_with('\r') {
            line = &line[..line.len() - 1];
        }
        pos = line_end + 1; // advance to next line

        // An empty line signals the end of headers.
        if line.is_empty() {
            break;
        }

        // Look for the header delimiter ": "
        if let Some(colon) = line.find(": ") {
            let key = &line[..colon];
            let value = &line[colon + 2..];
            headers.insert(key.to_string(), value.to_string());
        }
    }

    // The remainder of the request is the body.
    let body = &request[pos..];

    HttpRequest {
        _method: method.to_string(),
        _path: path.to_string(),
        _headers: headers,
        _body: body.to_string(),
    }
}

#[inline]
pub fn basic_sort(n: i64) {
    let mut data: Vec<i64> = (0..n).map(|_| rand::rng().random_range(0..1_000_000)).collect();
    data.sort();
}

#[inline]
pub fn closure_operation(n: usize) {
    let mut data: Vec<i32> = vec![1; n];
    let closure = |x| x + 1;
    
    for val in &mut data {
        *val = closure(*val);
    }
}

#[inline]
pub fn memory_allocation_and_management(n: usize) {
    let mut arr: Box<[i32]> = vec![0; n].into_boxed_slice();
    
    for i in 0..n {
        arr[i] = i as i32;
    }
}

#[inline]
pub fn iterators(n: usize) {
    let mut data: Vec<i32> = vec![1; n];
    
    data.iter_mut().for_each(|x| *x += 1);
}