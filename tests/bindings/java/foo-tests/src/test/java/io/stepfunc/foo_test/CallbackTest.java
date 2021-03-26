package io.stepfunc.foo_test;

import io.stepfunc.foo.CallbackInterface;
import io.stepfunc.foo.CallbackSource;
import org.assertj.core.data.Percentage;
import org.joou.UInteger;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Disabled;

import java.time.Duration;

import static org.assertj.core.api.Assertions.assertThat;
import static org.joou.Unsigned.uint;

public class CallbackTest {
    static class CallbackImpl implements CallbackInterface {
        public UInteger lastValue = uint(0);
        public Duration lastDuration = null;

        @Override
        public UInteger onValue(UInteger value) {
            this.lastValue = value;
            return value;
        }

        @Override
        public Duration onDuration(Duration value) {
            this.lastDuration = value;
            return value;
        }
    }

    @Test
    public void InterfaceAndOneTimeCallbackTest() {
        try(CallbackSource cbSource = new CallbackSource()) {
            CallbackImpl cb = new CallbackImpl();
            cbSource.setInterface(cb);

            assertThat(cb.lastValue).isEqualTo(uint(0));
            assertThat(cbSource.setValue(uint(76))).isEqualTo(uint(76));
            assertThat(cb.lastValue).isEqualTo(uint(76));

            assertThat(cb.lastDuration).isNull();
            assertThat(cbSource.setDuration(Duration.ofSeconds(76))).isEqualTo(Duration.ofSeconds(76));
            assertThat(cb.lastDuration).isEqualTo(Duration.ofSeconds(76));
        }
    }

    static class CallbackFinalizerCounterImpl implements CallbackInterface {
        private final Counters counters;

        @Override
        public UInteger onValue(UInteger value) {
            return value;
        }
        @Override
        public Duration onDuration(Duration value) {
            return value;
        }

        public CallbackFinalizerCounterImpl(Counters counters) {
            this.counters = counters;
            this.counters.numConstructorsCalled++;
        }

        @Override
        public void finalize() {
            this.counters.numFinalizersCalled++;
        }
    }

    static class Counters {
        public int numConstructorsCalled = 0;
        public int numFinalizersCalled = 0;
    }

    @Test
    @Disabled // System.gc and System.runFinalization are not deterministic
    public void CallbackMemoryLeakTest() {
        final int NUM_RUNS = 1000;
        final Counters counters = new Counters();

        for(int i = 0; i < NUM_RUNS; i++) {
            try(CallbackSource cbSource = new CallbackSource()) {
                cbSource.setInterface(new CallbackFinalizerCounterImpl(counters));
                cbSource.setValue(uint(76));
            }
        }

        System.gc();
        System.runFinalization();

        assertThat(counters.numConstructorsCalled).isEqualTo(NUM_RUNS);
        assertThat(counters.numFinalizersCalled).isCloseTo(NUM_RUNS, Percentage.withPercentage(5));
    }
}
