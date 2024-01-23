Task Details

In this homework, you will create a simple to-do list in Rust that can perform basic listing.

Steps:

Create a struct called Task with the following fields:

id (an integer representing the task ID)

description (a string representing the task description)

completed (a boolean indicating whether the task is completed or not)

Create a vector to store instances of the Task struct. This vector will represent your ToDo list.

Implement the following functions:

add_task(description: &str) -> Task: This function should take a task description as an argument, create a new Task instance with a unique ID, mark it as not completed, add it to the vector, and return the created Task.

complete_task(id: usize) -> Option<&Task>: This function should take a task ID as an argument, find the corresponding task in the vector, mark it as completed, and return a reference to the completed Task. If the task with the given ID does not exist, return None.

list_tasks(): This function should print the details of all tasks in the ToDo list, including their ID, description, and completion status.

Checklist:

Define the Task struct with the specified fields.

Create a vector to store instances of the Task struct.

Implement the add_task function.

Implement the complete_task function.

Implement the list_tasks function.
