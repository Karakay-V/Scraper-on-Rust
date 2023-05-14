use std::io;

pub fn type_username () -> String {
    println!("Очікую твій логін... ");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Помилка при читанні");
    return username;
}

pub fn type_password () -> String {
    println!("Очікую твій пароль... ");
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Помилка при читанні");
    return password;
}