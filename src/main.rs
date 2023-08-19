use crate::club::{club, cycle_num, name_user, num_bool};

mod club;

fn main() {
    println!("Hello, world!");
    sum(2, 1);
    number_to_string(3);
    //grow(vec![1,2,3,4,5])
    club();
    name_user();
    num_bool();
    cycle_num();
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
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(grow(vec![1, 2, 3]), 6);
        assert_eq!(grow(vec![4, 1, 1, 1, 4]), 16);
        assert_eq!(grow(vec![2, 2, 2, 2, 2, 2]), 64);
    }
}


