package io.stepfunc.foo_test;

import io.stepfunc.foo.MathIsBroken;
import io.stepfunc.foo.BrokenMathException;
import io.stepfunc.foo.ValueChangeListener;
import io.stepfunc.foo.ThreadClass;

import org.joou.UInteger;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Disabled;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ExecutionException;

import static org.assertj.core.api.Assertions.*;
import static org.joou.Unsigned.uint;

class ThreadTest {

    @Test
    void testAsynchronousCallbacks() throws Exception {
        List<UInteger> values = new ArrayList<>();
        ThreadClass tc = new ThreadClass(uint(42), v -> values.add(v));
        try {
            UInteger result = tc.add(uint(4)).toCompletableFuture().get();
            assertThat(result).isEqualTo(uint(46));
            tc.update(uint(43));
            tc.execute(value -> uint(2*value.intValue()));
        }
        finally {
            // explicitly shutdown the thread so that we can test post conditions
            tc.shutdown();
        }

        assertThat(values.size()).isEqualTo(3);
        assertThat(values.get(0)).isEqualTo(uint(46));
        assertThat(values.get(1)).isEqualTo(uint(43));
        assertThat(values.get(2)).isEqualTo(uint(86));
    }

    @Test
    void testAsynchronousExceptions() throws Exception {

        ThreadClass tc = new ThreadClass(uint(42), v -> {});

        try {
            tc.queueError(MathIsBroken.MATH_IS_BROKE);
            UInteger result = tc.add(uint(4)).toCompletableFuture().get();
            fail("Exception not thrown");
        }
        catch(ExecutionException ex) {
            BrokenMathException cause = (BrokenMathException) ex.getCause();
            assertThat(cause.error).isEqualTo(MathIsBroken.MATH_IS_BROKE);
        }
        finally {
            // explicitly shutdown the thread so that we can test post conditions
            tc.shutdown();
        }
    }

}
