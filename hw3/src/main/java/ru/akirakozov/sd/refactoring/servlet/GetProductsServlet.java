package ru.akirakozov.sd.refactoring.servlet;

import ru.akirakozov.sd.refactoring.DBManager;

import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.List;

/**
 * @author akirakozov
 */
public class GetProductsServlet extends HttpServlet {

    @Override
    public void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        try {
            List<DBManager.SqlRow> table = DBManager.executeQuery("SELECT * FROM PRODUCT");

            response.getWriter().println("<html><body>");
            for (DBManager.SqlRow rs : table) {
                String name = (String) rs.get("name");
                int price = (Integer) rs.get("price");
                response.getWriter().println(name + "\t" + price + "</br>");
            }
            response.getWriter().println("</body></html>");

        } catch (Exception e) {
            throw new RuntimeException(e);
        }

        response.setContentType("text/html");
        response.setStatus(HttpServletResponse.SC_OK);
    }
}
