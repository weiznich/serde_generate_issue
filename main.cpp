#include <iostream>
#include "include/testing.hpp"

using namespace std;

extern "C" size_t test_out(unsigned char** out_ptr);
extern "C" void test_in(const unsigned char* data, size_t len);

int main(int argc, char *argv[]) {


    uint8_t* out_ptr1 = nullptr;
    size_t len1 = test_out(&out_ptr1);
    test_in(out_ptr1, len1);
    std::cout << "Just in and out works fine" << std::endl;

    uint8_t* out_ptr = nullptr;
    size_t len = test_out(&out_ptr);

    std::vector<uint8_t> serialized_result(out_ptr, out_ptr + len);
    auto res = testing::MyStruct::bincodeDeserialize(serialized_result);

    auto serializer = serde::BincodeSerializer();
    serde::Serializable<testing::MyStruct>::serialize(res, serializer);
    std::vector<uint8_t> bytes = std::move(serializer).bytes();

    test_in(bytes.data(), bytes.size());

    std::cout << "Everything is fine" << std::endl;

    return 0;
}
