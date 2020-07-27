use pest_consume::{match_nodes, Error, Parser};
use stream::ast::*;
use stream::read_to_string;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[derive(Parser)]
#[grammar = "stream.pest"]
struct StreamParser;

#[pest_consume::parser]
impl StreamParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn expr(input: Node) -> Result<Expr> {
        Ok(Expr::Const(input.as_str().parse::<i64>().unwrap()))
    }

    fn file(input: Node) -> Result<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [expr(e).., _] => Prog {
                body: e.collect(),
            }
        ))
    }
}

fn parse_file(input_str: &str) -> Result<Prog> {
    let inputs = StreamParser::parse(Rule::file, input_str)?;
    let input = inputs.single()?;
    StreamParser::file(input)
}

fn main() -> Result<()> {
    let prog = read_to_string("examples/prog_0.stream");
    println!("{:?}", parse_file(&prog)?);
    Ok(())
}