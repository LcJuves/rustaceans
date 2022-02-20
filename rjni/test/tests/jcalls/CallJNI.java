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

    static native Class<?> defineClass(String name, byte[] bytes);

    static native Class<?> findClass(String name);

    static native String fromReflectedMethod(Method method);

    static native PrintStream fromReflectedField(Field field);

    static native Method toReflectedMethod();

    static native Class<?> getSuperclass(Class<?> clazz);

    static native boolean isAssignableFrom(Class<?> clazz1, Class<?> clazz2);

    static native Field toReflectedField();

    static native PrintStream getSystemOut();
}
