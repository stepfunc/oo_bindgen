using System;
using Xunit;
using foo;

namespace foo.Tests
{
    public class ClassTest
    {
        [Fact]
        public void ConstructionDestructionTest()
        {
            Assert.Equal(0u, TestClass.ConstructionCounter());

            var testclass = new TestClass(41);
            Assert.Equal(1u, TestClass.ConstructionCounter());
            Assert.Equal(41u, testclass.GetValue());

            testclass.IncrementValue();
            Assert.Equal(42u, testclass.GetValue());

            testclass.Shutdown();

            Assert.Equal(0u, TestClass.ConstructionCounter());
        }

        [Fact]
        public async void AsyncMethodTest()
        {
            var testclass = new TestClass(41);
            Assert.Equal(1u, TestClass.ConstructionCounter());
            Assert.Equal(42u, await testclass.AddAsync(1));

            testclass.IncrementValue();
            Assert.Equal(43u, await testclass.AddAsync(1));

            testclass.Shutdown();
        }
    }
}
