//
// Created by coolk on 15-07-2023.
//
#include "wrappers.h"

extern "C" {
    void* request_create(const char*);
    void request_destroy(void*);
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
