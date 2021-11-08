package io.stepfunc.foo_test;

import io.stepfunc.foo.*;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;
import static org.joou.Unsigned.*;

public class ErrorTest {
    @Test
    public void ThrowsBadPassword() {
        assertThatThrownBy(() -> {
            ClassWithPassword.getSpecialValue("hi!");
        }).isInstanceOf(MyException.class);
    }

    @Test
    public void AcceptsGoodPassword() {
        assertThat(ClassWithPassword.getSpecialValue("12345")).isEqualTo(uint(42));
    }

    @Test
    public void ValidatePasswordThrowsOnBadPassword() {
        assertThatThrownBy(() -> {
            ClassWithPassword.validatePassword("hi!");
        }).isInstanceOf(MyException.class);
    }

    @Test
    public void ValidatePasswordAcceptsCorrectPassword() {
        ClassWithPassword.validatePassword("12345");
    }

    @Test
    public void EchoPasswordThrowsOnBadPassword() {
        assertThatThrownBy(() -> {
            ClassWithPassword.echoPassword("hi!");
        }).isInstanceOf(MyException.class);
    }

    @Test
    public void EchoPasswordReturnsStruct() {
        String result = ClassWithPassword.echoPassword("12345");
        assertThat(result).isEqualTo("12345");
    }

    @Test
    public void ConstructorWithError() {
        assertThatThrownBy(() -> {
            new ClassWithPassword("magnolias for ever");
        }).isInstanceOf(MyException.class);

        ClassWithPassword secret = new ClassWithPassword("12345");
        assertThat(secret.getSpecialValue()).isEqualTo(uint(42));
    }
}
