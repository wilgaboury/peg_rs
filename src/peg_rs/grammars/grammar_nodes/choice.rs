use peg_rs::grammars::buildable::*;
use peg_rs::grammars::grammar_node::*;

pub struct ChoiceNode {
    pub choices: Vec<Rc<GrammarNode>>,
}

pub struct Choice {
    choices: Vec<Box<Buildable>>
}

impl Choice {
    pub fn new(choices: Vec<Box<Buildable>>) -> Choice {
        Choice { choices }
    }
}

impl GrammarNode for ChoiceNode {
    fn run<'a>(&self, input: &mut Parsable<'a>) -> ParseResult<'a> {
        for rc in &self.choices {
            match rc.run(input) {
                ParseResult::SUCCESS(mut parse_data) => return ParseResult::SUCCESS(parse_data),
                _ => ()
            }
        }
        ParseResult::FAILURE
    }
}

impl Buildable for Choice {
    fn build(&self, map: &mut HashMap<String, Rc<GrammarNode>>, prods: &HashMap<String, Production>) -> Result<Rc<GrammarNode>, String> {
        let mut ch = ChoiceNode {
            choices: Vec::new(),
        };
        for buildable in &self.choices {
            match buildable.build(map, prods) {
                Result::Ok(gn) => {
                    ch.choices.push(gn)
                },
                Result::Err(err) => return Result::Err(err),
            }
        }
        Result::Ok(Rc::new(ch))
    }
}

#[test]
fn test_choice() {
    use peg_rs::grammars::grammar_nodes::*;
    use peg_rs::grammars::grammar_builder::GrammarBuilder;

    let grammar = GrammarBuilder::new()
        .add_prod(Production::new("TestStrLit",
              Box::new(StrLit::new("test"))
            )
        )
        .build().unwrap();

    assert!(grammar.parse("test"));
    assert!(!grammar.parse("te"));
    assert!(!grammar.parse("tess"));
    assert!(grammar.parse("testing"));
}