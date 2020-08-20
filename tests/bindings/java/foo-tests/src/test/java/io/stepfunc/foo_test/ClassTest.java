package io.stepfunc.foo_test;

import io.stepfunc.foo.TestClass;
import org.junit.jupiter.api.Test;

import java.util.concurrent.ExecutionException;

import static org.assertj.core.api.Assertions.assertThat;

public class ClassTest {
    @Test
    public void ConstructionDestructionTest() {
        assertThat(TestClass.constructionCounter()).isZero();

        try(TestClass testclass = new TestClass(41)) {
            assertThat(TestClass.constructionCounter()).isEqualTo(1);
            assertThat(testclass.getValue()).isEqualTo(41);

            testclass.incrementValue();
            assertThat(testclass.getValue()).isEqualTo(42);
        }

        assertThat(TestClass.constructionCounter()).isZero();
    }

    @Test
    public void AsyncMethodTest() throws ExecutionException, InterruptedException {
        try(TestClass testclass = new TestClass(41)) {
            assertThat(TestClass.constructionCounter()).isEqualTo(1);
            assertThat(testclass.getValueAsync().get()).isEqualTo(41);

            testclass.incrementValue();
            assertThat(testclass.getValueAsync().get()).isEqualTo(42);
        }
    }
}
