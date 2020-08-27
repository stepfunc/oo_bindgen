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
            var characters = StringIterator.IterateString("ABCDE");
            Assert.Equal(new byte[] { 65, 66, 67, 68, 69 }, characters.Select(val => val.Value));
        }
    }
}
