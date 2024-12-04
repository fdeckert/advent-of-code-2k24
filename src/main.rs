use std::{fs::read_to_string, iter::zip};

fn main() {
    let input = read_to_string("./input.txt").unwrap();
    let mut list_1: Vec<i64> = Vec::new();
    let mut list_2: Vec<i64> = Vec::new();

    for line in input.split("\n") {
        println!("{line}");
        if line.is_empty() {
            continue
        }
        let elems: Vec<&str> = line.trim().split("   ").collect();
        println!("{}", elems[0]);
        println!("{}", elems.len());
        assert!(elems.len() == 2, "Fatal : malformed line");
        list_1.push(elems[0].trim().parse().unwrap());
        list_2.push(elems[1].trim().parse().unwrap());
    }

    assert!(list_1.len() == list_2.len(), "Fatal : lists don't have the same length");

    list_1.sort();
    list_2.sort();

    let mut total_distance = 0;

    for (x, y) in zip(list_1, list_2) {
        let mut distance = x - y;
        if distance < 0 {
            distance *= -1;
        }
        total_distance += distance;
    }

    println!("Total distance : {total_distance}");

}
