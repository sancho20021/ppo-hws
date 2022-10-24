import com.github.tomakehurst.wiremock.junit5.WireMockTest;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.net.http.HttpResponse;
import java.util.List;
import java.util.Map;

import static com.github.tomakehurst.wiremock.client.WireMock.*;

@WireMockTest(httpPort = 32453, httpsEnabled = true)
public class RequestSenderStubTest {
    private final RequestSender requestSender = new RequestSender("localhost", 32453);


    @Test
    public void sendRequest() {
        stubFor(get(urlEqualTo("/ping"))
                .willReturn(aResponse()
                        .withHeader("Content-Type", "text/plain")
                        .withBody("pong")));
        List<HttpResponse<String>> result = requestSender.sendRequests(
                List.of(new RequestSender.GetRequest("/ping", Map.of()))
        );
        Assertions.assertEquals(result.get(0).body(), "pong");
    }
}
