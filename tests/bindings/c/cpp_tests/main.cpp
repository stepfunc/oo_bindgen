void version_tests();
void constant_tests();
void defaulted_interface_tests();
void enum_tests();
void error_tests();
void duration_tests();
void string_tests();
void structure_tests();
void callback_tests();
void iterator_tests();
void primitive_iterator_tests();
void universal_tests();
void collection_tests();
void thread_tests();

int main()
{
    version_tests();
    constant_tests();
    defaulted_interface_tests();
    enum_tests();
    error_tests();
    iterator_tests();
    primitive_iterator_tests();
    duration_tests();
    string_tests();
    structure_tests();
    callback_tests();
    universal_tests();
    collection_tests();
    thread_tests();

    return 0;
}