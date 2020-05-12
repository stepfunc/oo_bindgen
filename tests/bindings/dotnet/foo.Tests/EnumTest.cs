using System;
using Xunit;
using foo;

namespace foo.Tests
{
    public class EnumTests
    {
        [Fact]
        public void EnumZeroToFiveEchoTest()
        {
            var value = EnumZeroToFive.Zero;
            var result = EnumEchoFunctions.EnumZeroToFiveEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(0, (int)result);

            value = EnumZeroToFive.One;
            result = EnumEchoFunctions.EnumZeroToFiveEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(1, (int)result);

            value = EnumZeroToFive.Two;
            result = EnumEchoFunctions.EnumZeroToFiveEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(2, (int)result);

            value = EnumZeroToFive.Three;
            result = EnumEchoFunctions.EnumZeroToFiveEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(3, (int)result);

            value = EnumZeroToFive.Four;
            result = EnumEchoFunctions.EnumZeroToFiveEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(4, (int)result);

            value = EnumZeroToFive.Five;
            result = EnumEchoFunctions.EnumZeroToFiveEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(5, (int)result);
        }

        [Fact]
        public void EnumOneToSixEchoTest()
        {
            var value = EnumOneToSix.One;
            var result = EnumEchoFunctions.EnumOneToSixEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(1, (int)result);

            value = EnumOneToSix.Two;
            result = EnumEchoFunctions.EnumOneToSixEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(2, (int)result);

            value = EnumOneToSix.Three;
            result = EnumEchoFunctions.EnumOneToSixEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(3, (int)result);

            value = EnumOneToSix.Four;
            result = EnumEchoFunctions.EnumOneToSixEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(4, (int)result);

            value = EnumOneToSix.Five;
            result = EnumEchoFunctions.EnumOneToSixEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(5, (int)result);

            value = EnumOneToSix.Six;
            result = EnumEchoFunctions.EnumOneToSixEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(6, (int)result);
        }

        [Fact]
        public void EnumDisjointEchoTest()
        {
            var value = EnumDisjoint.Five;
            var result = EnumEchoFunctions.EnumDisjointEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(5, (int)result);

            value = EnumDisjoint.One;
            result = EnumEchoFunctions.EnumDisjointEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(1, (int)result);

            value = EnumDisjoint.Twenty;
            result = EnumEchoFunctions.EnumDisjointEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(20, (int)result);

            value = EnumDisjoint.Four;
            result = EnumEchoFunctions.EnumDisjointEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(4, (int)result);

            value = EnumDisjoint.Seven;
            result = EnumEchoFunctions.EnumDisjointEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(7, (int)result);

            value = EnumDisjoint.Two;
            result = EnumEchoFunctions.EnumDisjointEcho(value);
            Assert.Equal(value, result);
            Assert.Equal(2, (int)result);
        }
    }
}
