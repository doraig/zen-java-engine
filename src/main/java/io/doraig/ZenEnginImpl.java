package io.doraig;

import io.questdb.jar.jni.JarJniLoader;

import java.util.ArrayList;

class ZenEnginImpl  implements IZenEngine {

    String jniStatus;

    @Override
    public String evaluate(String request, String ruleFile) {
        return this.jniEvaluate(request, ruleFile);
    }

    static {

        JarJniLoader.loadLib(
                IZenEngine.class,
                "libs",
                "evaluate"
        );
    }

    private native ArrayList<String> testJniCall(ArrayList<Integer> list, int value);

    private native String jniEvaluate(String request, String ruleFile);

    public int javaAdd(int i, int u) {
        return i + u;
    }
}
