package io.stepfunc.foo_test;

import io.stepfunc.foo.DurationEchoFunctions;
import org.junit.jupiter.api.Test;

import java.time.Duration;

import static org.assertj.core.api.Assertions.assertThat;

public class DurationTest {
    @Test
    public void DurationZeroTest() {
        Duration zero = Duration.ZERO;
        assertThat(DurationEchoFunctions.millisecondsEcho(zero)).isEqualTo(zero);
        assertThat(DurationEchoFunctions.secondsEcho(zero)).isEqualTo(zero);
        assertThat(DurationEchoFunctions.secondsFloatEcho(zero)).isEqualTo(zero);
    }

    @Test
    public void Duration5sTest() {
        Duration test = Duration.ofSeconds(5);
        assertThat(DurationEchoFunctions.millisecondsEcho(test)).isEqualTo(test);
        assertThat(DurationEchoFunctions.secondsEcho(test)).isEqualTo(test);
        assertThat(DurationEchoFunctions.secondsFloatEcho(test)).isEqualTo(test);
    }

    @Test
    public void Duration250msTest() {
        Duration test = Duration.ofMillis(250);
        assertThat(DurationEchoFunctions.millisecondsEcho(test)).isEqualTo(test);
        assertThat(DurationEchoFunctions.secondsEcho(test)).isEqualTo(Duration.ZERO);
        assertThat(DurationEchoFunctions.secondsFloatEcho(test)).isEqualTo(test);
    }

    @Test
    public void Duration41DaysTest() {
        Duration test = Duration.ofDays(41);
        assertThat(DurationEchoFunctions.millisecondsEcho(test)).isEqualTo(test);
        assertThat(DurationEchoFunctions.secondsEcho(test)).isEqualTo(test);
        assertThat(DurationEchoFunctions.secondsFloatEcho(test)).isEqualTo(test);
    }
}
