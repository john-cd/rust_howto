// TODO Reviewed ??-??-2026.

// ANCHOR: example
package com.example;

public class HelloWorld {
    private static native String hello(String input);

    static {
        System.loadLibrary("jni_greeting");
    }

    public static void main(String[] args) {
        String actual = hello("Zoë 🌱");
        if (!"Hello, Zoë 🌱!".equals(actual)) {
            throw new AssertionError("Unexpected greeting: " + actual);
        }
        System.out.println(actual);
    }
}
// ANCHOR_END: example
