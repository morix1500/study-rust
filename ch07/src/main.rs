use std::collections::HashMap;
use std::ops::Drop;

fn process_or_default(key: char, map: &mut HashMap<char, String>) {
    match map.get_mut(&key) {
        Some(value) => value.push_str(", world!"),
        None => {
            map.insert(key, Default::default());
        }
    }
}

#[derive(Debug)]
//#[derive(Copy, Clone, Debug)]
struct Parent(isize, Child, Child);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Debug)]
//#[derive(Copy, Clone, Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn f1(p: &Parent) {
    println!("p: {:?}", p);
}

fn f2(p: &mut Parent) {
    p.0 *= -1;
}

fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));
    f1(&p1);
    f2(&mut p1);
    println!("p1: {:?}", p1);

    let mut map = HashMap::new();
    map.insert('h', "Hello".to_string());
    process_or_default('h', &mut map);
}
