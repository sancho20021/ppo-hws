package ru.akirakozov.sd.mvc.model;

/**
 * @author akirakozov
 */
public class Task {
    private String name;
    private boolean completed;


    public Task() {
    }

    public Task(String name) {
        this.name = name;
        this.completed = false;
    }

    public String getName() {
        return name;
    }
    public boolean isCompleted() {
        return completed;
    }

    public void setCompleted(boolean completed) {
        this.completed = completed;
    }
}
