set(CMAKE_CXX_STANDARD 17)
# This should be set appropriately for Linux and Mac as well.
set(ENV{Qt6_DIR} "D:\\Qt\\6.5.1\\msvc2019_64")
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
list(APPEND CMAKE_PREFIX_PATH $ENV{Qt6_DIR})
find_package(Qt6Widgets)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(BUILD_SUFFIX "d")
else ()
    set(BUILD_SUFFIX "")
endif()

set(SOURCE main_window.hpp main.cpp main_window.cpp main_window.hpp)
add_executable(gui ${SOURCE})
get_target_property(CLIENT_DIR client LOCATION)

target_compile_features(gui PRIVATE cxx_std_17)
if (MSVC)
    # For windows, link against the stub library generated from client target.
    target_link_libraries(gui PRIVATE ${CLIENT_DIR}/client${CMAKE_SHARED_LIBRARY_SUFFIX}${CMAKE_STATIC_LIBRARY_SUFFIX})
    # Qt requires the below options to build for Windows.
    target_compile_options(gui PRIVATE "/Zc:__cplusplus")
    target_compile_options(gui PRIVATE "/permissive-")
    target_compile_options(gui PRIVATE "/Zc:wchar_t")
else()
    # For Linux/Mac, link against the .so/.dylib file directly
    target_link_libraries(gui PRIVATE ${CLIENT_DIR}/client${CMAKE_STATIC_LIBRARY_SUFFIX})
endif()


target_link_libraries(gui PUBLIC Qt6::Widgets)
add_dependencies(gui client)

# Copy the dependent dlls into the final executable directory so that the code runs.
add_custom_command(TARGET gui POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy "$ENV{Qt6_DIR}\\bin\\Qt6Widgets${BUILD_SUFFIX}${CMAKE_SHARED_LIBRARY_SUFFIX}" ${CMAKE_CURRENT_BINARY_DIR}
        COMMAND ${CMAKE_COMMAND} -E copy "$ENV{Qt6_DIR}\\bin\\Qt6Core${BUILD_SUFFIX}${CMAKE_SHARED_LIBRARY_SUFFIX}" ${CMAKE_CURRENT_BINARY_DIR}
        COMMAND ${CMAKE_COMMAND} -E copy "$ENV{Qt6_DIR}\\bin\\Qt6Gui${BUILD_SUFFIX}${CMAKE_SHARED_LIBRARY_SUFFIX}" ${CMAKE_CURRENT_BINARY_DIR}
        COMMAND ${CMAKE_COMMAND} -E copy "${CLIENT_DIR}/client${CMAKE_SHARED_LIBRARY_SUFFIX}" ${CMAKE_CURRENT_BINARY_DIR}
        )
