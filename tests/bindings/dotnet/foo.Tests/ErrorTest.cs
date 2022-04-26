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

        [Fact]
        public void ValidatePasswordThrowsOnBadPassword()
        {
            Assert.Throws<MyException>(() => ClassWithPassword.ValidatePassword("hi!"));
        }

        [Fact]
        public void ValidatePasswordAcceptsGoodPassword()
        {
            ClassWithPassword.ValidatePassword("12345");
        }

        [Fact]
        public void EchoPasswordThrowsOnBadPassword()
        {
            Assert.Throws<MyException>(() => ClassWithPassword.EchoPassword("hi!"));
        }

        [Fact]
        public void EchoPasswordAcceptsGoodPassword()
        {
            var result = ClassWithPassword.EchoPassword("12345");
            Assert.Equal("12345", result);
        }

        [Fact]
        public void ConstructorWithError()
        {
            Assert.Throws<MyException>(() => new ClassWithPassword("hi!"));

            var secret = new ClassWithPassword("12345");
            Assert.Equal(42u, secret.GetSpecialValue());
        }
    }
}
