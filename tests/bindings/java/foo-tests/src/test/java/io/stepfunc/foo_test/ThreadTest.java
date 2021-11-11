package io.stepfunc.foo_test;

import io.stepfunc.foo.ValueChangeListener;
import io.stepfunc.foo.ThreadClass;

import org.joou.UInteger;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Disabled;

import java.util.ArrayList;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;
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
        }
        finally {
            // explicitly shutdown the thread so that we can test post conditions
            tc.shutdown();
        }

        assertThat(values.size()).isEqualTo(2);
        assertThat(values.get(0)).isEqualTo(uint(46));
        assertThat(values.get(1)).isEqualTo(uint(43));
    }

}
