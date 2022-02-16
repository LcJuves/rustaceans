import java.io.PrintStream;
import java.lang.reflect.Method;

/** Main */
class Main {

    private static void println(Object x) {
        System.out.println(x);
    }

    private static void eprintln(Object x) {
        System.err.println(x);
    }

    private static void pass() {
        System.out.print("\033[32;1m");
        System.out.print("PASS");
        System.out.print("\033[m");
        System.out.println();
    }

    private static <E> void assertType(E x, Class<?> clazz) {
        if (clazz != x.getClass()) {
            throw new AssertionError();
        }
    }

    private static void _assert(boolean condition) {
        if (!condition) {
            throw new AssertionError();
        }
    }

    public static void main(String[] args) {
        for (; ; ) {
            try {
                int version = CallJNI.getVersion();
                assertType(version, Integer.class);
                println(String.format("Version: %s", version));
                pass();

                Class<?> clazz = CallJNI.findClass("java/lang/String");
                assertType(clazz, String.class.getClass());
                println(clazz.getName());
                pass();

                Method method = String.class.getMethod("valueOf", boolean.class);
                String ret = CallJNI.fromReflectedMethod(method);
                _assert(ret.equals(String.valueOf(false)));

                PrintStream out = (PrintStream) CallJNI.getSystemOut();
                assertType(out, PrintStream.class);
                out.println("CallJNI: System.out >>>");
                pass();

                break;
            } catch (Throwable tr) {
                tr.printStackTrace();
                eprintln("FAILED");
                break;
            }
        }
    }
}
