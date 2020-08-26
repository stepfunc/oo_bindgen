package io.stepfunc.foo_test;

import io.stepfunc.foo.CallbackInterface;
import io.stepfunc.foo.CallbackSource;
import io.stepfunc.foo.OneTimeCallbackInterface;
import org.assertj.core.data.Percentage;
import org.joou.UInteger;
import org.junit.jupiter.api.Test;

import java.time.Duration;

import static org.assertj.core.api.Assertions.assertThat;
import static org.joou.Unsigned.uint;

public class CallbackTest {
    static class CallbackImpl implements CallbackInterface {
        public UInteger lastValue = uint(0);
        public Duration lastDuration = null;

        @Override
        public void onValue(UInteger value) {
            this.lastValue = value;
        }

        @Override
        public void onDuration(Duration value) {
            this.lastDuration = value;
        }
    }

    static class OneTimeCallbackImpl implements OneTimeCallbackInterface {
        public UInteger lastValue = uint(0);

        @Override
        public void onValue(UInteger value) {
            this.lastValue = value;
        }
    }

    @Test
    public void InterfaceAndOneTimeCallbackTest() {
        try(CallbackSource cbSource = new CallbackSource()) {
            CallbackImpl cb = new CallbackImpl();
            cbSource.addFunc(cb);

            assertThat(cb.lastValue).isEqualTo(uint(0));
            cbSource.setValue(uint(76));
            assertThat(cb.lastValue).isEqualTo(uint(76));

            assertThat(cb.lastDuration).isNull();
            cbSource.setDuration(Duration.ofSeconds(76));
            assertThat(cb.lastDuration).isEqualTo(Duration.ofSeconds(76));

            OneTimeCallbackImpl oneTimeCb = new OneTimeCallbackImpl();
            cbSource.addOneTimeFunc(oneTimeCb);
            assertThat(oneTimeCb.lastValue).isEqualTo(uint(76));
        }
    }

    static class CallbackFinalizerCounterImpl implements CallbackInterface {
        private final Counters counters;

        @Override
        public void onValue(UInteger value) { }
        @Override
        public void onDuration(Duration value) { }

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
    public void CallbackMemoryLeakTest() {
        final int NUM_RUNS = 1000;
        final Counters counters = new Counters();

        for(int i = 0; i < NUM_RUNS; i++) {
            try(CallbackSource cbSource = new CallbackSource()) {
                cbSource.addFunc(new CallbackFinalizerCounterImpl(counters));
                cbSource.setValue(uint(76));
            }
        }

        System.gc();
        System.runFinalization();

        assertThat(counters.numConstructorsCalled).isEqualTo(NUM_RUNS);
        assertThat(counters.numFinalizersCalled).isCloseTo(NUM_RUNS, Percentage.withPercentage(1));
    }
}
