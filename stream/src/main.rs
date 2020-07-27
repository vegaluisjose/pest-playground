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

    fn identifier(input: Node) -> Result<Id> {
        Ok(input.as_str().to_string())
    }

    fn number(input: Node) -> Result<i64> {
        Ok(input.as_str().parse::<i64>().unwrap())
    }

    fn expr(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [number(n)] => Expr::Const(n),
            [identifier(n)] => Expr::Ref(n),
        ))
    }

    fn stmt(input: Node) -> Result<Stmt> {
        Ok(match_nodes!(
            input.into_children();
            [identifier(id), expr(e)] => Stmt {
                id,
                expr: e,
            }
        ))
    }

    fn file(input: Node) -> Result<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [stmt(s).., _] => Prog {
                body: s.collect(),
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
