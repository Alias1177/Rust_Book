use std::io;


#[derive(Debug)]
enum TaskStatus{
    New,
    InProgress,
    Done
}

struct Task{
    id: i64,
    title:String,
    status: TaskStatus
}

impl Task{

    fn new(id:i64,title:String)->Self{
        Task{
            id,
            title,
            status:TaskStatus::New
        }
    }
    fn progress(id:i64,tasks:&mut Vec<Task>){
        for task in tasks{
            if task.id == id{
                task.status = TaskStatus::InProgress;
            }
        }
    
    }
    fn done(id:i64,tasks:&mut Vec<Task>){
        for task in tasks{
            if task.id == id{
                task.status = TaskStatus::Done;
            }
        }
    }
    fn status_task(id:i64,tasks:&mut Vec<Task>){
        for task in tasks{
            if task.id == id{
                println!("Статус задачи: {:#?}",task.status);
            }
        }
    }
}

fn main(){
    let mut tasks = Vec::<Task>::new();
    

    loop{

        println!("\nМеню:");
        println!("1 - Добавить задачу");
        println!("2 - Список задач");
        println!("3 - Задача в работе");
        println!("4 - Задача выполнена");
        println!("5 - Статус задачи");
        println!("6 - Выход");

        
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("failed to read line");


        match buffer.trim(){
            
       "1" => {
                println!("Введите ID задачи:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("failed to read line");
                let id: i64 = id_input.trim().parse().expect("Нужно ввести число");

                println!("Введите название задачи:");
                let mut title_input = String::new();
                io::stdin().read_line(&mut title_input).expect("failed to read line");
                let title = title_input.trim().to_string();

                tasks.push(Task::new(id, title));
                println!("Задача добавлена!");
            }

            "2" => {
                println!("Список задач:");
                for task in &tasks {
                    println!("{} - {}", task.id, task.title);
                }
            }

            "3" => {
                println!("Введите ID задачи:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("failed to read line");
                let id: i64 = id_input.trim().parse().expect("Нужно ввести число");
                Task::progress(id,&mut tasks);
            }

            "4" => {
                println!("Введите ID задачи:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("failed to read line");
                let id: i64 = id_input.trim().parse().expect("Нужно ввести число");
                Task::done(id,&mut tasks);
            }
            "5" =>{
                println!("Введите ID задачи:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("failed to read line");
                let id: i64 = id_input.trim().parse().expect("Нужно ввести число");
                Task::status_task(id,&mut tasks);
            }

            "6" => {
                println!("Выход...");
                break;
            }

            _ => println!("Неизвестная команда"),
        }
    }
}
