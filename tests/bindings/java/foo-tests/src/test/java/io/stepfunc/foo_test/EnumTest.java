package io.stepfunc.foo_test;

import io.stepfunc.foo.*;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

public class EnumTest {
    @Test
    public void EnumZeroToFiveEchoTest() {
        EnumZeroToFive value = EnumZeroToFive.ZERO;
        EnumZeroToFive result = EnumEchoFunctions.enumZeroToFiveEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumZeroToFive.ONE;
        result = EnumEchoFunctions.enumZeroToFiveEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumZeroToFive.TWO;
        result = EnumEchoFunctions.enumZeroToFiveEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumZeroToFive.THREE;
        result = EnumEchoFunctions.enumZeroToFiveEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumZeroToFive.FOUR;
        result = EnumEchoFunctions.enumZeroToFiveEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumZeroToFive.FIVE;
        result = EnumEchoFunctions.enumZeroToFiveEcho(value);
        assertThat(result).isEqualTo(value);
    }

    @Test
    public void EnumOneToSixEchoTest() {
        EnumOneToSix value = EnumOneToSix.ONE;
        EnumOneToSix result = EnumEchoFunctions.enumOneToSixEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumOneToSix.TWO;
        result = EnumEchoFunctions.enumOneToSixEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumOneToSix.THREE;
        result = EnumEchoFunctions.enumOneToSixEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumOneToSix.FOUR;
        result = EnumEchoFunctions.enumOneToSixEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumOneToSix.FIVE;
        result = EnumEchoFunctions.enumOneToSixEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumOneToSix.SIX;
        result = EnumEchoFunctions.enumOneToSixEcho(value);
        assertThat(result).isEqualTo(value);
    }

    @Test
    public void EnumDisjointEchoTest() {
        EnumDisjoint value = EnumDisjoint.FIVE;
        EnumDisjoint result = EnumEchoFunctions.enumDisjointEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumDisjoint.ONE;
        result = EnumEchoFunctions.enumDisjointEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumDisjoint.TWENTY;
        result = EnumEchoFunctions.enumDisjointEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumDisjoint.FOUR;
        result = EnumEchoFunctions.enumDisjointEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumDisjoint.SEVEN;
        result = EnumEchoFunctions.enumDisjointEcho(value);
        assertThat(result).isEqualTo(value);

        value = EnumDisjoint.TWO;
        result = EnumEchoFunctions.enumDisjointEcho(value);
        assertThat(result).isEqualTo(value);
    }

    @Test
    public void EnumSingleEchoTest() {
        EnumSingle value = EnumSingle.SINGLE;
        EnumSingle result = EnumEchoFunctions.enumSingleEcho(value);
        assertThat(result).isEqualTo(value);
    }
}
