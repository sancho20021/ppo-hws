package ru.akirakozov.sd.refactoring.servlet;

import ru.akirakozov.sd.refactoring.DBManager;

import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.sql.SQLException;

/**
 * @author akirakozov
 */
public class AddProductServlet extends HttpServlet {

    @Override
    public void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        String name = request.getParameter("name");
        long price = Long.parseLong(request.getParameter("price"));

        try {
            String sql = "INSERT INTO PRODUCT " +
                    "(NAME, PRICE) VALUES (\"" + name + "\"," + price + ")";
            DBManager.executeUpdate(sql);
        } catch (SQLException e) {
            throw new RuntimeException("Error while executing SQL: ", e);
        }

        response.setContentType("text/html");
        response.setStatus(HttpServletResponse.SC_OK);
        response.getWriter().println("OK");
    }
}
