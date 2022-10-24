import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.io.UncheckedIOException;

public class PostsClientIntegrationTest {
    private final String token = Utils.getTokenUnchecked();

    @Test
    public void getFrequencies() {
        PostsClient client = new PostsClient(token);
        int[] freqs = client.getFrequencies("#тест", 5);
        Assertions.assertEquals(freqs.length, 5);
    }
}
