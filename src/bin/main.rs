extern crate plasma;

use plasma::interpreter;

fn main() {
    let code = "
        var ten = 5 + 5;
        var res = ten * 5 / 2;
        res == ten;
    ";

    if let Ok(result) = interpreter::execute(code) {
        println!("{}", result);
    }
}
