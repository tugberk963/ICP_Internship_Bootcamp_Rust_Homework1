#[derive(Clone)]

struct Task {
    id: usize,
    description: String,
    completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
    task_id: usize, // for the unique id requirement
}

impl TodoList{
    fn new() -> TodoList {
        TodoList {
            tasks: Vec::new(),
            task_id: 1, 
        }
    }

    fn add_task(&mut self, description: &str) -> Task {
        let task = Task {
            id: self.task_id,
            description: String::from(description),
            completed: false,
        };
        self.task_id += 1; // if they add new task it will have unique id which is the +1 of the old one
        self.tasks.push(task.clone());

        task
    }

    fn complete_task(&mut self, id: usize) -> Option<&Task> {
        // Find the task with the given ID and mark it as completed
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            Some(task)
        } else {
            None
        }
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "Task ID: {} - Task Description: {} - Task Completed: {}",
                task.id, task.description, task.completed
            );
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    let task1 = todo_list.add_task("Finish homework 1");
    let _task2 = todo_list.add_task("Finish homework 2");
    let _task3 = todo_list.add_task("Finish homework 3");
    let _task4 = todo_list.add_task("Finish homework 4");
    let _task3 = todo_list.add_task("Finish final project");
    println!("Tasks:");
    todo_list.list_tasks();

    todo_list.complete_task(task1.id);

    println!("\nIs homework 1 done now ?");
    println!("\nTasks:");
    todo_list.list_tasks();
}