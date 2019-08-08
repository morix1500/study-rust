use std::io;
use std::io::Cursor;
use std::collections::HashMap;
use wordcount::{count, CountOption};

#[macro_use]
mod utils;

#[test]
fn char_count_works() {
    let input = Cursor::new(b"abadracadabra");
    let freq  = count(input, CountOption::Char);
    assert_map!(freq,
        {
            "a" => 6,
            "b" => 2,
            "c" => 1,
            "d" => 2,
            "r" => 2
        }
    );
}

#[test]
fn word_count_works() {

    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("bb".to_string(), 2);
    exp.insert("cc".to_string(), 1);

    assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
}

#[test]
fn word_count_works2() {
    use std::io::Cursor;

    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("cc".to_string(), 1);
    exp.insert("dd".to_string(), 1);

    assert_eq!(count(Cursor::new("aa  cc dd"), CountOption::Word), exp);
}

#[test]
fn result_test() -> io::Result<()> {
    use std::fs::{read_to_string, remove_file, write};
    write("test.txt", "message")?;
    let message = read_to_string("test.txt")?;
    remove_file("test.txt")?;
    assert_eq!(message, "message");
    Ok(())

}

#[test]
#[should_panic]
fn word_count_do_not_contain_unknown_words() {
    use std::io::Cursor;

    count(
        Cursor::new([
            b'a',
            0xf0, 0x90, 0x80,
            0xe3, 0x81, 0x82,
        ]), CountOption::Word);
}