use stream::parser::{parse_file, Result};
use stream::read_to_string;

fn main() -> Result<()> {
    let prog = read_to_string("examples/prog_0.stream");
    println!("{:?}", parse_file(&prog)?);
    Ok(())
}
