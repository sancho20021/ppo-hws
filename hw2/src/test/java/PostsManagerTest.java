import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;

import java.util.Arrays;
import java.util.List;

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
    public void getCompanyNamesWithGrowingPrice() {
        String tag = "итмо";
//        when(client.getInfo(symbols))
//                .thenReturn(createAnswer());
//
//        List<String> names = stockManager.getCompanyNamesWithGrowingPrice(symbols);
//        Assert.assertEquals(Arrays.asList("GOOG", "YNDX"), names);
    }
}
