import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import ru.akirakozov.sd.refactoring.servlet.GetProductsServlet;

import java.io.IOException;

public class GetProductServletTest extends ServletTest {
    private GetProductsServlet servlet;

    @Before
    public void initServlet() {
        servlet = new GetProductsServlet();
    }

    @Test
    public void test1() throws IOException {
        servlet.doGet(request, response);

        Assert.assertEquals(
                "<html><body>\n" +
                        "macbook\t1000</br>\n" +
                        "iphone\t500</br>\n" +
                        "</body></html>\n",
                responseWriter.toString()
        );
    }
}
