#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenizeError {
    IncompleteOperator(String),
}

pub type Token = String;

fn push_token(sequence: &mut Vec<Token>, token: &mut Token) {
    if token != "" {
        sequence.push(token.clone());
        token.clear();
    }
}

pub fn generate_tokens(text: &String) -> Vec<Token> {
    let mut token_sequence: Vec<Token> = Vec::new();
    let mut current_token = String::new();

    let mut last_c = '_';

    let is_word = |c: char| c.is_alphanumeric() || c == '.';
    let is_operator = |c: char| !c.is_whitespace() && c != '.' && !c.is_alphanumeric();

    for c in text.chars() {
        if !(is_word(last_c) && is_word(c)) && !(is_operator(last_c) && is_operator(c)) { // mode swapped
            push_token(&mut token_sequence, &mut current_token);
        }
        if !c.is_whitespace() {
            current_token.push(c);
        }
        last_c = c;
    }

    push_token(&mut token_sequence, &mut current_token);
    token_sequence
}
