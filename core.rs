pub fn rot() -> String {
    if std::env::args().count() <= 1 {
        panic!("Need arg")
    }
    let args: Vec<String> = std::env::args().collect();
    let t = args[1..].join("").to_uppercase();
    let b = t.as_bytes();

    let mut output = String::new();

    for i in 0..t.len() {
        for j in 0..t.len() - 1 {
            output.push(b[(i + j) % t.len()] as char);
            output.push(' ');
        }
        output.push(b[(i + t.len() - 1) % t.len()] as char);
        output.push('\n');
    }

    return output;
}
