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
    public void GetStructThrowsOnBadPassword() {
        assertThatThrownBy(() -> {
            ClassWithPassword.getStruct("hi!");
        }).isInstanceOf(MyException.class);
    }

    @Test
    public void GetStructReturnsStruct() {
        OtherStructure result = ClassWithPassword.getStruct("12345");
        assertThat(result.test).isEqualTo(ushort(41));
        assertThat(result.firstEnumValue).isEqualTo(StructureEnum.VAR2);
        assertThat(result.int1).isEqualTo((short)1);
        assertThat(result.bool2).isEqualTo(false);
        assertThat(result.secondEnumValue).isEqualTo(StructureEnum.VAR2);
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
        assertThat(secret.getSpecialValueFromInstance()).isEqualTo(uint(42));
    }
}
