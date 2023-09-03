use std::io;

pub(crate) fn club() {
    let age: i8 = 20;
    if age >= 18 {
        println!("Можете проходить в клуб")
    } else {
        println!("Вам меньше нужного возраста")
    };
}

pub(crate) fn name_user() {
    let name: String = String::from("Andrew");
    if name != "Andrew" {
        println!("Имя пользователя неправильно указана")
    } else {
        println!("Добро пожаловать {} ", name)
    };
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

pub(crate) fn cycle_num(){
    let mut num=0;
    while num<=3 {
        println!("{}",num);
        num+=1;
    };
}

pub(crate) fn is_exit(){
    let mut year = String::new();

    io::stdin()
        .read_line(&mut year)
        .expect("Failed to read input");

    let year:i8=match year.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            eprint!("Invalid input, please enter a valid number");
            return;
        }
    };

    let is_old: bool= year>=18;
    match is_old {
        true => {println!("Проходите")}
        false => {println!("Ваш возраст меньше нужного")}
    };
}

pub(crate) fn multiply(a:i32, b:i32) -> i32 {
    //возвращаем результат умножения
    return a*b
}