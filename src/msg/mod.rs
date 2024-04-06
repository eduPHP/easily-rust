use crate::print_help;

pub fn info(msg: &str) {
    println!("{}{}", "\x1B[0;44m INFO \x1B[m ", msg);
}
pub fn error(msg: &str) {
    println!("{}{}", "\x1B[0;41m ERROR \x1B[m ", msg);
}
pub fn success(msg: &str) {
    println!("{}{}", "\x1B[0;30;42m SUCCESS \x1B[m ", msg);
}
pub fn warning(msg: &str) {
    println!("{}{}", "\x1B[0;30;43m WARN \x1B[m ", msg);
}

pub fn panic(msg: &str) {
   error(msg);
   print_help();
   std::process::exit(1); 
}