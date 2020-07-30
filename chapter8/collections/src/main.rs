fn main() {
    // let mut v: Vec<i32> = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    let mut v = vec![1, 2, 3, 4, 5];
    // let third = &v[100];
    // let third = v.get(100);
    let first = &v[0];
    v.push(6);  // Error出ないぞ…？

    let v2 = vec![100, 32, 57];
    for i in &v2 {
        println!("{}", i);
    }

    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50;
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

}
