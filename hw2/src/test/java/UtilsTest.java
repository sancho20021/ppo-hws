import org.junit.jupiter.api.Test;

import java.io.IOException;

import static org.junit.jupiter.api.Assertions.assertNotNull;

class UtilsTest {
    @Test
    public void testTokenExists() throws IOException {
        assertNotNull(Utils.getToken());
    }
}