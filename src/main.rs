//extern crate lazy_static;
extern crate regex;

use std::fs;



use regex::Regex;

fn main() {
    exercise1();
    exercise2("Mathematica");
}


fn exercise1 () {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];
    
    let mut idx = 0;
    let mut cnt = 0;

    for (org_index, i) in org_arr.iter().enumerate() {
        for (sub_index, j) in sub_arr.iter().enumerate() {
            if cnt == sub_index {
                if i == j {
                    idx = org_index;
                    cnt+=1;
                    break;
                } else if i == &sub_arr[0] {
                    cnt = 1;
                    break;
                }
                cnt = 0;
            }   
        }
        if cnt == sub_arr.len() {
            break;
        }
    }
    if cnt == sub_arr.len() {
        println!("Finded! Start from index {}", idx - cnt + 1);
    } else {
        println!("Can't find");
    }
}


fn exercise2 (text: &str) {
    let contents = fs::read_to_string("test.txt")
        .expect("Can not read file!");


    let re = Regex::new(text).unwrap();
    let result = re.find_iter(&contents).count();
    println!("Count: \n{}", result);
}