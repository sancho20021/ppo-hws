import ru.akirakozov.sd.refactoring.DBManager;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;

public class Common {
    public static void resetDB() throws IOException {
        Path src = Paths.get("start_db.db");
        Path dst = Paths.get(DBManager.TEST_DB_FILE);
        Files.copy(src, dst, StandardCopyOption.REPLACE_EXISTING);
    }
}
