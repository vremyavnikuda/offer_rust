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

#[cfg(test)]
mod test{
    use crate::penguin_data::count_sheep;

    #[test]
    fn returns_correct_sheep_count() {
        assert_eq!(count_sheep(&[false]), 0);
        assert_eq!(count_sheep(&[true]), 1);
        assert_eq!(count_sheep(&[true, false]), 1);
    }
}