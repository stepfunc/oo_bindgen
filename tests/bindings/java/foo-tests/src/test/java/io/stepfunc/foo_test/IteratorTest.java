package io.stepfunc.foo_test;

import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;
import static org.joou.Unsigned.*;

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

    static class TestChunkReceiver implements ChunkReceiver {
        List<String> values = new ArrayList<>();

        public void onChunk(java.util.List<Chunk> values) {
            for(Chunk c : values) {
                java.io.ByteArrayOutputStream chars = new java.io.ByteArrayOutputStream();
                for(ByteValue bv : c.iter) {
                    chars.write(bv.value.intValue());
                }
                try {
                   this.values.add(new String(chars.toByteArray(), "UTF-8"));
                }
                catch(Exception ex) {
                   System.out.println(ex);
                   System.exit(-1);
                }

            }
        }
    }

    @Test
    void ChunkIteratorTest() {
        TestChunkReceiver receiver = new TestChunkReceiver();
        DoubleIteratorTestHelper.iterateStringByChunks("Hello World!", uint(3), receiver);
        assertThat(receiver.values.size()).isEqualTo(4);
        assertThat(receiver.values.get(0)).isEqualTo("Hel");
        assertThat(receiver.values.get(1)).isEqualTo("lo ");
        assertThat(receiver.values.get(2)).isEqualTo("Wor");
        assertThat(receiver.values.get(3)).isEqualTo("ld!");
    }

    @Test
    void StringIteratorTest() {
        TestValuesReceiver receiver = new TestValuesReceiver();
        IteratorTestHelper.invokeCallback("ABCDE", receiver);
        assertThat(receiver.values.stream()).containsExactly(ubyte(65), ubyte(66), ubyte(67), ubyte(68), ubyte(69));
    }
}
