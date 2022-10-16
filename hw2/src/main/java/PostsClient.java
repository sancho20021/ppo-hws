import org.apache.commons.io.IOUtils;
import org.apache.hc.client5.http.classic.methods.HttpGet;
import org.apache.hc.client5.http.impl.classic.CloseableHttpClient;
import org.apache.hc.client5.http.impl.classic.HttpClients;
import org.apache.hc.core5.net.URIBuilder;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.net.URI;
import java.net.URISyntaxException;
import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class PostsClient {
    private final String token;
    private final String apiVersion = "5.131";

    public PostsClient(String token) {
        this.token = token;
    }

    public int[] getFrequencies(String tag, int hours) {
        try (final CloseableHttpClient client = HttpClients.createDefault()) {
            var requests = IntStream.range(1, hours + 1)
                    .mapToObj(hb -> getRequest(tag, hb))
                    .collect(Collectors.toList());
            var responses = requests.stream().map(req -> {
                try {
                    return client.execute(req).getEntity().getContent();
                } catch (IOException e) {
                    throw new UncheckedIOException(e);
                }
            }).collect(Collectors.toList());
            for (var response : responses) {
                var s = IOUtils.toString(response, StandardCharsets.UTF_8);
                System.out.println(s);
            }
        } catch (IOException e) {
            throw new UncheckedIOException(e);
        }
        return null;
    }

    private HttpGet getRequest(String tag, int hoursBefore) {
        try {
            long[] hourRange = getHourRange(hoursBefore);
            URI uri = new URIBuilder("https://api.vk.com:443/method/newsfeed.search/")
                    .addParameter("access_token", token)
                    .addParameter("v", apiVersion)
                    .addParameter("q", URLEncoder.encode(tag, StandardCharsets.UTF_8))
                    .addParameter("start_time", Long.toString(hourRange[0]))
                    .addParameter("end_time", Long.toString(hourRange[1]))
                    .build();
            HttpGet httpGet = new HttpGet("https://api.vk.com");
            httpGet.setUri(uri);
            return httpGet;
        } catch (URISyntaxException e) {
            throw new RuntimeException(e);
        }
    }

    private long[] getHourRange(int hoursBefore) {
        long now = System.currentTimeMillis() / 1000;
        long secondsInHour = 60 * 60;
        long start = now - (long) hoursBefore * secondsInHour;
        System.out.println(start);
        System.out.println(start + secondsInHour);
        return new long[]{start, start + secondsInHour};
    }
}
