use std::collections::HashMap;
use std::ops::Drop;
use std::rc::Rc;

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

struct A {
    c: char,
    s: String,
}

use std::cell::RefCell;

struct B {
    c: char,
    s: RefCell<String>,
}

fn main() {
    let mut rc1 = Rc::new(Child(1));
    println!("(a) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);
    {
        let rc2 = Rc::clone(&rc1);
        println!(
            "(b) count: {}, rc1: {:?}, rc2: {:?}",
            Rc::strong_count(&rc1),
            rc1,
            rc2,
        );
    }
    println!("(c) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    if let Some(child) = Rc::get_mut(&mut rc1) {
        child.0 += 1;
    }
    println!("(d) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    let weak = Rc::downgrade(&rc1);
    println!(
        "(e) count: {}, rc1: {:?}, weak: {:?}",
        Rc::strong_count(&rc1),
        rc1,
        weak,
    );

    if let Some(rc3) = weak.upgrade() {
        println!(
            "(f) count: {}, rc1: {:?}, rc3: {:?}",
            Rc::strong_count(&rc1),
            rc1,
            rc3,
        );
    }

    std::mem::drop(rc1);
    println!("(g) count: 0, weak.upgrade(): {:?}", weak.upgrade());

    //let a = A { c: 'a', s: "alex".to_string()};
    //let r = &a;
    //r.s.push('A');
    let b = B {
        c: 'a',
        s: RefCell::new("alex".to_string()),
    };
    let rb = &b;
    rb.s.borrow_mut().push('a');
    {
        let rbs = b.s.borrow();
        assert_eq!(&*rbs, "alexa");

        //b.s.borrow_mut();
        assert!(b.s.try_borrow_mut().is_err());
    }
    assert!(b.s.try_borrow_mut().is_ok());
}
