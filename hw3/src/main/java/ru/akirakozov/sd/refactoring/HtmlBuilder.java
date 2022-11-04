package ru.akirakozov.sd.refactoring;

public class HtmlBuilder {
    private final StringBuilder s = new StringBuilder();

    public HtmlBuilder append(Object str) {
        s.append(str);
        return this;
    }

    public HtmlBuilder println(String str) {
        return append(str).append("\n");
    }

    @Override
    public String toString() {
        return s.toString();
    }

    public static String inBody(String text) {
        return "<html><body>\n" + text + "</body></html>";
    }
}
