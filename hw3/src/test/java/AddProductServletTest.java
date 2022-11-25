import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import ru.akirakozov.sd.refactoring.servlet.AddProductServlet;
import ru.akirakozov.sd.refactoring.servlet.GetProductsServlet;

import static org.mockito.Mockito.when;

public class AddProductServletTest extends ServletTest {
    private AddProductServlet addServlet;
    private GetProductsServlet getServlet;

    @Before
    public void initServlet() {
        addServlet = new AddProductServlet();
        getServlet = new GetProductsServlet();
    }

    @Test
    public void test1() throws Exception {
        when(request.getParameter("name")).thenReturn("watch");
        when(request.getParameter("price")).thenReturn("666");
        addServlet.doGet(request, response);
        Assert.assertEquals("OK\n", responseWriter.toString());
    }

    @Test
    public void test2AddAndGet() throws Exception {
        getServlet.doGet(request, response);

        Assert.assertEquals(
                "<html><body>\n" +
                        "macbook\t1000</br>\n" +
                        "iphone\t500</br>\n" +
                        "</body></html>\n",
                responseWriter.toString()
        );
        resetMocks();

        when(request.getParameter("name")).thenReturn("watch");
        when(request.getParameter("price")).thenReturn("666");
        addServlet.doGet(request, response);
        resetMocks();

        getServlet.doGet(request, response);
        Assert.assertEquals(
                "<html><body>\n" +
                        "macbook\t1000</br>\n" +
                        "iphone\t500</br>\n" +
                        "watch\t666</br>\n" +
                        "</body></html>\n",
                responseWriter.toString()
        );
    }
}
