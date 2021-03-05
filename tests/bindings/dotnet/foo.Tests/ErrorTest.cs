using System;
using Xunit;
using foo;

namespace foo.Tests
{
    public class ErrorTest
    {
        [Fact]
        public void ThrowsBadPassword()
        {
            try
            {
                ClassWithPassword.GetSpecialValue("hi!");
                Assert.True(false);
            }
            catch(MyException ex)
            {
                Assert.Equal(MyError.BadPassword, ex.error);
            }            
        }

        [Fact]
        public void AcceptsGoodPassword()
        {
            Assert.Equal(42u, ClassWithPassword.GetSpecialValue("12345"));
        }
    }
}
