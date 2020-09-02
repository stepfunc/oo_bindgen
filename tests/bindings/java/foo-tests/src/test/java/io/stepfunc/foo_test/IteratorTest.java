package io.stepfunc.foo_test;

import io.stepfunc.foo.StringIterator;
import io.stepfunc.foo.StringIteratorItem;
import org.junit.jupiter.api.Test;

import java.util.Collection;

import static org.assertj.core.api.Assertions.assertThat;
import static org.joou.Unsigned.ubyte;

public class IteratorTest {
    @Test
    public void StringIteratorTest() {
        Collection<StringIteratorItem> characters = StringIterator.iterateString("ABCDE");
        assertThat(characters.stream().map(item -> item.value)).containsExactly(ubyte(65), ubyte(66), ubyte(67), ubyte(68), ubyte(69));
    }
}
