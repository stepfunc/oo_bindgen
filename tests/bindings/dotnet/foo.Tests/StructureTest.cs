using System;
using Xunit;
using foo;

namespace foo.Tests
{
    class TestInterface : IStructureInterface
    {
        public Structure? lastValue = null;

        public void OnValue(Structure? value)
        {
            this.lastValue = value;
        }
    }

    public class StructureTest
    {
        [Fact]
        public void StructureByValueEchoTest()
        {
            var value = CreateStructure();
            var result = Structure.StructByValueEcho(value);
            CheckStructure(ref result);
        }

        [Fact]
        public void StructureByReferenceEchoTest()
        {
            var value = CreateStructure();
            var result = value.StructByReferenceEcho();
            CheckStructure(ref result);
        }

        [Fact]
        public void InterfaceStruct()
        {
            var value = CreateStructure();
            var testInterface = new TestInterface();
            value.InterfaceValue = testInterface;

            Structure.StructByValueEcho(value);

            Assert.NotNull(testInterface.lastValue);
            var lastValue = testInterface.lastValue.Value;
            CheckStructure(ref lastValue);
        }

        [Fact]
        public void StructureMemoryLeakTest()
        {
            var numRuns = 1000;

            for (int i = 0; i < numRuns; i++)
            {
                CreateStructure();
            }
        }

        private Structure CreateStructure()
        {
            var structure = new Structure();

            structure.BooleanValue = true;
            structure.Uint8Value = 1;
            structure.Int8Value = -1;
            structure.Uint16Value = 2;
            structure.Int16Value = -2;
            structure.Uint32Value = 3;
            structure.Int32Value = -3;
            structure.Uint64Value = 4;
            structure.Int64Value = -4;
            structure.FloatValue = 12.34f;
            structure.DoubleValue = -56.78;
            structure.StringValue = "Hello from C#!";

            structure.StructureValue.Test = 41;

            structure.EnumValue = StructureEnum.Var2;

            structure.InterfaceValue = new TestInterface();

            structure.DurationMillis = TimeSpan.FromMilliseconds(4200);
            structure.DurationSeconds = TimeSpan.FromSeconds(76);
            structure.DurationSecondsFloat = TimeSpan.FromSeconds(15.25f);

            return structure;
        }

        private void CheckStructure(ref Structure structure)
        {
            Assert.True(structure.BooleanValue);
            Assert.Equal(1u, structure.Uint8Value);
            Assert.Equal(-1, structure.Int8Value);
            Assert.Equal(2u, structure.Uint16Value);
            Assert.Equal(-2, structure.Int16Value);
            Assert.Equal(3u, structure.Uint32Value);
            Assert.Equal(-3, structure.Int32Value);
            Assert.Equal(4u, structure.Uint64Value);
            Assert.Equal(-4, structure.Int64Value);
            Assert.Equal(12.34f, structure.FloatValue);
            Assert.Equal(-56.78, structure.DoubleValue);

            Assert.Equal(41, structure.StructureValue.Test);
            Assert.Equal(StructureEnum.Var2, structure.EnumValue);

            Assert.Equal(TimeSpan.FromMilliseconds(4200), structure.DurationMillis);
            Assert.Equal(TimeSpan.FromSeconds(76), structure.DurationSeconds);
            Assert.Equal(TimeSpan.FromSeconds(15.25f), structure.DurationSecondsFloat);
        }
    }
}
