import org.apache.hc.core5.net.URIBuilder;

import java.io.IOException;
import java.net.URISyntaxException;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class RequestSender {
    private final String host;
    private final int port;

    public RequestSender(String host, int port) {
        this.host = host;
        this.port = port;
    }

    public static class GetRequest {
        public final String route;
        public final Map<String, String> params;

        public GetRequest(String route, Map<String, String> params) {
            this.route = route;
            this.params = params;
        }
    }

    public static List<GetRequest> bunchRequests(
            String route,
            Map<String, String> params,
            List<Map<String, String>> changingParams
    ) {
        return changingParams.stream().map(p -> {
            var totalParams = Stream.concat(params.entrySet().stream(), p.entrySet().stream())
                    .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
            return new GetRequest(route, totalParams);
        }).collect(Collectors.toList());
    }

    public List<HttpResponse<String>> sendRequests(List<GetRequest> requests) {
        HttpClient client = HttpClient.newHttpClient();
        var httpRequests = requests.stream().map(req -> {
            try {
                var uriBuilder = new URIBuilder("https://" + host + ":" + port + req.route);
                for (var kv : req.params.entrySet()) {
                    uriBuilder.addParameter(kv.getKey(), kv.getValue());
                }
                var uri = uriBuilder.build();
                return HttpRequest.newBuilder().uri(uri).GET().build();
            } catch (URISyntaxException e) {
                throw new RuntimeException("error while building uri: ", e);
            }
        }).collect(Collectors.toList());

        return httpRequests.stream().map(req -> {
            try {
                var ans = client.send(req, HttpResponse.BodyHandlers.ofString());
                return ans;
            } catch (IOException | InterruptedException e) {
                throw new RuntimeException(e);
            }
        }).collect(Collectors.toList());
    }
}
