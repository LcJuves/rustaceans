/**
 * Created at 2021/7/25 15:43
 *
 * @author Liangcheng Juves
 */
import java.io.PrintStream;

class CallJNI {

  static {
    System.loadLibrary("rjnit");
    System.out.println();
  }

  static native int getVersion();

  static native Class<?> defineClass(String name, ClassLoader loader, byte[] buf, int len);

  static native Class<?> findClass(String name);

  static native PrintStream getSystemOut();
}
