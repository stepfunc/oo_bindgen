using System;
using Xunit;
using foo;

namespace foo.Tests
{
    public class DurationTest
    {
        [Fact]
        public void DurationZeroTest()
        {
            var zero = TimeSpan.Zero;
            Assert.Equal(zero, DurationEchoFunctions.MillisecondsEcho(zero));
            Assert.Equal(zero, DurationEchoFunctions.SecondsEcho(zero));
            Assert.Equal(zero, DurationEchoFunctions.SecondsFloatEcho(zero));
        }

        [Fact]
        public void Duration5sTest()
        {
            var fiveS = TimeSpan.FromSeconds(5);
            Assert.Equal(fiveS, DurationEchoFunctions.MillisecondsEcho(fiveS));
            Assert.Equal(fiveS, DurationEchoFunctions.SecondsEcho(fiveS));
            Assert.Equal(fiveS, DurationEchoFunctions.SecondsFloatEcho(fiveS));
        }

        [Fact]
        public void Duration250msTest()
        {
            var test = TimeSpan.FromMilliseconds(250);
            Assert.Equal(test, DurationEchoFunctions.MillisecondsEcho(test));
            Assert.Equal(TimeSpan.FromSeconds(0), DurationEchoFunctions.SecondsEcho(test));
            Assert.Equal(test, DurationEchoFunctions.SecondsFloatEcho(test));
        }

        [Fact]
        public void Duration41DaysTest()
        {
            var test = TimeSpan.FromDays(41);
            Assert.Equal(test, DurationEchoFunctions.MillisecondsEcho(test));
            Assert.Equal(test, DurationEchoFunctions.SecondsEcho(test));
            Assert.Equal(test, DurationEchoFunctions.SecondsFloatEcho(test));
        }
    }
}
