using System;
using Xunit;
using foo;

namespace foo.Tests
{
    public class StringTests
    {
        [Fact]
        public void EnglishTest()
        {
            using(var stringclass = new StringClass())
            {
                const string ENGLISH_SENTENCE_1 = "I like to be home with my monkey and my dog";
                Assert.Equal(ENGLISH_SENTENCE_1, stringclass.Echo(ENGLISH_SENTENCE_1));

                const string ENGLISH_SENTENCE_2 = "Don't care, shut up, play the record!";
                Assert.Equal(ENGLISH_SENTENCE_2, stringclass.Echo(ENGLISH_SENTENCE_2));
            }
        }

        [Fact]
        public void FrenchTest()
        {
            using (var stringclass = new StringClass())
            {
                const string FRENCH_SENTENCE_1 = "Devant mon miroir j'ai rêvé d'être une star, j'ai rêvé d'être immortellement belle";
                Assert.Equal(FRENCH_SENTENCE_1, stringclass.Echo(FRENCH_SENTENCE_1));

                const string FRENCH_SENTENCE_2 = "Ce soir j'irai voir à travers le miroir, si la vie est éternelle";
                Assert.Equal(FRENCH_SENTENCE_2, stringclass.Echo(FRENCH_SENTENCE_2));
            }
        }

        [Fact]
        public void MemoryLeakTtest()
        {
            const int NUM_ITERATIONS = 100000;

            using (var stringclass = new StringClass())
            {
                for (int i = 0; i < NUM_ITERATIONS; i++)
                {
                    const string FRENCH_SENTENCE_1 = "Devant mon miroir j'ai rêvé d'être une star, j'ai rêvé d'être immortellement belle";
                    Assert.Equal(FRENCH_SENTENCE_1, stringclass.Echo(FRENCH_SENTENCE_1));
                }
            }
        }
    }
}
