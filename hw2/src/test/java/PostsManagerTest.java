import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;

import static org.mockito.Mockito.when;

public class PostsManagerTest {
    private PostsManager postsManager;

    @Mock
    private PostsClient client;

    @BeforeEach
    public void setup() {
        MockitoAnnotations.openMocks(this);
        postsManager = new PostsManager(client);
    }

    @Test
    public void getFrequencies() {
        String tag = "#итмо";
        int hours = 5;

        when(client.getFrequencies(tag, hours)).thenReturn(createAnswer(hours));

        int[] freqs = postsManager.getFrequencies(tag, hours);
        Assertions.assertEquals(freqs.length, hours);
    }

    private int[] createAnswer(int length) {
        return new int[length];
    }
}
