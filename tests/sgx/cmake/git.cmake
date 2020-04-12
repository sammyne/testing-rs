cmake_minimum_required(VERSION 3.10)

ExternalProject_Add(teaclave-sgx-sdk
    GIT_REPOSITORY https://github.com/apache/incubator-teaclave-sgx-sdk
    GIT_TAG v1.1.1
    GIT_PROGRESS true
    SOURCE_DIR ${PROJECT_SOURCE_DIR}/third_party/teaclave-sgx-sdk
    UPDATE_DISCONNECTED true
    CONFIGURE_COMMAND echo "skip configure for teaclave-sgx-sdk"
    BUILD_COMMAND echo "skip build for teaclave-sgx-sdk"
    INSTALL_COMMAND echo "skip install for teaclave-sgx-sdk"
)