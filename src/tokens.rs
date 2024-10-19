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

fn is_new_token(last_char: char, new_char: char) -> bool {
    let is_word = |c: char| c.is_alphanumeric() || c == '.';
    let is_operator = |c: char| !c.is_whitespace() && c != '.' && !c.is_alphanumeric();

    match last_char {'(' | ')' | '[' | ']' => return true, _=>()};
    match new_char {'(' | ')' | '[' | ']' => return true, _=>()};

    if is_word(last_char) && is_word(new_char) {
        return false;
    }
    if is_operator(last_char) && is_operator(new_char) {
        return false;
    }
    return true;
}

pub fn generate_tokens(text: &String) -> Vec<Token> {
    let mut token_sequence: Vec<Token> = Vec::new();
    let mut current_token = String::new();

    let mut last_c = '_';

    for c in text.chars() {
        if is_new_token(last_c, c) {
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
