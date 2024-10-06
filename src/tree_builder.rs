use crate::tokens::Token;

// pub enum MathType {
//     Number(i128),
//     Vector(Vec<i128>),
//     Matrix(Vec<Vec<i128>>),
// }

// const TOKEN_MAP: [(&str, Operator); 21] = [
//     ("=", Operator::Assignment),
//     ("+", Operator::Plus),
//     ("-", Operator::Minus),
//     ("*", Operator::Mult),
//     ("/", Operator::Div),
//     ("^", Operator::Pow),
//     ("&", Operator::Addr),
//     (">", Operator::Gt),
//     (">=", Operator::Ge),
//     ("<", Operator::Lt),
//     ("<=", Operator::Le),
//     (".", Operator::Dot),
//     (",", Operator::Comma),
//     ("(", Operator::ParenOpen),
//     (")", Operator::ParenClose),
//     ("[", Operator::BracketOpen),
//     ("]", Operator::BracketClose),
//     ("{", Operator::CurlyOpen),
//     ("}", Operator::CurlyClose),
//     ("->", Operator::ArrowRight),
//     ("$&", Operator::Wacky),
// ];

#[allow(dead_code)]
#[derive(Debug)]
pub enum ExpressionBuildError {
    NoClosingBrace
}

#[derive(Debug)]
pub enum Node {
    Tkn(Token),
    Exp(Vec<Node>),
}

// loop through subnodes
// if closing closing brace, pull inner nodes into expression and return
// if opening brace, recurse into next layer
fn parse_tree_braces(mut nodes: Vec<Node>) -> Result<Vec<Node>, ExpressionBuildError> {
    let mut i = 0;
    while i < nodes.len() {
        if let Node::Tkn(token) = &nodes[i] {
            match token.as_str() {
                ")" => {
                    let mut remaining_nodes = nodes.split_off(i + 1);
                    nodes.pop(); // remove ")"

                    let expression = Node::Exp(nodes);

                    let mut new_nodes = Vec::with_capacity(remaining_nodes.len() + 1);

                    new_nodes.push(expression);
                    new_nodes.append(&mut remaining_nodes);
                    return Ok(new_nodes);
                },
                "(" => {
                    let contained_nodes = nodes.split_off(i + 1);
                    nodes.pop(); // remove "("

                    let mut parsed_nodes = parse_tree_braces(contained_nodes)?;
                    nodes.append(&mut parsed_nodes);
                    return Ok(nodes);
                },
                _ => {},
            }
        }
        i += 1;
    }
    Ok(nodes)
}


// fn build_brace_tree(node: &mut Node) -> Result<(), ExpressionBuildError> {
//     if let Node::Exp(subnodes) = node {
//         let open_brace = subnodes.iter().position(|node| match node {
//             Node::Tkn(token) => token == "(",
//             Node::Exp(_) => false,
//         });
//         if let Some(open_index) = open_brace {
//             let close_brace = subnodes.iter().rev().position(|node| match node {
//                 Node::Tkn(token) => token == ")",
//                 Node::Exp(_) => false,
//             });
//             match close_brace {
//                 None => return Err(ExpressionBuildError::NoClosingBrace),
//                 Some(close_index) => {
//                     let close_index = subnodes.len() - 1 - close_index; // adjust for rev()
//                     println!("Pulling parentheses at {} and {}", open_index, close_index);
//                     if open_index > close_index {
//                         return Err(ExpressionBuildError::NoClosingBrace);
//                     }
//                     let mut enclosed_nodes: Vec<_> = subnodes.drain(open_index..=close_index).skip(1).collect();
//                     enclosed_nodes.pop(); // skip and pop to remove parentheses
//                     let mut new_expression = Node::Exp(enclosed_nodes);
//                     build_brace_tree(&mut new_expression)?;
//                     subnodes.insert(open_index, new_expression);
//                 },
//             };
//         }
//     }
//     Ok(())
// }


pub fn build_expression_tree(token_sequence: Vec<Token>) -> Result<Node, ExpressionBuildError> {
    let nodes: Vec<Node> = token_sequence.iter().map(|token| Node::Tkn(token.clone())).collect();
    let nodes = parse_tree_braces(nodes)?;
    // TODO: mutate root expression going through full order of operations
    Ok(Node::Exp(nodes))
}
