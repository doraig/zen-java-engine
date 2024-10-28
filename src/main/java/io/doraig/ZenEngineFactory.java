package io.doraig;

@SuppressWarnings("unused")
public class ZenEngineFactory {

    @SuppressWarnings("unused")
    public static IZenEngine getEngine() {
        return new ZenEnginImpl();
    }
}
