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

    let is_word = |c: char| c.is_alphanumeric();
    let is_operator = |c: char| !c.is_whitespace() && !c.is_alphanumeric();

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


// pub fn generate_tokens(text: &String) -> Result<Vec<Token>, TokenizeError> {
//     let mut token_sequence: Vec<Token> = Vec::new();
//     let mut current_text = String::new();

//     let mut is_text = true;

//     let chars = text.chars().skip_while(|c| c.is_whitespace());

//     for c in chars {
//         if is_text {
//             let possible_token = TOKEN_MAP.iter().find(|entry| entry.0.starts_with(c));
//             if possible_token.is_some() {
//                 if current_text != "" {
//                     token_sequence.push(Token::Text(current_text)); // break text with operator
//                     current_text = String::new();
//                 }
//                 is_text = false;
//             }
//             if c.is_whitespace() {
//                 if current_text != "" {
//                     token_sequence.push(Token::Text(current_text)); // break text with whitespace
//                     current_text = String::new();
//                 }
//             } else {
//                 current_text.push(c);
//             }
//         } else {
//             let last_match = TOKEN_MAP.iter().find(|entry| entry.0 == current_text);
//             current_text.push(c);
//             let new_match = TOKEN_MAP.iter().find(|entry| entry.0 == current_text);
//             if new_match.is_none() {
//                 if let Some(token) = last_match {
//                     token_sequence.push(Token::Operator(token.1.clone())); // operator finished
//                     current_text.clear();
//                     if !c.is_whitespace() {
//                         current_text.push(c);
//                     }
//                     is_text = true;
//                 } else {
//                     return Err(TokenizeError::IncompleteOperator(current_text));
//                 }
//             }
//         }
//     }

//     if current_text != "" {
//         token_sequence.push(Token::Text(current_text));
//     }

//     Ok(token_sequence)
// }
