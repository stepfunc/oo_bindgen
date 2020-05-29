using System;
using Xunit;
using foo;

namespace foo.Tests
{
    public class ClassTests
    {
        [Fact]
        public void ConstructionDestructionTest()
        {
            Assert.Equal(0u, TestClass.ConstructionCounter());

            using(var testclass = new TestClass(41))
            {
                Assert.Equal(1u, TestClass.ConstructionCounter());
                Assert.Equal(41u, testclass.GetValue());

                testclass.IncrementValue();
                Assert.Equal(42u, testclass.GetValue());
            }

            Assert.Equal(0u, TestClass.ConstructionCounter());
        }
    }
}