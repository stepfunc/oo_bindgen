using System;
using Xunit;
using foo;
using System.Linq;
using System.Collections.Generic;

namespace foo.Tests
{
    public class PrimitiveIteratorTest
    {
        [Fact]
        public void StringIteratorTest()
        {
            var values = new List<uint>();

            foo.RangeIteratorTestHelper.InvokeRangeCallback(1, 3, (ICollection<uint> x) => values.AddRange(x));

            Assert.Equal(3, values.Count);
            Assert.Equal(1u, values[0]);
            Assert.Equal(2u, values[1]);
            Assert.Equal(3u, values[2]);
        }
    }
}
