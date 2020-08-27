using System;
using System.Text;
using Xunit;
using foo;

namespace foo.Tests
{
    public class StringTest
    {
        const string ENGLISH_SENTENCE_1 = "I like to be home with my monkey and my dog";
        const string ENGLISH_SENTENCE_2 = "Don't care, shut up, play the record!";
        const string FRENCH_SENTENCE_1 = "Devant mon miroir j'ai rêvé d'être une star, j'ai rêvé d'être immortellement belle";
        const string FRENCH_SENTENCE_2 = "Ce soir j'irai voir à travers le miroir, si la vie est éternelle";

        [Fact]
        public void EnglishTest()
        {
            using(var stringclass = new StringClass())
            {
                Assert.Equal(ENGLISH_SENTENCE_1, stringclass.Echo(ENGLISH_SENTENCE_1));
                Assert.Equal(ENGLISH_SENTENCE_2, stringclass.Echo(ENGLISH_SENTENCE_2));
            }
        }

        [Fact]
        public void FrenchTest()
        {
            using (var stringclass = new StringClass())
            {
                Assert.Equal(FRENCH_SENTENCE_1, stringclass.Echo(FRENCH_SENTENCE_1));
                Assert.Equal(FRENCH_SENTENCE_2, stringclass.Echo(FRENCH_SENTENCE_2));
            }
        }

        [Fact]
        public void LengthTest()
        {
            Assert.Equal(Encoding.UTF8.GetByteCount(ENGLISH_SENTENCE_1), (int)StringClass.GetLength(ENGLISH_SENTENCE_1));
            Assert.Equal(Encoding.UTF8.GetByteCount(ENGLISH_SENTENCE_2), (int)StringClass.GetLength(ENGLISH_SENTENCE_2));
            // TODO: fix these with UTF8 support
            //Assert.Equal(Encoding.UTF8.GetByteCount(FRENCH_SENTENCE_1), (int)StringClass.GetLength(FRENCH_SENTENCE_1));
            //Assert.Equal(Encoding.UTF8.GetByteCount(FRENCH_SENTENCE_2), (int)StringClass.GetLength(FRENCH_SENTENCE_2));
        }

        [Fact]
        public void MemoryLeakTest()
        {
            const int NUM_ITERATIONS = 100000;

            using (var stringclass = new StringClass())
            {
                for (int i = 0; i < NUM_ITERATIONS; i++)
                {
                    Assert.Equal(FRENCH_SENTENCE_1, stringclass.Echo(FRENCH_SENTENCE_1));
                }
            }
        }
    }
}
