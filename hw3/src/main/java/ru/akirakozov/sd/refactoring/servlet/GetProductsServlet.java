package ru.akirakozov.sd.refactoring.servlet;

import ru.akirakozov.sd.refactoring.DBManager;
import ru.akirakozov.sd.refactoring.HtmlBuilder;

import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.sql.SQLException;
import java.util.List;

/**
 * @author akirakozov
 */
public class GetProductsServlet extends HttpServlet {

    @Override
    public void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        try {
            List<DBManager.SqlRow> table = DBManager.executeQuery("SELECT * FROM PRODUCT");
            response.getWriter().println(buildNamePriceHtml(table));
        } catch (SQLException e) {
            throw new RuntimeException("Error while executing SQL: ", e);
        }

        response.setContentType("text/html");
        response.setStatus(HttpServletResponse.SC_OK);
    }

    private String buildNamePriceHtml(List<DBManager.SqlRow> products) {
        HtmlBuilder builder = new HtmlBuilder();
        for (DBManager.SqlRow rs : products) {
            builder.append(rs.get("name")).append("\t").append(rs.get("price")).println("</br>");
        }
        return HtmlBuilder.inBody(builder.toString());
    }
}
