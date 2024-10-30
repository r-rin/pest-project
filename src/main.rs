use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<( )>{
    let parsed_data = Grammar::parse(Rule::field, "-273.15")?;
    dbg!(parsed_data);
    Ok(())
}
