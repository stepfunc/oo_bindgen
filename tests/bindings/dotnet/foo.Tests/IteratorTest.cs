using System;
using Xunit;
using foo;
using System.Linq;
using System.Collections.Generic;

namespace foo.Tests
{
    public class IteratorTest
    {
        [Fact]
        public void StringIteratorTest()
        {
            List<Byte> values = new List<Byte>();

            IteratorTestHelper.InvokeCallback("ABCDE", strings => { 
                foreach(StringIteratorItem item in strings) {
                    values.Add(item.Value);
                }
            });
            Assert.Equal(new byte[] { 65, 66, 67, 68, 69 }, values);
        }

        [Fact]
        public void ChunkIteratorTest()
        {
            List<List<Char>> values = new List<List<Char>>();
            DoubleIteratorTestHelper.IterateStringByChunks("Hello World!", 3, chunks => {
                foreach (Chunk c in chunks)
                {
                    List<Char> bytes = new List<Char>();
                    foreach (ByteValue bv in c.Iter)
                    {
                        bytes.Add(Convert.ToChar(bv.Value));
                    }
                    values.Add(bytes);
                }
            });
            Assert.Equal(4, values.Count);
            Assert.Equal(values[0], new char[] { 'H', 'e', 'l' });
            Assert.Equal(values[1], new char[] { 'l', 'o', ' ' });
            Assert.Equal(values[2], new char[] { 'W', 'o', 'r' });
            Assert.Equal(values[3], new char[] { 'l', 'd', '!' });
        }
    }
}
