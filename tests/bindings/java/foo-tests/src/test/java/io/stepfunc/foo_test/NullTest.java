package io.stepfunc.foo_test;

import io.stepfunc.foo.*;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThatIllegalArgumentException;

public class NullTest {
    @Test
    public void NullUnsignedByte() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            IntegerEchoFunctions.uint8Echo(null);
        }).withMessage("value");
    }

    @Test
    public void NullUnsignedShort() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            IntegerEchoFunctions.uint16Echo(null);
        }).withMessage("value");
    }

    @Test
    public void NullUnsignedInteger() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            IntegerEchoFunctions.uint32Echo(null);
        }).withMessage("value");
    }

    @Test
    public void NullUnsignedLong() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            IntegerEchoFunctions.uint64Echo(null);
        }).withMessage("value");
    }

    @Test
    public void NullString() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            try(StringClass test = new StringClass()) {
                test.echo(null);
            }
        }).withMessage("value");
    }

    @Test
    public void NullEnum() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            EnumEchoFunctions.enumDisjointEcho(null);
        }).withMessage("value");
    }

    @Test
    public void NullCollection() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            StringCollection.getSize(null);
        }).withMessage("col");
    }

    @Test
    public void NullStruct() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            Structure.structByValueEcho(null);
        }).withMessage("value");
    }

    @Test
    public void NullStructElement() {
        Structure structure = StructureTest.createStructure();
        structure.interfaceValue = null;
        assertThatIllegalArgumentException().isThrownBy(() -> {
            Structure.structByValueEcho(structure);
        }).withMessage("value");
    }

    @Test
    public void NullInterface() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            try(CallbackSource source = new CallbackSource()) {
                source.setInterface(null);
            }
        }).withMessage("cb");
    }

    @Test
    public void NullStructSubStruct() {
        Structure structure = StructureTest.createStructure();
        structure.structureValue = null;
        assertThatIllegalArgumentException().isThrownBy(() -> {
            Structure.structByValueEcho(structure);
        }).withMessage("value");
    }

    @Test
    public void NullStructSubStructElement() {
        Structure structure = StructureTest.createStructure();
        structure.structureValue.test = null;
        assertThatIllegalArgumentException().isThrownBy(() -> {
            Structure.structByValueEcho(structure);
        }).withMessage("value");
    }

    @Test
    public void NullDurationMillis() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            DurationEchoFunctions.millisecondsEcho(null);
        }).withMessage("value");
    }

    @Test
    public void NullDurationSeconds() {
        assertThatIllegalArgumentException().isThrownBy(() -> {
            DurationEchoFunctions.secondsEcho(null);
        }).withMessage("value");
    }
}
