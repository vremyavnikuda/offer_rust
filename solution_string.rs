pub fn solution (s:&str)-> String{

    let mut result = String::new();

    // флаг значения символа
    // какой регистр символа
    let mut prev_char_was_lowercase=false;

    /*
    проверяем все символы в строке к какому регистру они относятся
    */
    for c in s.chars(){
        //проверяем символ в строке
        //если символ в строке относится к верхнему регистру ,а предыдуший символ относится к нижнему регистру
        if c.is_ascii_uppercase(){
        if prev_char_was_lowercase{
            //между ними добавляется пробел
            result.push(' ');
        }
        //а условному флагу назначется значение false
        prev_char_was_lowercase=false
        }else {
            //иначе true
            prev_char_was_lowercase=true
        }
        result.push(c);
    }
    result
}