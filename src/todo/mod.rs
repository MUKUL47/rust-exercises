
pub struct Todo{
    tasks: Vec<Task>
}
#[derive(Debug)]
enum TaskStatus{
    Pending, Completed, Inprogress
}
#[derive(Debug)]
struct Task{
    work: String,
    id: usize,
    status: TaskStatus
}
 impl Todo{
    pub fn new () -> Self{
        Todo{
            tasks: vec!()
        }
    }

    fn ask_std_inp(question: &str) -> String{
        println!("{}: ", question);
        let mut q = String::from("");
        std::io::stdin().read_line(&mut q).unwrap_or(0);
        q.trim_end().to_string()
    }

    fn create_new_task(&mut self) {
        let task = Todo::ask_std_inp("Enter task");
        self.tasks.push(Task{
            id: self.tasks.len() + 1,
            status: TaskStatus::Pending,
            work: task
        })
    }

    fn update_task_status(&mut self) {
        let task_id = self.ask_for_task_id().unwrap_or(0);
        if task_id > 0  {
            match self.get_task_by_id(task_id){
                Some((t, i)) => {
                    let task = Todo::ask_std_inp("1. Pending\n2. Completed\n 3. Inprogress");
                    t.status = Todo::get_status_map(task);
                },
                None => {}
            }
        }
    }

    fn update_task(&mut self) {
        let task_id = self.ask_for_task_id().unwrap_or(0);
        if task_id > 0  {
            match self.get_task_by_id(task_id){
                Some((t, _)) => {
                    t.work = Todo::ask_std_inp("Enter task to edit");
                },
                None => {}
            }
        }
    }

    fn get_status_map(task: String) ->TaskStatus{
        match task.as_str() {
            "1" => TaskStatus::Pending,
            "2" => TaskStatus::Completed,
            _ => TaskStatus::Inprogress,
        }
    }

    fn get_task_by_id(&mut self, id: usize) -> Option<(&mut Task, usize)>{
        for (i,k) in self.tasks.iter_mut().enumerate(){
            if k.id == id {
                return Some((k, i))
            }
        }
        println!("Task with {0} ID not found", id);
        None
    }

    fn ask_for_task_id(&self) -> Option<usize>{
        let task_id = Todo::ask_std_inp("Enter task ID");
        match task_id.parse::<usize>(){
            Ok(n) => Some(n),
            Err(_) => {
                println!("Invalid ID {0}", task_id);
                return None;
            }
        }
    }

    fn delete_task(&mut self) {
        let task_id = self.ask_for_task_id().unwrap_or(0);
        if task_id > 0  {
            match self.get_task_by_id(task_id){
                Some((_, i)) => {
                    self.tasks.swap_remove(i);
                },
                None => {}
            }
        }
    }

    fn print_options(&self){
        println!("###################");
        println!("1. Show TODOS");
        println!("2. Create");
        println!("3. Delete");
        println!("4. Edit");
        println!("5. Update status");
        println!("6. Exit");
        println!("###################");
    }

    fn perform_operations(&mut self, option: i32)-> bool{
        match option{
            1 => {self.print_todos(); return true},
            2 => {self.create_new_task(); return true},
            3 => {self.delete_task(); return true},
            4 => {self.update_task(); return true},
            5 => {self.update_task_status(); return true},
            _ => { return false}
        }
    }

    fn print_todos(&self){
        if self.tasks.len() == 0 {
            println!("You have no TODOS!");
            return;
        }
        for task in self.tasks.iter(){
            let status_str = format!("{:?}", task.status);
            println!("{0}. {1} | {2}", task.id, task.work, status_str);
        }
    }

    pub fn start(&mut self){
        loop{
            self.print_options();
            match Todo::ask_std_inp("").parse::<i32>(){
                Ok(o) => {
                    let response = self.perform_operations(o);
                    if !response {
                        break;
                    }
                },
                Err(_) => {
                    break;
                }
            }
        }
    }
}