package ru.akirakozov.sd.refactoring;

import java.sql.*;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class DBManager {
    public static final String TEST_DB_FILE = "test.db";
    public static final String TEST_DB = "jdbc:sqlite:" + TEST_DB_FILE;

    public static List<SqlRow> executeQuery(String query) throws SQLException {
        return executeQuery(TEST_DB, query);
    }

    public static List<SqlRow> executeQuery(String dbUrl, String query) throws SQLException {
        try (Connection c = DriverManager.getConnection(dbUrl)) {
            Statement stmt = c.createStatement();
            ResultSet rs = stmt.executeQuery(query);

            ResultSetMetaData md = rs.getMetaData();
            int columns = md.getColumnCount();
            List<SqlRow> rows = new ArrayList<>();
            while (rs.next()) {
                Map<String, Object> row = new HashMap<>(columns);
                for (int i = 1; i <= columns; ++i) {
                    row.put(md.getColumnName(i), rs.getObject(i));
                }
                rows.add(new SqlRow(row));
            }
            rs.close();
            stmt.close();
            return rows;
        }
    }

    public static void executeUpdate(String sql) throws SQLException {
        try (Connection c = DriverManager.getConnection(TEST_DB)) {
            Statement stmt = c.createStatement();
            stmt.executeUpdate(sql);
            stmt.close();
        }
    }

    public static class SqlRow {
        private final Map<String, Object> data;

        public SqlRow(Map<String, Object> data) {
            this.data = data;
        }

        public Object get(String column) {
            return data.get(column.toUpperCase());
        }
    }
}
