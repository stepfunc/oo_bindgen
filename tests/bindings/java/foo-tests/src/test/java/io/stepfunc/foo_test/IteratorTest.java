package io.stepfunc.foo_test;

import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;
import static org.joou.Unsigned.ubyte;

import io.stepfunc.foo.*;

class IteratorTest {

    static class TestValuesReceiver implements ValuesReceiver {
        List<org.joou.UByte> values = new ArrayList<>();

        public void onCharacters(java.util.List<StringIteratorItem> values){
            for(StringIteratorItem item : values) {
                this.values.add(item.value);
            }
        }
    }

    @Test
    void StringIteratorTest() {
        TestValuesReceiver receiver = new TestValuesReceiver();
        IteratorTestHelper.invokeCallback("ABCDE", receiver);
        assertThat(receiver.values.stream()).containsExactly(ubyte(65), ubyte(66), ubyte(67), ubyte(68), ubyte(69));
    }
}
