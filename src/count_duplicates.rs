use std::mem::forget;

fn count_duplicates(text: &str) -> u32 {
    let mut char_counts=std::collections::HashMap::new();

    for c in text.chars(){
        let c_lowercase =c.to_lowercase().next().unwrap();
        *char_counts.entry(c_lowercase).or_insert(0) += 1;
    }


    let mut count_duplicates=0;
    for &count in char_counts.values() {
        if count > 1 {
            count_duplicates += 1;

        }
    }
    count_duplicates
}