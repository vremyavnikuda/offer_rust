pub(crate) fn club() {
    let age: i8 = 20;
    if age >= 18 {
        println!("Можете проходить в клуб")
    } else {
        println!("Вам меньше нужного возраста")
    }
}

pub(crate) fn name_user() {
    let name: String = String::from("Andrew");
    if name != "Andrew" {
        println!("Имя пользователя неправильно указана")
    } else {
        println!("Добро пожаловать {} ", name)
    }
}

pub(crate) fn num_bool() -> [(); 1] {
    let is_true: bool = true;

    let num = if is_true {
        1
    } else {
        0
    };
    [println!("{}", num)]
}