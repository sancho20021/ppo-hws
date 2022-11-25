import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import ru.akirakozov.sd.refactoring.servlet.QueryServlet;

import java.io.IOException;

import static org.mockito.Mockito.when;


public class QueryServletTest extends ServletTest {
    private QueryServlet queryServlet;

    @Before
    public void initServlet() {
        queryServlet = new QueryServlet();
    }

    @Test
    public void testMax() throws IOException {
        when(request.getParameter("command")).thenReturn("max");
        queryServlet.doGet(request, response);
        Assert.assertEquals(
                "<html><body>\n" +
                        "<h1>Product with max price: </h1>\n" +
                        "macbook\t1000</br>\n" +
                        "</body></html>\n",
                responseWriter.toString()
        );
    }

    @Test
    public void testMin() throws IOException {
        when(request.getParameter("command")).thenReturn("min");
        queryServlet.doGet(request, response);
        Assert.assertEquals(
                "<html><body>\n" +
                        "<h1>Product with min price: </h1>\n" +
                        "iphone\t500</br>\n" +
                        "</body></html>\n",
                responseWriter.toString()
        );
    }

    @Test
    public void testSum() throws IOException {
        when(request.getParameter("command")).thenReturn("sum");
        queryServlet.doGet(request, response);
        Assert.assertEquals(
                "<html><body>\n" +
                        "Summary price: \n" +
                        "1500\n" +
                        "</body></html>\n",
                responseWriter.toString()
        );
    }

    @Test
    public void testCount() throws IOException {
        when(request.getParameter("command")).thenReturn("count");
        queryServlet.doGet(request, response);
        Assert.assertEquals(
                "<html><body>\n" +
                        "Number of products: \n" +
                        "2\n" +
                        "</body></html>\n",
                responseWriter.toString()
        );
    }
}
