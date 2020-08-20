package io.stepfunc.foo_test;

import io.stepfunc.foo.OtherStructure;
import io.stepfunc.foo.Structure;
import io.stepfunc.foo.StructureEnum;
import io.stepfunc.foo.StructureInterface;
import jdk.nashorn.internal.ir.annotations.Ignore;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Disabled;

import java.time.Duration;

import static org.assertj.core.api.Assertions.assertThat;

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
    @Disabled // Because of the way we handle interfaces atm, it will return null
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

    private static Structure createStructure() {
        Structure structure = new Structure();

        structure.booleanValue = true;
        structure.uint8Value = 1;
        structure.int8Value = -1;
        structure.uint16Value = 2;
        structure.int16Value = -2;
        structure.uint32Value = 3;
        structure.int32Value = -3;
        structure.uint64Value = 4L;
        structure.int64Value = -4L;
        structure.floatValue = 12.34f;
        structure.doubleValue = -56.78;

        structure.structureValue = new OtherStructure();
        structure.structureValue.test = 41;

        structure.enumValue = StructureEnum.VAR2;

        structure.interfaceValue = new TestInterface();

        structure.durationMillis = Duration.ofMillis(4200);
        structure.durationSeconds = Duration.ofSeconds(76);
        structure.durationSecondsFloat = Duration.ofSeconds(15).plusMillis(250);

        return structure;
    }

    private static void checkStructure(Structure structure) {
        assertThat(structure.booleanValue).isTrue();
        assertThat(structure.uint8Value).isEqualTo((byte)1);
        assertThat(structure.int8Value).isEqualTo((byte)-1);
        assertThat(structure.uint16Value).isEqualTo((short)2);
        assertThat(structure.int16Value).isEqualTo((short)-2);
        assertThat(structure.uint32Value).isEqualTo(3);
        assertThat(structure.int32Value).isEqualTo(-3);
        assertThat(structure.uint64Value).isEqualTo(4);
        assertThat(structure.int64Value).isEqualTo(-4);
        assertThat(structure.floatValue).isEqualTo(12.34f);
        assertThat(structure.doubleValue).isEqualTo(-56.78);

        assertThat(structure.structureValue.test).isEqualTo((short)41);
        assertThat(structure.enumValue).isEqualTo(StructureEnum.VAR2);

        assertThat(structure.durationMillis).isEqualTo(Duration.ofMillis(4200));
        assertThat(structure.durationSeconds).isEqualTo(Duration.ofSeconds(76));
        assertThat(structure.durationSecondsFloat).isEqualTo(Duration.ofSeconds(15).plusMillis(250));
    }
}
