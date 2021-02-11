using System;
using Xunit;
using foo;

namespace foo.Tests
{   
    public class ConstantTests
    {
        [Fact]
        public void ConstantTest()
        {
            Assert.Equal(1, SpecialValues.One);
            Assert.Equal(2, SpecialValues.Two);
        }
    }
}
