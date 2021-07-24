class CallJNI {

  static {
    System.loadLibrary("rjni");
  }

  static native int getVersion();

  static native Class<?> defineClass(String name, ClassLoader loader, byte[] buf, int len);

  static native Class<?> findClass(String name);
}
