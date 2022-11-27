package ru.akirakozov.sd.mvc.model;

import java.util.ArrayList;
import java.util.List;

/**
 * @author akirakozov
 */
public class TaskList {
    private String name;
    private final List<Task> tasks = new ArrayList<>();

    public TaskList() {
    }

    public TaskList(String name) {
        this.name = name;
    }

    public String getName() {
        return name;
    }

    public List<Task> getTasks() {
        return tasks;
    }

    public void addTask(String task) {
        tasks.add(new Task(task));
    }

    public void removeTask(String task) {
        tasks.removeIf(task1 -> task1.getName().equals(task));
    }

    public void setTaskCompleted(String name, boolean completed) {
        tasks.stream().filter(task -> task.getName().equals(name)).forEach(task -> task.setCompleted(completed));
    }
}
