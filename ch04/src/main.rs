fn main() {
    let f = "hoge\nfuga\npiyo";
    let lines = f.lines();

    for s in lines {
        println!("{:?}", s);
    }
}
