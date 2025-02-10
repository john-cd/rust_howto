public class NativeLibrary {
    public native String greet(String name);

    static {
        System.loadLibrary("native-library"); // Replace "native-library" with the actual library name
    }

    public static void main(String[] args) {
        NativeLibrary nativeLibrary = new NativeLibrary();
        String greeting = nativeLibrary.greet("World");
        System.out.println(greeting); // Output: Hello, World!
    }
}
