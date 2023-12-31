//
// Created by coolk on 15-07-2023.
//

#ifndef REST_CLIENT_WRAPPERS_H
#define REST_CLIENT_WRAPPERS_H
#include <string>
#include <vector>
#include "client.hpp"

class WrapperException : std::exception {
public:
    WrapperException(const std::string& msg) : msg(msg) {};
    static auto last_error() -> WrapperException;
    auto what() const throw() -> const char* {
        return msg.c_str();
    }
private:
    std::string msg;
};

class Response {
public:
    Response(ffi::Resp* raw) : raw(raw) {}
    ~Response();
    auto read_body() -> std::vector<char>;
private:
    ffi::Resp* raw;
};

class Request {
public:
    Request(const std::string&);
    ~Request();
    auto send() -> Response;
private:
    ffi::Request* raw;
};

#endif //REST_CLIENT_WRAPPERS_H


