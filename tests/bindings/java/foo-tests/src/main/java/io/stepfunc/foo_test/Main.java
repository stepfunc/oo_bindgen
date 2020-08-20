package io.stepfunc.foo_test;

import io.stepfunc.foo.CallbackInterface;
import io.stepfunc.foo.CallbackSource;
import io.stepfunc.foo.OneTimeCallbackInterface;

import java.time.Duration;

class Asdf implements CallbackInterface {

    @Override
    public void onValue(Integer value) {
        System.out.println(value);
    }

    @Override
    public void onDuration(Duration value) {
        System.out.println(value);
    }
}

class Qwerty implements OneTimeCallbackInterface {

    @Override
    public void onValue(Integer value) {
        System.out.println(value);
    }
}

public class Main {
    public static void main(String[] args) {
        System.out.println("Hello world!");
        CallbackSource source = new CallbackSource();
        source.addFunc(new Asdf());
        source.setValue(42);
        source.setValue(43);
        source.setValue(44);
        source.setDuration(Duration.ofSeconds(10));

        source.addOneTimeFunc(new Qwerty());
    }
}
