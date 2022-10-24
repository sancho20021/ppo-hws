import org.apache.commons.io.IOUtils;

import java.io.FileInputStream;
import java.io.IOException;
import java.io.UncheckedIOException;
import java.nio.charset.StandardCharsets;

public class Utils {
    public static String getToken() throws IOException {
        try (FileInputStream inputStream = new FileInputStream("./../../vkservicetoken.txt")) {
            return IOUtils.toString(inputStream, StandardCharsets.UTF_8).strip();
        }
    }

    public static String getTokenUnchecked() {
        try {
            return getToken();
        } catch (IOException e) {
            throw new UncheckedIOException("error while reading token: ", e);
        }
    }
}
