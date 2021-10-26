using System;
using Xunit;
using foo;

namespace foo.Tests
{
    class EmptyInterface : IEmptyInterface {}

    public class StructureTest
    {                       
        private static void CheckNumbersDefaults(Numbers x)
        {
            Assert.Equal(1u, x.Uint8Value);
            Assert.Equal(-1, x.Int8Value);
            Assert.Equal(2u, x.Uint16Value);
            Assert.Equal(-2, x.Int16Value);
            Assert.Equal(3u, x.Uint32Value);
            Assert.Equal(-3, x.Int32Value);
            Assert.Equal(4u, x.Uint64Value);
            Assert.Equal(-4, x.Int64Value);
            Assert.Equal(12.34f, x.FloatValue);
            Assert.Equal(-56.78, x.DoubleValue);
        }

        private static void CheckInnerStructureDefaults(InnerStructure x)
        {
            Assert.NotNull(x.InterfaceField);
            CheckNumbersDefaults(x.NumbersField);
        }

        private static void CheckStructureDefaults(Structure x)
        {
            Assert.False(x.BooleanFalse);
            Assert.True(x.BooleanTrue);
            Assert.Equal(StructureEnum.Var1, x.EnumVar1);
            Assert.Equal(StructureEnum.Var2, x.EnumVar2);
            Assert.Equal(TimeSpan.FromMilliseconds(4200), x.DurationMillis);
            Assert.Equal(TimeSpan.FromSeconds(76), x.DurationSeconds);
            Assert.Equal("Hello", x.StringHello);
            CheckInnerStructureDefaults(x.InnerStructure);
        }

        [Fact]
        public void StructureConstructorTest()
        {
            var x = new Structure(new InnerStructure(new EmptyInterface()));
            CheckStructureDefaults(x);
        }

       
       
    }
}
