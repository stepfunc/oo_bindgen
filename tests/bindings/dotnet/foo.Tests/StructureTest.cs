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

            structure.boolean_value = true;
            structure.uint8_value = 1;
            structure.int8_value = -1;
            structure.uint16_value = 2;
            structure.int16_value = -2;
            structure.uint32_value = 3;
            structure.int32_value = -3;
            structure.uint64_value = 4;
            structure.int64_value = -4;

            structure.structure_value.test = 41;

            structure.enum_value = StructureEnum.Var2;

            structure.duration_millis = TimeSpan.FromMilliseconds(4200);
            structure.duration_seconds = TimeSpan.FromSeconds(76);
            structure.duration_seconds_float = TimeSpan.FromSeconds(15.25f);

            return structure;
        }

        private void CheckStructure(ref Structure structure)
        {
            Assert.True(structure.boolean_value);
            Assert.Equal(1u, structure.uint8_value);
            Assert.Equal(-1, structure.int8_value);
            Assert.Equal(2u, structure.uint16_value);
            Assert.Equal(-2, structure.int16_value);
            Assert.Equal(3u, structure.uint32_value);
            Assert.Equal(-3, structure.int32_value);
            Assert.Equal(4u, structure.uint64_value);
            Assert.Equal(-4, structure.int64_value);

            Assert.Equal(41, structure.structure_value.test);
            Assert.Equal(StructureEnum.Var2, structure.enum_value);

            Assert.Equal(TimeSpan.FromMilliseconds(4200), structure.duration_millis);
            Assert.Equal(TimeSpan.FromSeconds(76), structure.duration_seconds);
            Assert.Equal(TimeSpan.FromSeconds(15.25f), structure.duration_seconds_float);
        }
    }
}
