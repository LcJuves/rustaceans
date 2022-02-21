public final class FatalErrorTest {
    private FatalErrorTest() {}

    public static void main(String[] args) {
        Main.test("FatalError", () -> CallJNI.fatalError("JNICALL"));
    }
}
