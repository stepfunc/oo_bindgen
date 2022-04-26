using System;
using Xunit;
using foo;
using System.Linq;
using System.Collections.Generic;

namespace foo.Tests
{
    public class PrimitivePointrTest
    {
        void AssertFloatEqual(float expected, float value)
        {
            Assert.True(Math.Abs(expected - value) < 1e-6);
        }

        void AssertDoubleEqual(double expected, double value)
        {
            Assert.True(Math.Abs(expected - value) < 1e-6);
        }

        [Fact]
        public void CanReadBoolean()
        {
            var values = new PrimitivePointers();

            Assert.False(values.GetBool(false));
            Assert.True(values.GetBool(true));
        }

        [Fact]
        public void CanReadUnsignedByte()
        {
            var values = new PrimitivePointers();

            Assert.Equal(0, values.GetU8(0));
            Assert.Equal(1, values.GetU8(1));
            Assert.Equal(254, values.GetU8(254));
            Assert.Equal(255, values.GetU8(255));
        }

        [Fact]
        public void CanReadFloat()
        {
            var values = new PrimitivePointers();
            AssertFloatEqual(3.14f, values.GetFloat(3.14f));
            AssertFloatEqual(1e6f, values.GetFloat(1e6f));
            AssertFloatEqual(0f, values.GetFloat(0f));
        }

        [Fact]
        public void CanReadDouble()
        {
            var values = new PrimitivePointers();
            AssertDoubleEqual(3.14, values.GetDouble(3.14));
            AssertDoubleEqual(1e6, values.GetDouble(1e6));
            AssertDoubleEqual(0, values.GetDouble(0));
        }
    }
}
