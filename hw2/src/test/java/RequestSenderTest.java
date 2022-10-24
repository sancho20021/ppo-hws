import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Map;

class RequestSenderTest {
    @Test
    public void read() {
        var host = "api.vk.com";
        var port = 443;
        var sender = new RequestSender(host, port);
        var route = "/method/newsfeed.search/";
        var params = Map.of(
                "access_token", Utils.getTokenUnchecked(),
                "v", "5.131",
                "q", "test"
        );
        var result = sender.sendRequests(List.of(
                new RequestSender.GetRequest(route, params)
        ));
        Assertions.assertNotNull(result.get(0));
    }
}