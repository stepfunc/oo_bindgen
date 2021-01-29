package io.stepfunc.foo_test;

import io.stepfunc.foo.OtherStructure;
import io.stepfunc.foo.Structure;
import io.stepfunc.foo.StructureEnum;
import io.stepfunc.foo.StructureInterface;
import org.junit.jupiter.api.Test;

import java.time.Duration;

import static org.assertj.core.api.Assertions.assertThat;
import static org.joou.Unsigned.*;

public class StructureTest {
    @Test
    public void StructureByValueEchoTest() {
        Structure value = createStructure();
        Structure result = Structure.structByValueEcho(value);
        checkStructure(result);
    }

    @Test
    public void StructureByReferenceEchoTest() {
        Structure value = createStructure();
        Structure result = value.structByReferenceEcho();
        checkStructure(result);
    }

    @Test
    public void InterfaceStruct() {
        Structure value = createStructure();
        TestInterface testInterface = new TestInterface();
        value.interfaceValue = testInterface;

        Structure result = Structure.structByValueEcho(value);

        assertThat(result.interfaceValue).isNotNull();
        checkStructure(testInterface.lastValue);
    }

    static class TestInterface implements StructureInterface {
        public Structure lastValue = null;

        @Override
        public void onValue(Structure value) {
            lastValue = value;
        }
    }

    public static Structure createStructure() {
        return new Structure(new TestInterface());
    }

    private static void checkStructure(Structure structure) {
        assertThat(structure.booleanValue).isTrue();
        assertThat(structure.uint8Value).isEqualTo(ubyte(1));
        assertThat(structure.int8Value).isEqualTo((byte)-1);
        assertThat(structure.uint16Value).isEqualTo(ushort(2));
        assertThat(structure.int16Value).isEqualTo((short)-2);
        assertThat(structure.uint32Value).isEqualTo(uint(3));
        assertThat(structure.int32Value).isEqualTo(-3);
        assertThat(structure.uint64Value).isEqualTo(ulong(4));
        assertThat(structure.int64Value).isEqualTo(-4);
        assertThat(structure.floatValue).isEqualTo(12.34f);
        assertThat(structure.doubleValue).isEqualTo(-56.78);
        assertThat(structure.stringValue).isEqualTo("Hello");

        assertThat(structure.structureValue.test).isEqualTo(ushort(41));
        assertThat(structure.enumValue).isEqualTo(StructureEnum.VAR2);

        assertThat(structure.durationMillis).isEqualTo(Duration.ofMillis(4200));
        assertThat(structure.durationSeconds).isEqualTo(Duration.ofSeconds(76));
        assertThat(structure.durationSecondsFloat).isEqualTo(Duration.ofSeconds(15).plusMillis(250));
    }
}
