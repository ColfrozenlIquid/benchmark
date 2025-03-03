#include <iostream>
#include <string>
#include <string_view>
#include <unordered_map>
#include <vector>
#include <variant>
#include <stdexcept>
#include <cstdlib>
#include <cstring>
#include <cctype>

using JsonObject = std::unordered_map<std::string, struct JsonValue>;
using JsonArray  = std::vector<struct JsonValue>;

struct JsonValue {
    std::variant<std::nullptr_t, bool, double, std::string_view, JsonArray, JsonObject> value;

    JsonValue() : value(nullptr) {}
    JsonValue(std::nullptr_t n) : value(n) {}
    JsonValue(bool b) : value(b) {}
    JsonValue(double d) : value(d) {}
    JsonValue(std::string_view s) : value(s) {}
    JsonValue(const JsonArray &a) : value(a) {}
    JsonValue(const JsonObject &o) : value(o) {}
};

class JsonParser {
public:
    JsonParser(const char* s) : p(s) {}

    JsonValue parse() {
        skipWhitespace();
        JsonValue val = parseValue();
        skipWhitespace();
        if (*p != '\0')
            throw std::runtime_error("Extra characters after JSON value");
        return val;
    }

private:
    const char* p;

    void skipWhitespace() {
        while (*p && std::isspace(static_cast<unsigned char>(*p)))
            p++;
    }

    JsonValue parseValue() {
        skipWhitespace();
        if (*p == '{')
            return parseObject();
        else if (*p == '[')
            return parseArray();
        else if (*p == '"')
            return parseString();
        else if (*p == '-' || std::isdigit(static_cast<unsigned char>(*p)))
            return parseNumber();
        else if (std::strncmp(p, "true", 4) == 0) {
            p += 4;
            return JsonValue(true);
        } else if (std::strncmp(p, "false", 5) == 0) {
            p += 5;
            return JsonValue(false);
        } else if (std::strncmp(p, "null", 4) == 0) {
            p += 4;
            return JsonValue(nullptr);
        }
        throw std::runtime_error("Unexpected token while parsing value");
    }

    JsonValue parseObject() {
        JsonObject obj;
        p++;
        skipWhitespace();
        if (*p == '}') {
            p++;
            return JsonValue(obj);
        }
        while (true) {
            skipWhitespace();
            if (*p != '"')
                throw std::runtime_error("Expected string for object key");
            std::string_view key = std::get<std::string_view>(parseString().value);
            skipWhitespace();
            if (*p != ':')
                throw std::runtime_error("Expected ':' after object key");
            p++;
            skipWhitespace();
            JsonValue val = parseValue();
            obj.emplace(std::string(key), val);
            skipWhitespace();
            if (*p == '}') {
                p++;
                break;
            }
            if (*p != ',')
                throw std::runtime_error("Expected ',' between object members");
            p++;
        }
        return JsonValue(obj);
    }

    JsonValue parseArray() {
        JsonArray arr;
        p++;
        skipWhitespace();
        if (*p == ']') {
            p++;
            return JsonValue(arr);
        }
        while (true) {
            skipWhitespace();
            arr.push_back(parseValue());
            skipWhitespace();
            if (*p == ']') {
                p++;
                break;
            }
            if (*p != ',')
                throw std::runtime_error("Expected ',' between array elements");
            p++;
        }
        return JsonValue(arr);
    }

    JsonValue parseString() {
        p++;
        const char* start = p;
        while (*p && *p != '"') {
            if (*p == '\\') {
                p++;
                if (!*p)
                    throw std::runtime_error("Unexpected end in escape sequence");
            }
            p++;
        }
        if (*p != '"')
            throw std::runtime_error("Unterminated string");
        std::string_view s(start, p - start);
        p++;
        return JsonValue(s);
    }

    JsonValue parseNumber() {
        const char* start = p;
        if (*p == '-') p++;
        while (std::isdigit(static_cast<unsigned char>(*p))) p++;
        if (*p == '.') {
            p++;
            while (std::isdigit(static_cast<unsigned char>(*p))) p++;
        }
        double num = std::strtod(start, nullptr);
        return JsonValue(num);
    }
};

std::string serializeJson(const JsonValue &j);

struct JsonSerializer {
    std::string operator()(std::nullptr_t) const { return "null"; }
    std::string operator()(bool b) const { return b ? "true" : "false"; }
    std::string operator()(double d) const { return std::to_string(d); }
    std::string operator()(std::string_view s) const {
        std::string out = "\"";
        out.append(s);
        out.push_back('"');
        return out;
    }
    std::string operator()(const JsonArray &arr) const {
        std::string out = "[";
        bool first = true;
        for (const auto &elem : arr) {
            if (!first)
                out.push_back(',');
            first = false;
            out += serializeJson(elem);
        }
        out.push_back(']');
        return out;
    }
    std::string operator()(const JsonObject &obj) const {
        std::string out = "{";
        bool first = true;
        for (const auto &pair : obj) {
            if (!first)
                out.push_back(',');
            first = false;
            out += "\"" + pair.first + "\":";
            out += serializeJson(pair.second);
        }
        out.push_back('}');
        return out;
    }
};

std::string serializeJson(const JsonValue &j) {
    return std::visit(JsonSerializer(), j.value);
}