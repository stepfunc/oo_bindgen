package io.stepfunc.foo_test;

import io.stepfunc.foo.StringClass;
import org.junit.jupiter.api.Test;

import java.io.UnsupportedEncodingException;
import java.nio.charset.StandardCharsets;

import static org.assertj.core.api.Assertions.assertThat;

public class StringTest {
    final String ENGLISH_SENTENCE_1 = "I like to be home with my monkey and my dog";
    final String ENGLISH_SENTENCE_2 = "Don't care, shut up, play the record!";
    final String FRENCH_SENTENCE_1 = "Devant mon miroir j'ai rêvé d'être une star, j'ai rêvé d'être immortellement belle";
    final String FRENCH_SENTENCE_2 = "Ce soir j'irai voir à travers le miroir, si la vie est éternelle";

    @Test
    public void EnglishTest() {
        try(StringClass stringclass = new StringClass()) {
            assertThat(stringclass.echo(ENGLISH_SENTENCE_1)).isEqualTo(ENGLISH_SENTENCE_1);
            assertThat(stringclass.echo(ENGLISH_SENTENCE_2)).isEqualTo(ENGLISH_SENTENCE_2);
        }
    }

    @Test
    public void FrenchTest() {
        try(StringClass stringclass = new StringClass()) {
            assertThat(stringclass.echo(FRENCH_SENTENCE_1)).isEqualTo(FRENCH_SENTENCE_1);
            assertThat(stringclass.echo(FRENCH_SENTENCE_2)).isEqualTo(FRENCH_SENTENCE_2);
        }
    }

    @Test
    public void LengthTest() throws UnsupportedEncodingException {
        assertThat(StringClass.getLength(ENGLISH_SENTENCE_1).intValue()).isEqualTo(ENGLISH_SENTENCE_1.getBytes(StandardCharsets.UTF_8).length);
        assertThat(StringClass.getLength(ENGLISH_SENTENCE_2).intValue()).isEqualTo(ENGLISH_SENTENCE_2.getBytes(StandardCharsets.UTF_8).length);
        assertThat(StringClass.getLength(FRENCH_SENTENCE_1).intValue()).isEqualTo(FRENCH_SENTENCE_1.getBytes(StandardCharsets.UTF_8).length);
        assertThat(StringClass.getLength(FRENCH_SENTENCE_2).intValue()).isEqualTo(FRENCH_SENTENCE_2.getBytes(StandardCharsets.UTF_8).length);
    }
}
