// TODO Reviewed ??-??-2026.

package com.example;

public class HelloWorld {
    private static native String hello(String input);

    static {
        System.loadLibrary("mylib");
    }

    public static void main(String[] args) {
        System.out.println(HelloWorld.hello("World"));
    }
}
