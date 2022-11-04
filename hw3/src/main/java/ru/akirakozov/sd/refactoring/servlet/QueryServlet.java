package ru.akirakozov.sd.refactoring.servlet;

import ru.akirakozov.sd.refactoring.DBManager;
import ru.akirakozov.sd.refactoring.HtmlBuilder;

import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.List;
import java.util.Optional;

/**
 * @author akirakozov
 */
public class QueryServlet extends HttpServlet {

    @Override
    public void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        String command = request.getParameter("command");

        if ("max".equals(command)) {
            try {
                List<DBManager.SqlRow> table = DBManager.executeQuery("SELECT * FROM PRODUCT ORDER BY PRICE DESC LIMIT 1");
                response.getWriter().println(buildNamePriceWithHeaderHtml(table, "Product with max price: "));
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        } else if ("min".equals(command)) {
            try {
                List<DBManager.SqlRow> table = DBManager.executeQuery("SELECT * FROM PRODUCT ORDER BY PRICE LIMIT 1");
                response.getWriter().println(buildNamePriceWithHeaderHtml(table, "Product with min price: "));
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        } else if ("sum".equals(command)) {
            try {
                List<DBManager.SqlRow> table = DBManager.executeQuery("SELECT SUM(price) as SUM FROM PRODUCT");
                String html = buildKeyOValueHtml(
                        "Summary price", table.isEmpty()
                                ? Optional.empty()
                                : Optional.of(table.get(0).get("SUM").toString())
                );
                response.getWriter().println(html);
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        } else if ("count".equals(command)) {
            try {
                List<DBManager.SqlRow> table = DBManager.executeQuery("SELECT COUNT(*) as CNT FROM PRODUCT");
                String html = buildKeyOValueHtml(
                        "Number of products",
                        table.isEmpty()
                                ? Optional.empty()
                                : Optional.of(table.get(0).get("CNT").toString())
                );
                response.getWriter().println(html);
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        } else {
            response.getWriter().println("Unknown command: " + command);
        }

        response.setContentType("text/html");
        response.setStatus(HttpServletResponse.SC_OK);
    }

    private String buildNamePriceWithHeaderHtml(List<DBManager.SqlRow> products, String header) {
        HtmlBuilder builder = new HtmlBuilder();
        builder.append("<h1>").append(header).println("</h1>");
        for (DBManager.SqlRow rs : products) {
            builder.append(rs.get("name")).append("\t").append(rs.get("price")).println("</br>");
        }
        return HtmlBuilder.inBody(builder.toString());
    }

    private String buildKeyOValueHtml(String key, Optional<String> value) {
        HtmlBuilder builder = new HtmlBuilder();
        builder.append(key).println(": ");
        value.ifPresent(builder::println);
        return HtmlBuilder.inBody(builder.toString());
    }

}
