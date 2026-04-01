fn fn3() {
    let blah = 123;
    panic!("{}", blah);
}

fn fn2() {
    fn3();
}

fn main() {
    comfy_color_backtrace::install();
    fn2();
}
