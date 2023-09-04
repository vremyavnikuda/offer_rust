use std::f32::consts::E;
use std::io::BufRead;
use std::ptr::null;

pub(crate) fn penguin_data(random_text:&str){

    let records=random_text.lines();
    for (i,record) in records.enumerate() {

        if i==0||record.trim().len()==0{
            continue;
        }

        let fields:Vec<_> = record.split(",").map(|field|field.trim()).collect();
        if cfg! (debug_assertions){
            eprint!("debug: {:?} -> {:?}",record,fields)
        }

        if let Some(name) = fields.get(0) {
            if let Some(length_str) = fields.get(0) {
                if let Ok(length) = length_str.parse::<f32>() {
                    println!("{},{}cm", name, length)
                }
            }
        }
    }
}

fn count_sheep(sheep:&[bool])-> u8 {
    sheep.iter().filter(|&&present| present).count() as u8
}
fn count_by(x:u32,n:u32)->Vec<u32>{
    (1..n+1).map(|i|x*i).collect()
}

#[cfg(test)]
mod test{
    use crate::penguin_data::{count_by, count_sheep};

    #[test]
    fn returns_correct_sheep_count() {
        assert_eq!(count_sheep(&[false]), 0);
        assert_eq!(count_sheep(&[true]), 1);
        assert_eq!(count_sheep(&[true, false]), 1);
    }
    #[test]
    fn sample_tests() {
        assertion(vec![1,2,3,4,5,6,7,8,9,10], (1, 10));
        assertion(vec![2,4,6,8,10], (2, 5));
        assertion(vec![3,6,9,12,15,18,21], (3, 7));
        assertion(vec![50,100,150,200,250], (50, 5));
        assertion(vec![100,200,300,400,500,600], (100, 6));
    }

    fn assertion(expected : Vec<u32>, inputs : (u32, u32)) {
        let actual = count_by(inputs.0, inputs.1);

        assert_eq!(expected, actual, "\nTest failed!\n expected: [{}]\n actual: [{}]\n x: {}\n n: {}\n", expected.iter().join(", "), actual.iter().join(", "), inputs.0, inputs.1);
    }
}