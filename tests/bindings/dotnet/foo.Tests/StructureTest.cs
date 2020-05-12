using System;
using Xunit;
using foo;

namespace foo.Tests
{
    public class StructureTests
    {
        [Fact]
        public void StructureByValueEchoTest()
        {
            var value = CreateStructure();
            var result = StructEchoFunctions.StructByValueEcho(value);
            CheckStructure(ref result);
        }

        [Fact]
        public void StructureByReferenceEchoTest()
        {
            var value = CreateStructure();
            var result = StructEchoFunctions.StructByReferenceEcho(value);
            CheckStructure(ref result);
        }

        private Structure CreateStructure()
        {
            var structure = new Structure();

            structure.booleanValue = true;
            structure.uint8Value = 1;
            structure.int8Value = -1;
            structure.uint16Value = 2;
            structure.int16Value = -2;
            structure.uint32Value = 3;
            structure.int32Value = -3;
            structure.uint64Value = 4;
            structure.int64Value = -4;

            structure.structureValue.test = 41;

            structure.enumValue = StructureEnum.Var2;

            structure.durationMillis = TimeSpan.FromMilliseconds(4200);
            structure.durationSeconds = TimeSpan.FromSeconds(76);
            structure.durationSecondsFloat = TimeSpan.FromSeconds(15.25f);

            return structure;
        }

        private void CheckStructure(ref Structure structure)
        {
            Assert.True(structure.booleanValue);
            Assert.Equal(1u, structure.uint8Value);
            Assert.Equal(-1, structure.int8Value);
            Assert.Equal(2u, structure.uint16Value);
            Assert.Equal(-2, structure.int16Value);
            Assert.Equal(3u, structure.uint32Value);
            Assert.Equal(-3, structure.int32Value);
            Assert.Equal(4u, structure.uint64Value);
            Assert.Equal(-4, structure.int64Value);

            Assert.Equal(41, structure.structureValue.test);
            Assert.Equal(StructureEnum.Var2, structure.enumValue);

            Assert.Equal(TimeSpan.FromMilliseconds(4200), structure.durationMillis);
            Assert.Equal(TimeSpan.FromSeconds(76), structure.durationSeconds);
            Assert.Equal(TimeSpan.FromSeconds(15.25f), structure.durationSecondsFloat);
        }
    }
}
