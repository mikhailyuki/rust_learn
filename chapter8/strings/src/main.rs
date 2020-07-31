fn main() {
    // let mut s = String::new();
    // let data = "initial contents";
    // s = data.to_string();
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);   // 動作する
    let mut s3 = String::from("lo");
    s3.push('l');
    println!("s3 is {}", s3);

    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let hello_world = hello + &world;
    println!("{}", hello_world);

    let s4 = String::from("ran");
    let s5 = String::from("ran");
    let s6 = String::from("roo");
    let donald = format!("{} {} {}!", s4, s5, s6);
    println!("{}", donald);

    let japanese = String::from("文字列");
    // let sliced = &japanese[0..2];
    let sliced = &japanese[0..3];
    println!("{}", sliced);
    for c in japanese.chars() {
        println!("{}", c);
    }
}
