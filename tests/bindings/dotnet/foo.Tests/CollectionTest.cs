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

            Assert.Equal((uint)strings.Count, StringCollection.GetSize(strings));
            Assert.Equal("Hello", StringCollection.GetValue(strings, 0));
            Assert.Equal("World!", StringCollection.GetValue(strings, 1));
            Assert.Equal("Émile", StringCollection.GetValue(strings, 2));
        }
    }
}
