import com.google.gson.JsonParser;

import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class PostsClient {
    private final String token;
    private final String apiVersion = "5.131";

    public PostsClient(String token) {
        this.token = token;
    }

    public int[] getFrequencies(String tag, int hours) {
        var requests = RequestSender.bunchRequests(
                "/method/newsfeed.search/",
                Map.of("access_token", token,
                        "v", apiVersion,
                        "q", URLEncoder.encode(tag, StandardCharsets.UTF_8)
                ),
                IntStream.range(1, hours + 1)
                        .mapToObj(hb -> {
                            var hourRange = getHourRange(hb);
                            return Map.of("start_time", Long.toString(hourRange[0]),
                                    "end_time", Long.toString(hourRange[1]));
                        })
                        .collect(Collectors.toList())
        );
        var responses = new RequestSender("api.vk.com", 443, true).sendRequests(requests);
        return responses.stream().mapToInt(response ->
                JsonParser.parseString(response.body())
                        .getAsJsonObject().get("response")
                        .getAsJsonObject().get("count").getAsInt()
        ).toArray();
    }

    private long[] getHourRange(int hoursBefore) {
        long now = System.currentTimeMillis() / 1000;
        long secondsInHour = 60 * 60;
        long start = now - (long) hoursBefore * secondsInHour;
        return new long[]{start, start + secondsInHour};
    }
}
