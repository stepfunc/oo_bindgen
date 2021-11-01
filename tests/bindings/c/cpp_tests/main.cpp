void version_tests();
void constant_tests();
void enum_tests();
void error_tests();
void duration_tests();
void string_tests();
void structure_tests();
void callback_tests();
void iterator_tests();
void universal_tests();

int main()
{
    version_tests();
    constant_tests();
    enum_tests();
    error_tests();
    iterator_tests();
    duration_tests();
    callback_tests();
    universal_tests();

    return 0;
}