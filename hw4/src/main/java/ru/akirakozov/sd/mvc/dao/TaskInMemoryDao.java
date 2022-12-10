package ru.akirakozov.sd.mvc.dao;

import ru.akirakozov.sd.mvc.model.Task;
import ru.akirakozov.sd.mvc.model.TaskList;

import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Optional;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.atomic.AtomicInteger;

public class TaskInMemoryDao implements TaskDao {
    private final LinkedHashMap<String, TaskList> taskLists = new LinkedHashMap<>();

    @Override
    public void addTask(String taskList, String task) {
        TaskList list = taskLists.get(taskList);
        if (list != null) list.addTask(task);
    }

    @Override
    public void removeTask(String taskList, String task) {
        TaskList list = taskLists.get(taskList);
        if(list != null) list.removeTask(task);
    }

    @Override
    public List<TaskList> getTaskLists() {
        return new ArrayList<>(taskLists.values());
    }

    @Override
    public void createTaskList(String taskList) {
        taskLists.put(taskList, new TaskList(taskList));
    }

    @Override
    public void completeTask(String taskList, String task) {
        TaskList list = taskLists.get(taskList);
        if (list == null) return;
        list.setTaskCompleted(task, true);
    }

    @Override
    public void uncompleteTask(String taskList, String task) {
        TaskList list = taskLists.get(taskList);
        if (list == null) return;
        list.setTaskCompleted(task, false);
    }
}
