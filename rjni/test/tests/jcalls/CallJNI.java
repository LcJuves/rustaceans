/**
 * Created at 2021/7/25 15:43
 *
 * @author Liangcheng Juves
 */
import java.io.PrintStream;
import java.lang.reflect.Field;
import java.lang.reflect.Method;

class CallJNI {

    static final String loadStatus = null;

    static {
        System.loadLibrary("rjnit");
    }

    static native int getVersion();

    static native Class<?> defineClass(String name, ClassLoader loader, byte[] buf, int len);

    static native Class<?> findClass(String name);

    static native String fromReflectedMethod(Method method);

    static native void fromReflectedField(Field field);

    static native PrintStream getSystemOut();
}
