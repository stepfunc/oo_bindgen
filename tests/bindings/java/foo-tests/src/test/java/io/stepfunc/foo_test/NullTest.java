package io.stepfunc.foo_test;

import io.stepfunc.foo.*;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThatNullPointerException;

public class NullTest {
    @Test
    public void NullUnsignedByte() {
        assertThatNullPointerException().isThrownBy(() -> {
            IntegerEchoFunctions.uint8Echo(null);
        }).withMessage("value cannot be null");
    }

    @Test
    public void NullUnsignedShort() {
        assertThatNullPointerException().isThrownBy(() -> {
            IntegerEchoFunctions.uint16Echo(null);
        }).withMessage("value cannot be null");
    }

    @Test
    public void NullUnsignedInteger() {
        assertThatNullPointerException().isThrownBy(() -> {
            IntegerEchoFunctions.uint32Echo(null);
        }).withMessage("value cannot be null");
    }

    @Test
    public void NullUnsignedLong() {
        assertThatNullPointerException().isThrownBy(() -> {
            IntegerEchoFunctions.uint64Echo(null);
        }).withMessage("value cannot be null");
    }

    @Test
    public void NullString() {
        assertThatNullPointerException().isThrownBy(() -> {
            try(StringClass test = new StringClass()) {
                test.echo(null);
            }
        }).withMessage("value cannot be null");
    }

    @Test
    public void NullEnum() {
        assertThatNullPointerException().isThrownBy(() -> {
            EnumEchoFunctions.enumDisjointEcho(null);
        }).withMessage("value cannot be null");
    }

    @Test
    public void NullCollection() {
        assertThatNullPointerException().isThrownBy(() -> {
            StringCollectionTestMethods.getSize(null);
        }).withMessage("col cannot be null");
    }

    @Test
    public void NullInterface() {
        assertThatNullPointerException().isThrownBy(() -> {
            try(CallbackSource source = new CallbackSource()) {
                source.setInterface(null);
            }
        }).withMessage("cb cannot be null");
    }

    @Test
    public void NullDurationMillis() {
        assertThatNullPointerException().isThrownBy(() -> {
            DurationEchoFunctions.millisecondsEcho(null);
        }).withMessage("value cannot be null");
    }

    @Test
    public void NullDurationSeconds() {
        assertThatNullPointerException().isThrownBy(() -> {
            DurationEchoFunctions.secondsEcho(null);
        }).withMessage("value cannot be null");
    }
}
