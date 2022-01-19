package io.stepfunc.foo_test;

import io.stepfunc.foo.PrimitivePointers;
import io.stepfunc.foo.RangeIteratorTestHelper;
import io.stepfunc.foo.RangeReceiver;
import org.assertj.core.api.Assert;
import org.joou.UInteger;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;
import static org.joou.Unsigned.ubyte;
import static org.joou.Unsigned.uint;

class PrimitivePointerTest {

    @Test
    void canReadBoolean() {
        PrimitivePointers values = new PrimitivePointers();
        assertThat(values.getBool(true)).isTrue();
        assertThat(values.getBool(false)).isFalse();
    }

    @Test
    void canReadUnsignedByte() {
        PrimitivePointers values = new PrimitivePointers();
        assertThat(values.getU8(ubyte(0))).isEqualTo(ubyte(0));
        assertThat(values.getU8(ubyte(1))).isEqualTo(ubyte(1));
        assertThat(values.getU8(ubyte(254))).isEqualTo(ubyte(254));
        assertThat(values.getU8(ubyte(255))).isEqualTo(ubyte(255));
    }

    @Test
    void canReadFloat() {
        PrimitivePointers values = new PrimitivePointers();
        assertThat(values.getFloat(3.14f)).isEqualTo(3.14f);
        assertThat(values.getFloat(1e6f)).isEqualTo(1e6f);
        assertThat(values.getFloat(0f)).isEqualTo(0f);
    }

    @Test
    void canReadDouble() {
        PrimitivePointers values = new PrimitivePointers();
        assertThat(values.getDouble(3.14)).isEqualTo(3.14);
        assertThat(values.getDouble(1e6)).isEqualTo(1e6);
        assertThat(values.getDouble(0)).isEqualTo(0);
    }

}
