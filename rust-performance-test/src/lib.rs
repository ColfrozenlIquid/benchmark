pub mod json_parser;
use std::{collections::{HashMap, VecDeque}, sync::{Arc, Condvar, Mutex}, thread};
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
pub fn parse_http_request_optimized(request: &str) -> HttpRequest {
    let mut headers = HashMap::new();

    let newline_pos = request.find('\n').unwrap_or(request.len());
    let mut request_line = &request[..newline_pos];
    if request_line.ends_with('\r') {
        request_line = &request_line[..request_line.len() - 1];
    }

    let method_end = request_line.find(' ').unwrap_or(request_line.len());
    let method = &request_line[..method_end];

    let path_start = method_end + 1;
    let path_end = request_line[path_start..]
        .find(' ')
        .map(|i| path_start + i)
        .unwrap_or(request_line.len());
    let path = &request_line[path_start..path_end];

    let mut pos = newline_pos + 1;
    let len = request.len();

    while pos < len {
        let line_end = request[pos..].find('\n').map(|i| pos + i).unwrap_or(len);
        let mut line = &request[pos..line_end];
        if line.ends_with('\r') {
            line = &line[..line.len() - 1];
        }
        pos = line_end + 1;

        if line.is_empty() {
            break;
        }

        if let Some(colon) = line.find(": ") {
            let key = &line[..colon];
            let value = &line[colon + 2..];
            headers.insert(key.to_string(), value.to_string());
        }
    }

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

const NUM_WORKERS: usize = 4;
const NUM_TASKS: usize = 250000;

fn process_task(n: usize) -> usize {
    n * n
}

struct TaskQueue {
    queue: VecDeque<usize>,
    done: bool,
}

pub fn worker_pool_processing() {
    let queue = Arc::new((Mutex::new(TaskQueue { queue: VecDeque::new(), done: false }), Condvar::new()));
    let results = Arc::new(Mutex::new(Vec::with_capacity(NUM_TASKS)));
    let mut handles = Vec::new();
    
    for _ in 0..NUM_WORKERS {
        let queue = Arc::clone(&queue);
        let results = Arc::clone(&results);
        handles.push(thread::spawn(move || {
            let mut local_results = Vec::new(); // Thread-local storage
            let process_task = |n: usize| -> usize { n * n };
            loop {
                let task = {
                    let (lock, cond) = &*queue;
                    let mut queue = lock.lock().unwrap();
                    while queue.queue.is_empty() && !queue.done {
                        queue = cond.wait(queue).unwrap();
                    }
                    if queue.queue.is_empty() && queue.done {
                        break;
                    }
                    queue.queue.pop_front()
                };
                
                if let Some(task) = task {
                    local_results.push(process_task(task));
                }
            }
            
            // Merge local results into global results
            let mut global_results = results.lock().unwrap();
            global_results.extend(local_results);
        }));
    }
    
    {
        let (lock, cond) = &*queue;
        let mut queue = lock.lock().unwrap();
        for i in 0..NUM_TASKS {
            queue.queue.push_back(i);
        }
        cond.notify_all();
    }
    
    {
        let (lock, cond) = &*queue;
        let mut queue = lock.lock().unwrap();
        queue.done = true;
        cond.notify_all();
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}