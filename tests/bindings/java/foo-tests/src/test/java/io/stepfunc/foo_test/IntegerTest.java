package io.stepfunc.foo_test;

import io.stepfunc.foo.IntegerEchoFunctions;
import org.joou.UByte;
import org.joou.UInteger;
import org.joou.ULong;
import org.joou.UShort;
import org.junit.jupiter.api.Test;

import java.util.Collection;

import static org.assertj.core.api.Assertions.assertThat;
import static org.joou.Unsigned.ubyte;

public class IntegerTest {
    @Test
    public void Uint8Test() {
        assertThat(IntegerEchoFunctions.uint8Echo(UByte.MIN)).isEqualTo(UByte.MIN);
        assertThat(IntegerEchoFunctions.uint8Echo(UByte.MAX)).isEqualTo(UByte.MAX);
    }

    @Test
    public void Uint16Test() {
        assertThat(IntegerEchoFunctions.uint16Echo(UShort.MIN)).isEqualTo(UShort.MIN);
        assertThat(IntegerEchoFunctions.uint16Echo(UShort.MAX)).isEqualTo(UShort.MAX);
    }

    @Test
    public void Uint32Test() {
        assertThat(IntegerEchoFunctions.uint32Echo(UInteger.MIN)).isEqualTo(UInteger.MIN);
        assertThat(IntegerEchoFunctions.uint32Echo(UInteger.MAX)).isEqualTo(UInteger.MAX);
    }

    @Test
    public void Uint64Test() {
        assertThat(IntegerEchoFunctions.uint64Echo(ULong.MIN)).isEqualTo(ULong.MIN);
        assertThat(IntegerEchoFunctions.uint64Echo(ULong.MAX)).isEqualTo(ULong.MAX);
    }
}
