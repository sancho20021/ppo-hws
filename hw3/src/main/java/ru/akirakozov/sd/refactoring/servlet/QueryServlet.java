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
public class QueryServlet extends HttpServlet {
    @Override
    public void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        String command = request.getParameter("command");

        if ("max".equals(command)) {
            try {
                List<DBManager.SqlRow> table = DBManager.executeQuery("SELECT * FROM PRODUCT ORDER BY PRICE DESC LIMIT 1");
                response.getWriter().println("<html><body>");
                response.getWriter().println("<h1>Product with max price: </h1>");

                for (DBManager.SqlRow row : table) {
                    String name = (String) row.get("name");
                    int price = (Integer) row.get("price");
                    response.getWriter().println(name + "\t" + price + "</br>");
                }
                response.getWriter().println("</body></html>");

            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        } else if ("min".equals(command)) {
            try {
                List<DBManager.SqlRow> table = DBManager.executeQuery("SELECT * FROM PRODUCT ORDER BY PRICE LIMIT 1");
                response.getWriter().println("<html><body>");
                response.getWriter().println("<h1>Product with min price: </h1>");

                for (DBManager.SqlRow rs : table) {
                    String name = (String) rs.get("name");
                    int price = (Integer) rs.get("price");
                    response.getWriter().println(name + "\t" + price + "</br>");
                }
                response.getWriter().println("</body></html>");
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        } else if ("sum".equals(command)) {
            try {
                List<DBManager.SqlRow> table = DBManager.executeQuery("SELECT SUM(price) as SUM FROM PRODUCT");

                response.getWriter().println("<html><body>");
                response.getWriter().println("Summary price: ");

                if (!table.isEmpty()) {
                    response.getWriter().println((Integer) table.get(0).get("SUM"));
                }
                response.getWriter().println("</body></html>");
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        } else if ("count".equals(command)) {
            try {
                List<DBManager.SqlRow> table = DBManager.executeQuery("SELECT COUNT(*) as CNT FROM PRODUCT");
                response.getWriter().println("<html><body>");
                response.getWriter().println("Number of products: ");

                if (!table.isEmpty()) {
                    response.getWriter().println((Integer) table.get(0).get("CNT"));
                }
                response.getWriter().println("</body></html>");
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        } else {
            response.getWriter().println("Unknown command: " + command);
        }

        response.setContentType("text/html");
        response.setStatus(HttpServletResponse.SC_OK);
    }

}
