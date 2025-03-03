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
    let mut lines = request.lines();
    let mut headers = HashMap::new();
    let mut body = String::new();

    let request_line = lines.next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let (method, path) = (parts.get(0).unwrap_or(&""), parts.get(1).unwrap_or(&""));

    for line in &mut lines {
        if line.is_empty() { break; }
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    body = lines.collect::<Vec<&str>>().join("\n");

    HttpRequest {
        _method: method.to_string(),
        _path: path.to_string(),
        _headers: headers,
        _body: body,
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