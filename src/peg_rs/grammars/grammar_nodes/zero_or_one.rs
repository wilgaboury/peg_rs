use peg_rs::grammars::buildable::*;
use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::grammar_nodes::production::ProductionNode;
use peg_rs::grammars::matches::match_collector::*;

pub struct ZeroOrOneNode {
    pub child: Box<GrammarNode>,
}

pub struct ZeroOrOne {
    child: Box<Buildable>,
}

impl ZeroOrOne {
    pub fn new(child: Box<Buildable>) -> Box<ZeroOrOne> {
        Box::new(ZeroOrOne{child})
    }
}

impl GrammarNode for ZeroOrOneNode {
    fn run(&self, input: &mut Parsable) -> ParseResult {
        match self.child.run(input) {
            ParseResult::Success(parse_data) => ParseResult::Success(parse_data),
            ParseResult::Failure => ParseResult::new_empty()
        }
    }
}

impl Buildable for ZeroOrOne {
    fn build(&self, map: &mut HashMap<String, Rc<RefCell<ProductionNode>>>, prods: &HashMap<String, Production>) -> Result<Box<GrammarNode>, String> {
        match self.child.build(map, prods) {
            Result::Ok(grammar_node) => Result::Ok(
                Box::new(ZeroOrOneNode {
                    child: grammar_node
                })
            ),
            Result::Err(err) => Result::Err(err),
        }
    }
}

#[test]
fn test_zero_or_one() {
    use peg_rs::grammars::grammar_nodes::*;
    use peg_rs::grammars::grammar_builder::GrammarBuilder;

    let grammar = GrammarBuilder::new(
        Production::new(
            "Prod1",
            ZeroOrOne::new(
                StrLit::new("test")
            )
        )).build().unwrap();

    assert!(grammar.parse("test"));
    assert!(grammar.parse(""));
}