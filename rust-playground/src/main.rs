fn build_greeting(user_name: &str) -> String{
    let message = format!("Привет");
    message
}

fn create_user_label(name: &str, role: &str) -> String{
    format!("{name} - {role}")
}

fn validate_task_title(title: &str) -> Result<String, String>{
    if title.trim().is_empty(){
        return Err("Название задачи не может быть пустым".to_string());
    }

    Ok(title.trim().to_string())
}

fn calculate_task_score(priority: u32, complexity: u32) -> u32 {
    priority * complexity
}

fn main() {
    let app_name = "TeamFlow Desktop";
    let lesson_number = 1;
    let is_rust_installed = true;

    println!("Приложение: {app_name}");
    println!("Номер урока: {lesson_number}");
    println!("Rust установлен?: {is_rust_installed}");

    // в расте по умолчанию переменные не изменяемые
    let counter = 0;

    let name = "Oleg";
    let greeting = build_greeting(name);
    println!("{greeting}");

    let user_name = "Kirill";
    let user_role = "Kill";
    let label = create_user_label(user_name, user_role);

    println!("Пользователь: {label}");

    let title = "   сделать первый экран KanBan   ";
    let validate_result = validate_task_title(title);

    match validate_result{
        Ok(clean_title) =>{
            println!("Задача создана: {clean_title}");
        }
        Err(error_message) => {
            println!("Ошибка: {error_message}");
        }
    }

    let score = calculate_task_score(3, 5);
    println!("Score: {score}");
}
