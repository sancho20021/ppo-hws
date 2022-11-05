import org.junit.Before;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.io.PrintWriter;
import java.io.StringWriter;

import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

public class ServletTest {
    protected HttpServletRequest request;
    protected HttpServletResponse response;
    protected StringWriter responseWriter;

    @Before
    public void init() throws IOException {
        Common.resetDB();
        resetMocks();
    }

    public void resetMocks() throws IOException {
        request = mock(HttpServletRequest.class);
        response = mock(HttpServletResponse.class);
        responseWriter = new StringWriter();
        when(response.getWriter()).thenReturn(new PrintWriter(responseWriter));
    }
}
