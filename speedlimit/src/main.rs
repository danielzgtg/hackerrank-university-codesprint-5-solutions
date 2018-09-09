use std::io;

fn main() {
    let num: u32;
    {
        let mut numstr = String::new();
        io::stdin().read_line(&mut numstr).unwrap();
        num = numstr.trim().parse().unwrap();
    }
    let (fine, notice) = match num {
        0 ... 90 => (0, "No punishment"),
        91 ... 110 => ((num - 90) * 300, "Warning"),
        _ => ((num - 90) * 500, "License removed"),
    };
    println!("{} {}", fine, notice);
}
