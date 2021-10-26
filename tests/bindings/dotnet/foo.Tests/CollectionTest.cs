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

            Assert.Equal((uint)strings.Count, StringCollectionMethods.GetSize(strings));
            Assert.Equal("Hello", StringCollectionMethods.GetValue(strings, 0));
            Assert.Equal("World!", StringCollectionMethods.GetValue(strings, 1));
            Assert.Equal("Émile", StringCollectionMethods.GetValue(strings, 2));
        }

        [Fact]
        public void StringCollectionWithReserveTest()
        {
            var strings = new List<string>();
            strings.Add("Hello");
            strings.Add("World!");
            strings.Add("Émile");

            Assert.Equal((uint)strings.Count, StringCollectionMethods.GetSizeWithReserve(strings));
            Assert.Equal("Hello", StringCollectionMethods.GetValueWithReserve(strings, 0));
            Assert.Equal("World!", StringCollectionMethods.GetValueWithReserve(strings, 1));
            Assert.Equal("Émile", StringCollectionMethods.GetValueWithReserve(strings, 2));
        }
    }
}
