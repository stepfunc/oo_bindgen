package io.stepfunc.foo_test;

import io.stepfunc.foo.IntegerEchoFunctions;
import org.joou.UByte;
import org.joou.UInteger;
import org.joou.ULong;
import org.joou.UShort;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

public class IntegerTest {
    @Test
    public void Uint8Test() {
        assertThat(IntegerEchoFunctions.uint8Echo(UByte.MIN)).isEqualTo(UByte.MIN);
        assertThat(IntegerEchoFunctions.uint8Echo(UByte.MAX)).isEqualTo(UByte.MAX);
    }

    @Test
    public void Sint8Test() {
        assertThat(IntegerEchoFunctions.sint8Echo(Byte.MIN_VALUE)).isEqualTo(Byte.MIN_VALUE);
        assertThat(IntegerEchoFunctions.sint8Echo(Byte.MAX_VALUE)).isEqualTo(Byte.MAX_VALUE);
    }

    @Test
    public void Uint16Test() {
        assertThat(IntegerEchoFunctions.uint16Echo(UShort.MIN)).isEqualTo(UShort.MIN);
        assertThat(IntegerEchoFunctions.uint16Echo(UShort.MAX)).isEqualTo(UShort.MAX);
    }

    @Test
    public void Sint16Test() {
        assertThat(IntegerEchoFunctions.sint16Echo(Short.MIN_VALUE)).isEqualTo(Short.MIN_VALUE);
        assertThat(IntegerEchoFunctions.sint16Echo(Short.MAX_VALUE)).isEqualTo(Short.MAX_VALUE);
    }

    @Test
    public void Uint32Test() {
        assertThat(IntegerEchoFunctions.uint32Echo(UInteger.MIN)).isEqualTo(UInteger.MIN);
        assertThat(IntegerEchoFunctions.uint32Echo(UInteger.MAX)).isEqualTo(UInteger.MAX);
    }

    @Test
    public void Sint32Test() {
        assertThat(IntegerEchoFunctions.sint32Echo(Integer.MIN_VALUE)).isEqualTo(Integer.MIN_VALUE);
        assertThat(IntegerEchoFunctions.sint32Echo(Integer.MAX_VALUE)).isEqualTo(Integer.MAX_VALUE);
    }

    @Test
    public void Uint64Test() {
        assertThat(IntegerEchoFunctions.uint64Echo(ULong.MIN)).isEqualTo(ULong.MIN);
        assertThat(IntegerEchoFunctions.uint64Echo(ULong.MAX)).isEqualTo(ULong.MAX);
    }

    @Test
    public void Sint64Test() {
        assertThat(IntegerEchoFunctions.sint64Echo(Long.MIN_VALUE)).isEqualTo(Long.MIN_VALUE);
        assertThat(IntegerEchoFunctions.sint64Echo(Long.MAX_VALUE)).isEqualTo(Long.MAX_VALUE);
    }
}
