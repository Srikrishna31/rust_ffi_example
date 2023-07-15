//
// Created by coolk on 15-07-2023.
//

#ifndef REST_CLIENT_WRAPPERS_H
#define REST_CLIENT_WRAPPERS_H
#include <string>

class Request {
public:
    Request(const std::string&);
    ~Request();

private:
    void* raw;
};
#endif //REST_CLIENT_WRAPPERS_H
