set(CMAKE_CXX_STANDARD 17)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMT cargo build --release)
    set(TARGET_DIR "release")
endif()

set(CLIENT_SO "${CMAKE_SOURCE_DIR}/client/target/${TARGET_DIR}/client${CMAKE_SHARED_LIBRARY_SUFFIX}")

# For windows, we have to decouple each of the commands into separate function calls.
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /Zc:__cpluscplus")

set(CARGO_TARGET_DIR ${CMAKE_CURRENT_BINARY_DIR})
add_custom_target(client ALL
    COMMENT "Compiling client module"
    COMMAND ${CARGO_CMD}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

MESSAGE("Current Binary Directory: ${CMAKE_CURRENT_BINARY_DIR}")

add_custom_command(TARGET client
        POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy ${CLIENT_SO} ${CMAKE_CURRENT_BINARY_DIR}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
        DEPENDS client
)

set_target_properties(client PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

add_test(NAME client_test
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
