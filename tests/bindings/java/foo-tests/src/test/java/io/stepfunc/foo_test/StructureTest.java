package io.stepfunc.foo_test;

import io.stepfunc.foo.*;
import org.junit.jupiter.api.Test;

import java.time.Duration;

import static org.assertj.core.api.Assertions.assertThat;
import static org.joou.Unsigned.*;

public class StructureTest {
    static class TestInterface implements EmptyInterface {}

     private static void checkNumbersDefaults(Numbers x)
     {
        assertThat(x.uint8Value).isEqualTo(ubyte(1));
        assertThat(x.int8Value).isEqualTo((byte)-1);
        assertThat(x.uint16Value).isEqualTo(ushort(2));
        assertThat(x.int16Value).isEqualTo((short)-2);
        assertThat(x.uint32Value).isEqualTo(uint(3));
        assertThat(x.int32Value).isEqualTo(-3);
        assertThat(x.uint64Value).isEqualTo(ulong(4));
        assertThat(x.int64Value).isEqualTo(-4);
        assertThat(x.floatValue).isEqualTo(12.34f);
        assertThat(x.doubleValue).isEqualTo(-56.78);
    }

    private static void checkInnerStructure(InnerStructure x) {
        assertThat(x.interfaceField).isNotNull();
        checkNumbersDefaults(x.numbersField);
    }

    private static void checkStructure(Structure x) {
        assertThat(x.booleanTrue).isTrue();
        assertThat(x.booleanFalse).isFalse();
        assertThat(x.enumVar1).isEqualTo(StructureEnum.VAR1);
        assertThat(x.enumVar2).isEqualTo(StructureEnum.VAR2);
        assertThat(x.durationMillis).isEqualTo(Duration.ofMillis(4200));
        assertThat(x.durationSeconds).isEqualTo(Duration.ofSeconds(76));
        assertThat(x.stringHello).isEqualTo("Hello");
        checkInnerStructure(x.innerStructure);
    }

    @Test
    public void testStructureConstructor() {
        Structure x = new Structure(new InnerStructure(new TestInterface()));
        checkStructure(x);
    }

    @Test
    public void testStructureBuilderMethods() {
        Structure x = new Structure(new InnerStructure(new TestInterface()))
            .withBooleanFalse(true) // introducing some chaos
            .withBooleanTrue(false);

        assertThat(x.booleanTrue).isFalse();
        assertThat(x.booleanFalse).isTrue();
    }
}
