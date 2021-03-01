using System;
using Xunit;
using foo;

namespace foo.Tests
{
   
    public class OpaqueStructureTest
    {
        [Fact]
        public void StructureByValueEchoTest()
        {            
            Assert.Equal(42ul, OpaqueStruct.CreateMagicValue().GetId());
        }
        
    }
}
