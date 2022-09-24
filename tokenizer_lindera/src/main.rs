use lindera::tokenizer::Tokenizer;
use lindera::LinderaResult;

fn main() -> LinderaResult<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let word = &args[1];
    // create tokenizer
    let tokenizer = Tokenizer::new()?;

    // tokenize text
    let tokens = tokenizer.tokenize(word)?;

    // output tokens
    for token in tokens {
        println!(
            "{:?},{:?}",
            token.text,
            tokenizer.word_detail(token.word_id)
        );
    }

    Ok(())
}
