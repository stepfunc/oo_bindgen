package io.stepfunc.foo_test;

import io.stepfunc.foo.*;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import static org.joou.Unsigned.*;

class OpaqueStructTest {
    @Test
    void OpaqueStructureCanRoundTripValues() {

        Assertions.assertEquals(ulong(42), OpaqueStructHelpers.getId(OpaqueStructHelpers.createMagicValue()));
    }
}
