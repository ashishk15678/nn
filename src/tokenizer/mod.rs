use rand::Rng;
#[derive(Debug, PartialEq)]
pub struct Token {
    pub count: usize,
    pub random_number: usize,
    pub value: String,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    const MIN_OCCURRENCES: usize = 2;
    let mut rng = rand::thread_rng();
    // let mut token_map = std::collections::HashMap::new();
    let mut tokens = Vec::new();

    for word in input.split_whitespace() {
        tokens.push(Token {
            count: 1,
            random_number: rng.gen(),
            value: word.to_string(),
        });
    }

    tokens
}
