package io.stepfunc.foo_test;

import io.stepfunc.foo.DefaultInterfaceTest;
import io.stepfunc.foo.DefaultedInterface;
import io.stepfunc.foo.SwitchPosition;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.time.Duration;


class DefaultInterfaceMethodTest {

    static class DefaultInterfaceImpl implements DefaultedInterface {}

    @Test
    void InterfaceAndOneTimeCallbackTest() {
        DefaultedInterface di = new DefaultInterfaceImpl();

        DefaultInterfaceTest.invokeDoNothing(di);

        Assertions.assertTrue(DefaultInterfaceTest.getBoolValue(di));
        Assertions.assertEquals(42, DefaultInterfaceTest.getI32Value(di));
        Assertions.assertEquals(Duration.ofMillis(42), DefaultInterfaceTest.getDurationValue(di));
        Assertions.assertEquals(SwitchPosition.ON, DefaultInterfaceTest.getSwitchPos(di));
        Assertions.assertEquals(42, DefaultInterfaceTest.getWrappedNumber(di).num);
    }
}
