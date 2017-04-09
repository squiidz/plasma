extern crate plasma;

use plasma::interpreter;

fn main() {
    let code = "
        var square = function(num) { num * 2 };
        square(13);
    ";

    if let Ok(result) = interpreter::execute(code) {
        println!("{}", result);
    }
}
