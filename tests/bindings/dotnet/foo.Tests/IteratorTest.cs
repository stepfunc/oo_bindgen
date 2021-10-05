using System;
using Xunit;
using foo;
using System.Linq;
using System.Collections.Generic;

namespace foo.Tests
{
    public class IteratorTest
    {
        class ValuesReceiver : IValuesReceiver
        {
            public readonly List<Byte> values = new List<Byte>();

            public void OnCharacters(ICollection<StringIteratorItem> values)
            {
                foreach(StringIteratorItem v in values)
                {
                    this.values.Add(v.Value);
                }
            }
        }

        [Fact]
        public void StringIteratorTest()
        {
            var receiver = new ValuesReceiver();
            
            IteratorTestHelper.InvokeCallback("ABCDE", receiver);
            Assert.Equal(new byte[] { 65, 66, 67, 68, 69 }, receiver.values);
        }
        
    }
}

