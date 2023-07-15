//
// Created by coolk on 15-07-2023.
//

#ifndef REST_CLIENT_WRAPPERS_H
#define REST_CLIENT_WRAPPERS_H
#include <string>
#include <vector>

class Response {
public:
    Response(void* raw) : raw(raw) {}
    ~Response();
    auto read_body() -> std::vector<char>;
private:
    void* raw;
};

class Request {
public:
    Request(const std::string&);
    ~Request();
    auto send() -> Response;
private:
    void* raw;
};

#endif //REST_CLIENT_WRAPPERS_H


