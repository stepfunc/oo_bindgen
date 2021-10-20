using System;
using Xunit;
using foo;
using System.Linq;
using System.Collections.Generic;

namespace foo.Tests
{
    public class IteratorTest
    {
        class ValuesReceiver : IValuesReceiver
        {
            public readonly List<Byte> values = new List<Byte>();

            public void OnCharacters(ICollection<StringIteratorItem> values)
            {
                foreach(StringIteratorItem v in values)
                {
                    this.values.Add(v.Value);
                }
            }
        }

        class ChunkReceiver : IChunkReceiver
        {
            public readonly List<List<Char>> values = new List<List<Char>>();

            public void OnChunk(ICollection<Chunk> values)
            {
                foreach(Chunk c in values)
                {
                    List<Char> bytes = new List<Char>();
                    foreach(ByteValue bv in c.Iter) {
                        bytes.Add(Convert.ToChar(bv.Value));
                    }
                    this.values.Add(bytes);
                }
            }
        }

        [Fact]
        public void StringIteratorTest()
        {
            var receiver = new ValuesReceiver();
            
            IteratorTestHelper.InvokeCallback("ABCDE", receiver);
            Assert.Equal(new byte[] { 65, 66, 67, 68, 69 }, receiver.values);
        }

        [Fact]
        public void ChunkIteratorTest()
        {
            var receiver = new ChunkReceiver();
            DoubleIteratorTestHelper.IterateStringByChunks("Hello World!", 3, receiver);
            Assert.Equal(4, receiver.values.Count);
            Assert.Equal(receiver.values[0], new char[] { 'H', 'e', 'l' });
            Assert.Equal(receiver.values[1], new char[] { 'l', 'o', ' ' });
            Assert.Equal(receiver.values[2], new char[] { 'W', 'o', 'r' });
            Assert.Equal(receiver.values[3], new char[] { 'l', 'd', '!' });
        }

    }
}

