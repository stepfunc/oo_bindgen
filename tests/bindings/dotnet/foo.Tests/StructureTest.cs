using System;
using Xunit;
using foo;

namespace foo.Tests
{
    class TestInterface : IStructureInterface
    {
        public Structure lastValue = null;

        public void OnValue(Structure value)
        {
            this.lastValue = value;
        }
    }

    public class StructureTest
    {               
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
            var structure = new Structure(new TestInterface());

            return structure;
        }

        private void CheckStructure(Structure structure)
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
        }
    }
}
