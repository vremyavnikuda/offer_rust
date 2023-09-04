use crate::club::{club, cycle_num, is_exit, name_user, num_bool};
use crate::penguin_data::penguin_data;

mod club;
mod penguin_data;
mod string_to_array;
mod get_volume_of_cuboid;

fn main() {
    println!("Hello, world!");
    sum(2, 1);
    number_to_string(3);
    //grow(vec![1,2,3,4,5])
    club();
    name_user();
    num_bool();
    cycle_num();
    is_exit();
    penguin_data("\
    common name , length (cm) Little penguin , 33 Yellow-eyed penguin , 65 Fiordland penguin , 60 Invalid ,data");
}


fn sum(number1: i32, number2: i32) -> i32 {
    return number1 + number2;
}

//преобразование числа в строку
fn number_to_string(i: i32) -> String {
    println!("{}", i);
    return i.to_string();
}

//поиск самой короткой строки
fn find_short(s: &str) -> u32 {
    //сплитуем каждое слово строки в коллекцию
    let words: Vec<&str> = s.split_whitespace().collect();

    //присвоим переменной shorts_words самое длинной слово в коллекции
    let mut shorts_words = u32::MAX;

    //итерируем значения коллекции => ищем самое короткое слово в коллекции
    for word in words {
        let length = word.len() as u32;

        if length < shorts_words {
            shorts_words = length
        }
    }
    println!("{}", shorts_words);
    shorts_words
}

fn grow(nums: Vec<i32>) -> i32 {
    nums.iter().product()
}

fn boolean_to_string(b: bool) -> String {
    //переводим bool значение -> в значение строки
    b.to_string()
}


#[cfg(test)]
mod tests {
    use crate::club::multiply;
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(grow(vec![1, 2, 3]), 6);
        assert_eq!(grow(vec![4, 1, 1, 1, 4]), 16);
        assert_eq!(grow(vec![2, 2, 2, 2, 2, 2]), 64);
    }

    #[test]
    fn returns_expected() {
        assert_eq!(multiply(3, 5), 15)
    }
}


