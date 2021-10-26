package io.stepfunc.foo_test;

import io.stepfunc.foo.*;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThatIllegalArgumentException;

public class NullTest {
    @Test
    public void NullUnsignedByte() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            IntegerEchoFunctions.uint8Echo(null);
        }).withMessage("value");
    }

    @Test
    public void NullUnsignedShort() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            IntegerEchoFunctions.uint16Echo(null);
        }).withMessage("value");
    }

    @Test
    public void NullUnsignedInteger() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            IntegerEchoFunctions.uint32Echo(null);
        }).withMessage("value");
    }

    @Test
    public void NullUnsignedLong() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            IntegerEchoFunctions.uint64Echo(null);
        }).withMessage("value");
    }

    @Test
    public void NullString() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            try(StringClass test = new StringClass()) {
                test.echo(null);
            }
        }).withMessage("value");
    }

    @Test
    public void NullEnum() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            EnumEchoFunctions.enumDisjointEcho(null);
        }).withMessage("value");
    }

    @Test
    public void NullCollection() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            StringCollectionMethods.getSize(null);
        }).withMessage("col");
    }

    @Test
    public void NullInterface() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            try(CallbackSource source = new CallbackSource()) {
                source.setInterface(null);
            }
        }).withMessage("cb");
    }

    @Test
    public void NullDurationMillis() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            DurationEchoFunctions.millisecondsEcho(null);
        }).withMessage("value");
    }

    @Test
    public void NullDurationSeconds() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            DurationEchoFunctions.secondsEcho(null);
        }).withMessage("value");
    }
}
