public class PostsManager {
    private final PostsClient client;

    public int[] getFrequencies(String tag, int hours) {
        return client.getFrequencies(tag, hours);
    }

    public PostsManager(PostsClient client) {
        this.client = client;
    }
}
