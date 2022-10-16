import java.io.IOException;

public class PostsClientTest {
    public static void main(String[] args) throws IOException {
        var client = new PostsClient(Utils.getToken());
        client.getFrequencies("#путин", 2);
    }
}
