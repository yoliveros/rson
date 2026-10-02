use rson::run;
use std::process::exit;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let ret = run(&args);
    exit(ret);
}
