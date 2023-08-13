fn main() {
    println!("Hello, world!");
    sum(2,1);
    number_to_string(3);
}


fn sum(number1:i32, number2:i32) -> i32 {
    return number1 + number2;
}

//преобразование числа в строку
fn number_to_string(i:i32)->String{
    println!("{}", i);
    return i.to_string();
}
//поиск самой короткой строки
fn find_short(s:&str)->u32{
    //сплитуем каждое слово строки в коллекцию
    let words:Vec<&str>=s.split_whitespace().collect();
    //присваем переменной shorts_words самое длинной слово в коллекции
    let mut shorts_words=u32::MAX;
    //итерируем значения коллекции => ищем самое короткое слово в коллекции
    for word in words {

        let length=word.len()as u32;

        if length < shorts_words{
            shorts_words=length
        }
    }
    println!("{}", shorts_words);
    shorts_words
}