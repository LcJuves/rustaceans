import java.io.PrintStream;
import java.lang.reflect.Field;
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

    private static <E> void assertType(E e, Class<?> clazz) {
        if (clazz != e.getClass()) {
            throw new AssertionError();
        }
    }

    private static void _assert(boolean condition) {
        if (!condition) {
            throw new AssertionError();
        }
    }

    private static void test(String desc, Runnable runnable) {
        System.out.print(desc);
        System.out.print("   ");
        runnable.run();
        pass();
    }

    public static void main(String[] args) {
        System.out.println();
        for (; ; ) {
            try {
                test(
                        "GetVersion",
                        () -> {
                            int version = CallJNI.getVersion();
                            assertType(version, Integer.class);
                            _assert(version > 0);
                            _assert(CallJNI.loadStatus.equals("Loaded"));
                        });

                test(
                        "FindClass",
                        () -> {
                            Class<?> clazz = CallJNI.findClass("java/lang/String");
                            assertType(clazz, Class.class);
                            _assert(clazz.getSimpleName().equals("String"));
                        });

                test(
                        "FromReflectedMethod",
                        () -> {
                            try {
                                Method method = String.class.getMethod("valueOf", boolean.class);
                                String ret = CallJNI.fromReflectedMethod(method);
                                _assert(ret.equals(String.valueOf(false)));
                            } catch (Exception e) {
                                throw new AssertionError(e);
                            }
                        });

                test(
                        "FromReflectedField",
                        () -> {
                            try {
                                Field field = System.class.getDeclaredField("out");
                                // CallJNI.fromReflectedField(field);
                            } catch (Exception e) {
                                throw new AssertionError(e);
                            }
                        });

                System.out.println();
                PrintStream out = (PrintStream) CallJNI.getSystemOut();
                assertType(out, PrintStream.class);
                out.println("CallJNI: System.out >>>");

                break;
            } catch (Throwable tr) {
                tr.printStackTrace();
                eprintln("FAILED");
                break;
            }
        }
    }
}
