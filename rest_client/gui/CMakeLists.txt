set(CMAKE_CXX_STANDARD 17)
set(ENV{Qt6_DIR} "D:\\Qt\\6.5.1\\msvc2019_64")
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
list(APPEND CMAKE_PREFIX_PATH $ENV{QT_DIR})
#list(APPEND CMAKE_PREFIX_PATH "D:\\Qt\\6.5.1\\mingw_64\\lib\\cmake")
# list(APPEND CMAKE_PREFIX_PATH "/cygdrive/d/Qt/6.5.1")
find_package(Qt6Widgets)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(BUILD_SUFFIX "d")
else ()
    set(BUILD_SUFFIX "")
endif()

set(SOURCE main.cpp)
add_executable(gui ${SOURCE})
target_compile_features(gui PRIVATE cxx_std_17)
target_compile_options(gui PRIVATE "/Zc:__cplusplus")
target_compile_options(gui PRIVATE "/permissive-")
target_compile_options(gui PRIVATE "/Zc:wchar_t")
get_target_property(CLIENT_DIR client LOCATION)
target_link_libraries(gui PUBLIC Qt6::Widgets)
# target_link_libraries(gui PUBLIC ${CLIENT_DIR}/client${CMAKE_SHARED_LIBRARY_SUFFIX})
add_dependencies(gui client)

# Copy the dependent dlls into the final executable directory so that the code runs.
add_custom_command(TARGET gui POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy "$ENV{QT_DIR}\\bin\\Qt6Widgets${BUILD_SUFFIX}${CMAKE_SHARED_LIBRARY_SUFFIX}" ${CMAKE_CURRENT_BINARY_DIR}
        COMMAND ${CMAKE_COMMAND} -E copy "$ENV{QT_DIR}\\bin\\Qt6Core${BUILD_SUFFIX}${CMAKE_SHARED_LIBRARY_SUFFIX}" ${CMAKE_CURRENT_BINARY_DIR}
        COMMAND ${CMAKE_COMMAND} -E copy "$ENV{QT_DIR}\\bin\\Qt6Gui${BUILD_SUFFIX}${CMAKE_SHARED_LIBRARY_SUFFIX}" ${CMAKE_CURRENT_BINARY_DIR}
        )
