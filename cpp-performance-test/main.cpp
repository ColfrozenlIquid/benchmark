#include <benchmark/benchmark.h>
#include <vector>
#include <algorithm>
#include <random>
#include <functional>
#include <iostream>
#include <string>
#include <sstream>
#include <unordered_map>
#include <thread>
#include <mutex>
#include <condition_variable>
#include "json_parser.hpp"
#include <queue>

struct HttpRequest {
    std::string method;
    std::string path;
    std::unordered_map<std::string, std::string> headers;
    std::string body;
};

// Function to generate and sort a vector
inline static void BasicSort(benchmark::State& state) {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<int64_t> dist(0, 1'000'000);

    for (auto _ : state) {
        std::vector<int64_t> data(state.range(0));
        std::generate(data.begin(), data.end(), [&]() { return dist(gen); });
        std::sort(data.begin(), data.end());
        benchmark::DoNotOptimize(data);
    }
}

inline void closure_operation(benchmark::State& state, size_t n) {
    for (auto _ : state) {
        std::vector<int> data(n, 1);
        auto closure = [](int x) { return x + 1; };
        for (auto& val : data) {
            val = closure(val);
        }
        benchmark::DoNotOptimize(data);
    }
}

inline void memory_allocation_and_management(benchmark::State& state, size_t n) {
    for (auto _ : state) {
        std::unique_ptr<int[]> arr(new int[n]);
        for (size_t i = 0; i < n; ++i) {
            arr[i] = static_cast<int>(i);
        }
        benchmark::DoNotOptimize(arr);
    }
}

inline void iterators(benchmark::State& state, size_t n) {
    for (auto _ : state) {
        std::vector<int> data(n, 1);
        std::for_each(data.begin(), data.end(), [](int& x) { x += 1; });
        benchmark::DoNotOptimize(data);
    }
}

inline HttpRequest parseHttpRequest(const std::string& request) {
    std::istringstream stream(request);
    HttpRequest req;
    std::string line;

    if (std::getline(stream, line)) {
        std::istringstream lineStream(line);
        lineStream >> req.method >> req.path;
    }

    while (std::getline(stream, line) && line != "\r") {
        std::istringstream headerStream(line);
        std::string key, value;
        if (std::getline(headerStream, key, ':')) {
            std::getline(headerStream >> std::ws, value);
            req.headers[key] = value;
        }
    }

    req.body.assign(std::istreambuf_iterator<char>(stream), std::istreambuf_iterator<char>());

    return req;
}

inline HttpRequest parseHttpRequestOptimized(const std::string& request) {
    HttpRequest req;
    std::string_view sv(request);

    size_t pos = sv.find('\n');
    std::string_view request_line = (pos == std::string_view::npos) ? sv : sv.substr(0, pos);
    if (!request_line.empty() && request_line.back() == '\r')
        request_line.remove_suffix(1);

    size_t method_end = request_line.find(' ');
    if (method_end != std::string_view::npos) {
        req.method = std::string(request_line.substr(0, method_end));
        size_t path_start = method_end + 1;
        size_t path_end = request_line.find(' ', path_start);
        if (path_end == std::string_view::npos)
            req.path = std::string(request_line.substr(path_start));
        else
            req.path = std::string(request_line.substr(path_start, path_end - path_start));
    }

    if (pos != std::string_view::npos)
        sv.remove_prefix(pos + 1);

    while (true) {
        size_t newline_pos = sv.find('\n');
        std::string_view line;
        if (newline_pos == std::string_view::npos) {
            line = sv;
            sv = std::string_view();
        } else {
            line = sv.substr(0, newline_pos);
            sv.remove_prefix(newline_pos + 1);
        }

        if (!line.empty() && line.back() == '\r')
            line.remove_suffix(1);

        if (line.empty())
            break;

        size_t colon = line.find(": ");
        if (colon != std::string_view::npos) {
            std::string key(line.substr(0, colon));
            std::string value(line.substr(colon + 2));
            req.headers.emplace(std::move(key), std::move(value));
        }
    }

    req.body = std::string(sv);
    return req;
}

inline static void BenchmarkParseHttpRequest(benchmark::State& state) {
    std::string rawRequest =
        "POST /submit HTTP/1.1\r\n"
        "Host: example.com\r\n"
        "Content-Length: 13\r\n"
        "Content-Type: text/plain\r\n"
        "\r\n"
        "Hello, world!";

    for (auto _ : state) {
        benchmark::DoNotOptimize(parseHttpRequest(rawRequest));
    }
}

inline static void BenchmarkParseHttpRequestOptimized(benchmark::State& state) {
    std::string rawRequest =
        "POST /submit HTTP/1.1\r\n"
        "Host: example.com\r\n"
        "Content-Length: 13\r\n"
        "Content-Type: text/plain\r\n"
        "\r\n"
        "Hello, world!";

    for (auto _ : state) {
        benchmark::DoNotOptimize(parseHttpRequestOptimized(rawRequest));
    }
}

// Benchmark function that runs the JSON parsing, modification, and serialization
inline static void BenchmarkParseJson(benchmark::State& state) {
    // Use the argument (e.g., 20) to run multiple iterations per benchmark loop.
    int num_runs = state.range(0);
    for (auto _ : state) {
        for (int i = 0; i < num_runs; i++) {
            const char* jsonInput = R"({
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
    })";

            try {
                JsonParser parser(jsonInput);
                JsonValue root = parser.parse();

                // Modify the "counter" field by incrementing it (if present)
                if (std::holds_alternative<JsonObject>(root.value)) {
                    JsonObject &obj = std::get<JsonObject>(root.value);
                    auto it = obj.find("counter");
                    if (it != obj.end() && std::holds_alternative<double>(it->second.value)) {
                        double counter = std::get<double>(it->second.value);
                        it->second = JsonValue(counter + 1);
                    }
                }

                std::string output = serializeJson(root);
                benchmark::DoNotOptimize(output);
            } catch (const std::exception &ex) {
                benchmark::DoNotOptimize(ex.what());
            }
        }
    }
}

const int NUM_WORKERS = 4;
const int NUM_TASKS = 250000;

inline int workerPoolProcessing() {
    std::queue<int> taskQueue;
    std::vector<int> results;
    std::mutex queueMutex, resultsMutex;
    std::condition_variable condition;
    bool done = false;

    auto worker = [&](int id) {
        while (true) {
            int task;
            {
                std::unique_lock<std::mutex> lock(queueMutex);
                condition.wait(lock, [&] { return !taskQueue.empty() || done; });

                if (done && taskQueue.empty())
                    break;

                task = taskQueue.front();
                taskQueue.pop();
            }

            int result = task * task;
            {
                std::lock_guard<std::mutex> lock(resultsMutex);
                results.push_back(result);
            }
        }
    };

    std::vector<std::thread> threads;
    for (int i = 0; i < NUM_WORKERS; ++i) {
        threads.emplace_back(worker, i);
    }

    {
        std::lock_guard<std::mutex> lock(queueMutex);
        for (int i = 0; i < NUM_TASKS; ++i) {
            taskQueue.push(i);
        }
    }
    condition.notify_all();

    {
        std::lock_guard<std::mutex> lock(queueMutex);
        done = true;
    }
    condition.notify_all();

    for (auto& t : threads) {
        t.join();
    }
    
    return 0;
}

inline static void BenchmarkWorkerPoolProcessing(benchmark::State& state) {
    for (auto _ : state) {
        workerPoolProcessing();
    }
}

BENCHMARK(BasicSort)->Arg(20); 
BENCHMARK_CAPTURE(closure_operation, test, 20);
BENCHMARK_CAPTURE(memory_allocation_and_management, test, 20);
BENCHMARK_CAPTURE(iterators, test, 20);
BENCHMARK(BenchmarkParseHttpRequest)->Arg(20);
BENCHMARK(BenchmarkParseHttpRequestOptimized)->Arg(20);
BENCHMARK(BenchmarkParseJson)->Arg(20);
BENCHMARK(BenchmarkWorkerPoolProcessing);
BENCHMARK_MAIN();
