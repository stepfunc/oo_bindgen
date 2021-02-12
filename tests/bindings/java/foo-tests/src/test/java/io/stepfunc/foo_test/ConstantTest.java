package io.stepfunc.foo_test;

import io.stepfunc.foo.SpecialValues;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

class ConstantTest {
    @Test
    void specialValues() {
        assertThat(SpecialValues.ONE.byteValue()).isEqualTo((byte) 0x01);
        assertThat(SpecialValues.TWO.byteValue()).isEqualTo((byte) 0x02);
    }
}
