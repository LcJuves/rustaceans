import static java.io.File.separator;

import java.io.File;

class HelloWorld {
  // This declares that the static `hello` method will be provided
  // a native library.
  private static native String hello(String input);

  private static final boolean DEBUG = false;

  static {
    // This actually loads the shared object that we'll be creating.
    // The actual location of the .so or .dll may differ based on your
    // platform.
    addLibraryPath();
    System.loadLibrary("rust_jni");
  }

  private static void addLibraryPath() {
    String path =
        new File("").getAbsolutePath()
            + separator
            + "target"
            + separator
            + (DEBUG ? "debug" : "release");
    Platform.addLibraryPath(path);
  }

  // The rest is just regular ol' Java!
  public static void main(String[] args) {
    String output = HelloWorld.hello("josh");
    System.out.println(output);
  }
}
