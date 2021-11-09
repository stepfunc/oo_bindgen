package io.stepfunc.foo_test;

import io.stepfunc.foo.TestClass;
import org.junit.jupiter.api.Test;

import java.util.concurrent.ExecutionException;

import static org.assertj.core.api.Assertions.assertThat;
import static org.joou.Unsigned.uint;

public class ClassTest {
    @Test
    public void ConstructionDestructionTest() {
        assertThat(TestClass.constructionCounter().intValue()).isZero();

        TestClass testclass = new TestClass(uint(41));
        assertThat(TestClass.constructionCounter()).isEqualTo(uint(1));
        assertThat(testclass.getValue()).isEqualTo(uint(41));

        testclass.incrementValue();
        assertThat(testclass.getValue()).isEqualTo(uint(42));

        testclass.delete();

        assertThat(TestClass.constructionCounter().intValue()).isZero();
    }

    @Test
    public void AsyncMethodTest() throws ExecutionException, InterruptedException {
        TestClass testclass = new TestClass(uint(41));
        assertThat(TestClass.constructionCounter()).isEqualTo(uint(1));
        assertThat(testclass.addAsync(uint(1)).toCompletableFuture().get()).isEqualTo(uint(42));

        testclass.incrementValue();
        assertThat(testclass.addAsync(uint(1)).toCompletableFuture().get()).isEqualTo(uint(43));

        testclass.delete();
    }
}
