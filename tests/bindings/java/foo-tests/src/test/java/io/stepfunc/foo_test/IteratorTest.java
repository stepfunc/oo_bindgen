package io.stepfunc.foo_test;

import io.stepfunc.foo.StringIterator;
import io.stepfunc.foo.StringIteratorItem;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;

public class IteratorTest {
    @Test
    public void StringIteratorTest() {
        List<StringIteratorItem> characters = StringIterator.iterateString("ABCDE");
        assertThat(characters.stream().map(item -> item.value)).containsExactly((byte)65, (byte)66, (byte)67, (byte)68, (byte)69);
    }
}
