package io.stepfunc.foo_test;

import io.stepfunc.foo.StringCollection;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;
import static org.joou.Unsigned.uint;

public class CollectionTest {
    @Test
    public void StringCollectionTest() {
        List<String> strings = new ArrayList<>();
        strings.add("Hello");
        strings.add("World!");
        strings.add("Émile");

        assertThat(StringCollection.getSize(strings)).isEqualTo(uint(3));
        assertThat(StringCollection.getValue(strings, uint(0))).isEqualTo("Hello");
        assertThat(StringCollection.getValue(strings, uint(1))).isEqualTo("World!");
        assertThat(StringCollection.getValue(strings, uint(2))).isEqualTo("Émile");
    }
}
