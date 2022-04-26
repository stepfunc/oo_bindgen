using System;
using Xunit;
using foo;
using System.Linq;
using System.Collections.Generic;

namespace foo.Tests
{
    public class CollectionTest
    {
        [Fact]
        public void StringCollectionTest()
        {
            var strings = new List<string>();
            strings.Add("Hello");
            strings.Add("World!");
            strings.Add("Émile");

            Assert.Equal((uint)strings.Count, StringCollectionTestMethods.GetSize(strings));
            Assert.Equal("Hello", StringCollectionTestMethods.GetValue(strings, 0));
            Assert.Equal("World!", StringCollectionTestMethods.GetValue(strings, 1));
            Assert.Equal("Émile", StringCollectionTestMethods.GetValue(strings, 2));
        }

        [Fact]
        public void StringCollectionWithReserveTest()
        {
            var strings = new List<string>();
            strings.Add("Hello");
            strings.Add("World!");
            strings.Add("Émile");

            Assert.Equal((uint)strings.Count, StringCollectionTestMethods.GetSizeWithReserve(strings));
            Assert.Equal("Hello", StringCollectionTestMethods.GetValueWithReserve(strings, 0));
            Assert.Equal("World!", StringCollectionTestMethods.GetValueWithReserve(strings, 1));
            Assert.Equal("Émile", StringCollectionTestMethods.GetValueWithReserve(strings, 2));
        }
    }
}
