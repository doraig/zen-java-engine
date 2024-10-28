package io.doraig;

public class ZenEngineFactory {

    public static IZenEngine getEngine() {
        return new ZenEnginImpl();
    }
}
