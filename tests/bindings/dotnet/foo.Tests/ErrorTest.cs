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
        public void GetStructThrowsOnBadPassword()
        {
            Assert.Throws<MyException>(() => ClassWithPassword.GetStruct("hi!"));
        }

        [Fact]
        public void GetStructAcceptsGoodPassword()
        {
            var result = ClassWithPassword.GetStruct("12345");
            Assert.Equal(41, result.Test);
            Assert.Equal(StructureEnum.Var2, result.FirstEnumValue);
            Assert.Equal(1, result.Int1);
            Assert.False(result.Bool2);
            Assert.Equal(StructureEnum.Var2, result.SecondEnumValue);
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
            Assert.Equal(42u, secret.GetSpecialValueFromInstance());
        }
    }
}
