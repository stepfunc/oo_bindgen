#include <cassert>
#include <cstring>

#include "foo.hpp"

#define ENGLISH_STRING_1 "I like to be home with my monkey and my dog"
#define ENGLISH_STRING_2 "Don't care, shut up, play the record!"

#define FRENCH_STRING_1 "Devant mon miroir j'ai rêvé d'être une star, j'ai rêvé d'être immortellement belle"
#define FRENCH_STRING_2 "Ce soir j'irai voir à travers le miroir, si la vie est éternelle"

static void test_english_string()
{
    foo::StringClass string_class;

    assert(string_class.echo(ENGLISH_STRING_1) == ENGLISH_STRING_1);
    assert(strlen(ENGLISH_STRING_1) == string_class.get_length(ENGLISH_STRING_1));

    assert(string_class.echo(ENGLISH_STRING_2) == ENGLISH_STRING_2);
    assert(strlen(ENGLISH_STRING_2) == string_class.get_length(ENGLISH_STRING_2));    
}

static void test_french_string()
{
    foo::StringClass string_class;

    assert(string_class.echo(FRENCH_STRING_1) == FRENCH_STRING_1);
    assert(strlen(FRENCH_STRING_1) == string_class.get_length(FRENCH_STRING_1));

    assert(string_class.echo(FRENCH_STRING_2) == FRENCH_STRING_2);
    assert(strlen(FRENCH_STRING_2) == string_class.get_length(FRENCH_STRING_2));
}

void string_tests()
{
    test_english_string();
    test_french_string();
}
