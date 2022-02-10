package io.stepfunc.foo_test;

import io.stepfunc.foo.*;
import org.joou.UInteger;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;
import static org.joou.Unsigned.ubyte;
import static org.joou.Unsigned.uint;

class PrimitiveIteratorTest {
    static class TestRangeReceiver implements RangeReceiver {
        List<org.joou.UInteger> values = new ArrayList<>();

        @Override
        public void onRange(List<UInteger> values) {
            this.values.addAll(values);
        }
    }

    @Test
    void canReceiveUnsignedInts() {
        TestRangeReceiver rx = new TestRangeReceiver();
        RangeIteratorTestHelper.invokeRangeCallback(uint(1), uint(3), rx);
        assertThat(rx.values.size()).isEqualTo(3);
        assertThat(rx.values.get(0)).isEqualTo(uint(1));
        assertThat(rx.values.get(1)).isEqualTo(uint(2));
        assertThat(rx.values.get(2)).isEqualTo(uint(3));
    }
}
