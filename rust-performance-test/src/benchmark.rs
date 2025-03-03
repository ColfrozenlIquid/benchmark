use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_performance_tets::{basic_sort, closure_operation, iterators, json_parser::{serialize_json, JsonParser, JsonValue}, memory_allocation_and_management, parse_http_request, parse_http_request_optimized, worker_pool_processing};

fn sorting_benchmark(c: &mut Criterion) {
    c.bench_function("Sorting algorithm", |b| b.iter(|| basic_sort(black_box(20))));
}

fn closure_benchmark(c: &mut Criterion) {
    c.bench_function("Closure operation", |b| b.iter(|| closure_operation(black_box(20))));
}

fn memory_management_benchmark(c: &mut Criterion) {
    c.bench_function("Memory management operation", |b| b.iter(|| memory_allocation_and_management(black_box(20))));
}

fn iterators_benchmark(c: &mut Criterion) {
    c.bench_function("Iterators", |b| b.iter(|| iterators(black_box(20))));
}

fn worker_pool_processing_benchmark(c: &mut Criterion) {
    c.bench_function("Worker pool processing", |b| b.iter(|| worker_pool_processing()));
}

fn benchmark_parse_http_request(c: &mut Criterion) {
    let raw_request = "POST /submit HTTP/1.1\n\
                       Host: example.com\n\
                       Content-Length: 13\n\
                       Content-Type: text/plain\n\
                       \n\
                       Hello, world!";

    c.bench_function("Http Request", |b| {
        b.iter(|| parse_http_request(black_box(raw_request)))
    });
}

fn benchmark_parse_http_request_optimized(c: &mut Criterion) {
    let raw_request = "POST /submit HTTP/1.1\n\
                       Host: example.com\n\
                       Content-Length: 13\n\
                       Content-Type: text/plain\n\
                       \n\
                       Hello, world!";

    c.bench_function("Http Request optimized", |b| {
        b.iter(|| parse_http_request_optimized(black_box(raw_request)))
    });
}

fn bench_json_parsing(c: &mut Criterion) {
    let json_input = r#"{
        "name": "Test",
        "counter": 1,
        "users": [
            {"id": 1, "name": "Alice", "email": "alice@example.com", "scores": [100, 90, 95]},
            {"id": 2, "name": "Bob", "email": "bob@example.com", "scores": [80, 85, 88]},
            {"id": 3, "name": "Charlie", "email": "charlie@example.com", "scores": [90, 92, 87]},
            {"id": 4, "name": "David", "email": "david@example.com", "scores": [70, 75, 80]},
            {"id": 5, "name": "Eve", "email": "eve@example.com", "scores": [88, 90, 92]}
        ],
        "metadata": {
            "page": 1,
            "per_page": 5,
            "total": 50,
            "timestamp": "2025-02-19T12:34:56Z"
        },
        "nested": {
            "value": 42,
            "description": "This is a nested object with more data",
            "more_data": {
                "flag": true,
                "status": "active",
                "data": [1,2,3,4,5,6,7,8,9,10]
            }
        },
        "list": [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],
        "tags": ["rust", "json", "benchmark", "testing", "performance", "parsing"],
        "comments": [
            {"user": "Alice", "comment": "Great tool!", "likes": 10},
            {"user": "Bob", "comment": "Needs more work.", "likes": 5},
            {"user": "Charlie", "comment": "I love it!", "likes": 8},
            {"user": "David", "comment": "Could be improved.", "likes": 3},
            {"user": "Eve", "comment": "Fantastic performance.", "likes": 12}
        ]
    }"#;

    c.bench_function("json parsing", |b| {
        b.iter(|| {
            let mut parser = JsonParser::new(black_box(json_input));
            let mut root = parser.parse().expect("Failed to parse JSON");
            // If the root is an object, increment the "counter" field.
            if let JsonValue::Object(ref mut obj) = root {
                if let Some(JsonValue::Number(ref mut counter)) = obj.get_mut("counter") {
                    *counter += 1.0;
                }
            }
            let output = serialize_json(&root);
            black_box(output);
        })
    });
}

criterion_group!(benches, sorting_benchmark, closure_benchmark, memory_management_benchmark, iterators_benchmark, benchmark_parse_http_request, benchmark_parse_http_request_optimized, bench_json_parsing, worker_pool_processing_benchmark);
criterion_main!(benches);