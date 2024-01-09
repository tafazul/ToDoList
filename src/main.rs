// Task struct Definition
#[derive(Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool
}

fn main() {
    // A vector to store instances of the Task struct
    let mut to_do_list: Vec<Task> = Vec::new();

    // add_tasks
    let added_task1 = add_task("Rust Homework 1", &mut to_do_list);
    let added_task2 = add_task("Rise in Internet Computer Bootcamp", &mut to_do_list);

    // list_tasks before completion
    println!("*****Before Completion*****");
    list_tasks(&to_do_list);

    // complete_task
    complete_task(added_task1.id, &mut to_do_list);
    complete_task(added_task2.id, &mut to_do_list);

    // list_tasks after completion
    println!("*****After Completion*****");
    list_tasks(&to_do_list);
}

fn add_task(description: &str, to_do_list: &mut Vec<Task>) -> Task {
    let new_task = Task {
        id: to_do_list.iter().count()+1,
        description: String::from(description),
        completed: false
    };
    to_do_list.push(new_task.clone());
    new_task
}

fn complete_task(id: usize, to_do_list: &mut Vec<Task>) -> Option<&Task> {
    if let Some(task) = to_do_list.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        Some(task)
    } else {
        None
    }
}

fn list_tasks(to_do_list: &Vec<Task>) {
    for task in to_do_list {
        println!("{:?}", task);
    }
}