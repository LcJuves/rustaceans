import java.io.IOException;
import java.io.PrintStream;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

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
        long startTimeMillis = System.currentTimeMillis();
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
                        "defineClass",
                        () -> {
                            try {
                                byte[] definedClassBytes =
                                        Files.readAllBytes(Paths.get("DefinedClass.class"));
                                Class<?> clazz =
                                        CallJNI.defineClass("DefinedClass", definedClassBytes);
                                Method method = clazz.getMethod("getClassSimpleName");
                                _assert(((String) method.invoke(clazz)).equals("DefinedClass"));
                            } catch (Exception e) {
                                throw new AssertionError(e);
                            }
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
                                PrintStream ret = CallJNI.fromReflectedField(field);
                                _assert(ret.equals(System.out));
                            } catch (Exception e) {
                                throw new AssertionError(e);
                            }
                        });

                test(
                        "ToReflectedMethod",
                        () -> {
                            try {
                                Method method = CallJNI.toReflectedMethod();
                                _assert(
                                        String.class
                                                .cast(method.invoke(String.class, false))
                                                .equals(String.valueOf(false)));
                            } catch (Exception e) {
                                throw new AssertionError(e);
                            }
                        });

                test(
                        "GetSuperclass",
                        () -> {
                            try {
                                Class<?> clazz = CallJNI.getSuperclass(String.class);
                                _assert(clazz == Object.class);
                            } catch (Exception e) {
                                throw new AssertionError(e);
                            }
                        });

                test(
                        "IsAssignableFrom",
                        () -> {
                            try {
                                boolean canBeCast =
                                        CallJNI.isAssignableFrom(ArrayList.class, List.class);
                                _assert(canBeCast);
                            } catch (Exception e) {
                                throw new AssertionError(e);
                            }
                        });

                test(
                        "ToReflectedField",
                        () -> {
                            try {
                                Field field = CallJNI.toReflectedField();
                                _assert(
                                        PrintStream.class
                                                .cast(field.get(System.class))
                                                .equals(System.out));
                            } catch (Exception e) {
                                throw new AssertionError(e);
                            }
                        });

                test(
                        "Throw",
                        () -> {
                            try {
                                _assert(CallJNI.$_throw(new IOException()) == 0);
                            } catch (Exception e) {
                                _assert(e.getClass() == IOException.class);
                            }
                        });

                test(
                        "ThrowNew",
                        () -> {
                            try {
                                _assert(CallJNI.throwNew(RuntimeException.class, "JNICALL") == 0);
                            } catch (Exception e) {
                                _assert(e.getClass() == RuntimeException.class);
                                _assert(e.getMessage().equals("JNICALL"));
                            }
                        });

                test(
                        "ExceptionOccurred",
                        () -> {
                            Throwable throwable = CallJNI.exceptionOccurred();
                            _assert(throwable.getClass() == RuntimeException.class);
                            _assert(throwable.getMessage().equals("JNICALL"));
                        });

                test(
                        "ExceptionDescribe",
                        () -> {
                            try {
                                CallJNI.exceptionDescribe();
                            } catch (Exception e) {
                                String stackTrace0 = e.getStackTrace()[0].toString();
                                _assert(
                                        stackTrace0.contains(
                                                "Exception in thread \"main\" java.lang.RuntimeException: JNICALL"));
                            }
                        });

                test(
                        "ExceptionClear",
                        () -> {
                            try {
                                CallJNI.exceptionClear();
                            } catch (Exception e) {
                                _assert(e == null);
                            }
                        });

                test(
                        "FatalError",
                        () -> {
                            try {
                                // TODO
                                // CallJNI.fatalError("JNICALL");
                            } catch (Throwable tr) {

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
        long endTimeMillis = System.currentTimeMillis();
        System.out.println(endTimeMillis - startTimeMillis);
    }
}
