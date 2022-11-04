import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertNotNull;

class UtilsTest {
    @Test
    public void testTokenExists() {
        assertNotNull(Utils.getTokenUnchecked());
    }
}