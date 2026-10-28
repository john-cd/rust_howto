// TODO Reviewed ??-??-2026.

package com.example;

public class NativeLibrary {
    private static native String greet(String name);

    static {
        System.loadLibrary("mylib");
    }

    public static void main(String[] args) {
        System.out.println(NativeLibrary.greet("World"));
    }
}
