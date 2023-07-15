//
// Created by coolk on 15-07-2023.
//
#include "wrappers.h"

extern "C" {
    void* request_create(const char*);
    void request_destroy(void*);

    void response_destroy(void*);
    int response_body_length(void*);
    int response_body(void*, char*, int);

    void * request_send(void*);
}

Request::Request(const std::string& url) {
    raw = request_create(url.c_str());
    if (raw == nullptr) {
        throw "Invalid URL";
    }
}

Request::~Request() {
    request_destroy(raw);
}

auto Request::send() -> Response {
    void* raw_response = request_send(raw);

    if (raw_response == nullptr) {
        throw "Request failed";
    }

    return Response(raw_response);
};

Response::~Response() { response_destroy(raw); }

auto Response::read_body() -> std::vector<char>{
    auto length = response_body_length(raw);
    if (length <0) {
        throw "Response body's length was less than zero";
    }

    auto buffer= std::vector<char>(length);

    auto bytes_written = response_body(raw, buffer.data(), buffer.size());
    if (bytes_written != length) {
        throw "Response body was a different size than what we expected";
    }

    return buffer;
}
