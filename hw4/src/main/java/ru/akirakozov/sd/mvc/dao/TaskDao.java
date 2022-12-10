package ru.akirakozov.sd.mvc.dao;

import ru.akirakozov.sd.mvc.model.Task;
import ru.akirakozov.sd.mvc.model.TaskList;

import java.util.List;
import java.util.Optional;

/**
 * @author akirakozov
 */
public interface TaskDao {
    void addTask(String taskList, String task);

    void removeTask(String taskList, String task);

    List<TaskList> getTaskLists();

    void createTaskList(String taskList);

    void completeTask(String taskList, String task);

    void uncompleteTask(String taskList, String task);
}
