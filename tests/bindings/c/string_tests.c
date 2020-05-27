#include <assert.h>
#include <string.h>
#include <stdio.h>

#include "foo.h"

static void test_english_string()
{
    StringClass* string_class = string_new();

    #define ENGLISH_STRING_1 "I like to be home with my monkey and my dog"
    printf(string_echo(string_class, ENGLISH_STRING_1));
    assert(strcmp(ENGLISH_STRING_1, string_echo(string_class, ENGLISH_STRING_1)) == 0);
    assert(strlen(ENGLISH_STRING_1) == string_length(ENGLISH_STRING_1));

    #define ENGLISH_STRING_2 "Don't care, shut up, play the record!"
    assert(strcmp(ENGLISH_STRING_2, string_echo(string_class, ENGLISH_STRING_2)) == 0);
    assert(strlen(ENGLISH_STRING_2) == string_length(ENGLISH_STRING_2));

    string_destroy(string_class);
}

static void test_french_string()
{
    StringClass* string_class = string_new();

    #define FRENCH_STRING_1 "Devant mon miroir j'ai rêvé d'être une star, j'ai rêvé d'être immortellement belle"
    printf(string_echo(string_class, FRENCH_STRING_1));
    assert(strcmp(FRENCH_STRING_1, string_echo(string_class, FRENCH_STRING_1)) == 0);
    assert(strlen(FRENCH_STRING_1) == string_length(FRENCH_STRING_1));

    #define FRENCH_STRING_2 "Ce soir j'irai voir à travers le miroir, si la vie est éternelle"
    assert(strcmp(FRENCH_STRING_2, string_echo(string_class, FRENCH_STRING_2)) == 0);
    assert(strlen(FRENCH_STRING_2) == string_length(FRENCH_STRING_2));

    string_destroy(string_class);
}

void string_tests()
{
    test_english_string();
    test_french_string();
}
