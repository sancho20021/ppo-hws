package ru.akirakozov.sd.mvc.controller;

import org.springframework.stereotype.Controller;
import org.springframework.ui.ModelMap;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.RequestParam;
import ru.akirakozov.sd.mvc.dao.TaskDao;
import ru.akirakozov.sd.mvc.model.Task;
import ru.akirakozov.sd.mvc.model.TaskList;

import java.util.List;
import java.util.Optional;

/**
 * @author akirakozov
 */
@Controller
public class TaskController {
    private final TaskDao taskDao;

    public TaskController(TaskDao taskDao) {
        this.taskDao = taskDao;
    }

    @RequestMapping(value = "/add-task", method = RequestMethod.POST)
    public String addTask(@RequestParam String taskList, @RequestParam String task) {
        taskDao.addTask(taskList, task);
        return "redirect:/get-task-lists";
    }

    @RequestMapping(value = "/remove-task", method = RequestMethod.DELETE)
    public String removeTask(@RequestParam String taskList, @RequestParam String task) {
        taskDao.removeTask(taskList, task);
        return "redirect:/get-task-lists";
    }

    @RequestMapping(value = "/get-task-lists", method = RequestMethod.GET)
    public String getTaskLists(ModelMap map) {
        prepareModelMap(map);
        return "index";
    }

    @RequestMapping(value = "/create-task-list", method = RequestMethod.POST)
    public String createTaskList(@RequestParam String name) {
        taskDao.createTaskList(name);
        return "redirect:/get-task-lists";
    }

    @RequestMapping(value = "/complete-task", method = RequestMethod.POST)
    public String completeTask(@RequestParam String taskList, @RequestParam String task) {
        taskDao.completeTask(taskList, task);
        return "redirect:/get-task-lists";
    }


    @RequestMapping(value = "/uncomplete-task", method = RequestMethod.POST)
    public String uncompleteTask(@RequestParam String taskList, @RequestParam String task) {
        taskDao.uncompleteTask(taskList, task);
        return "redirect:/get-task-lists";
    }

    private void prepareModelMap(ModelMap map) {
        map.addAttribute("taskLists", taskDao.getTaskLists());
        map.addAttribute("taskList", new TaskList());
        map.addAttribute("task", new Task());
    }
}
