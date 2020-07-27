use stream::parser::parse_file;
use stream::read_to_string;

fn main() {
    let prog = read_to_string("examples/prog.stream");
    println!("Input:\n{}", &prog);
    println!("Output:\n{}", parse_file(&prog).unwrap());
}
