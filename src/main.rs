// TODO: Since we are using prefix notation only, we can
// get rid of semicolons(i think)

fn main() {
    let test = "
    fn main() {
        let x: Int = 5;
        let y: Int = 10;
        let z: Int = add(x, y);
        print_int(z);
    }";
    println!("{}", test);

    let tokens = lex(test);
    println!("{:?}", tokens);
    let module = parse(tokens);
    println!("{:?}", module);
}

fn parse(tokens: Vec<Token>) -> Module {
    let mut tokens = tokens.iter().peekable();
    let mut functions = Vec::new();

    while let Some(&token) = tokens.peek() {
        match token {
            Token::Fn => {
                tokens.next();
                let name = match tokens.next() {
                    Some(Token::Name { name }) => name,
                    _ => panic!("Expected function name"),
                };
                let args = parse_argument_definitions(&mut tokens);
                let body = parse_statements(&mut tokens);
                functions.push(Function {
                    name: name.clone(),
                    args,
                    body,
                });
            }
            _ => {
                tokens.next();
            }
        }
    }

    Module {
        name: "main".to_string(),
        functions,
    }
}

fn parse_argument_definitions(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Vec<ArgumentDefinition> {
    let mut args = Vec::new();

    match tokens.next() {
        Some(Token::LeftParen) => {}
        _ => panic!("Expected left paren"),
    }

    loop {
        match tokens.peek() {
            Some(Token::RightParen) => {
                tokens.next();
                break;
            }
            Some(Token::Name { name }) => {
                tokens.next();
                match tokens.next() {
                    Some(Token::Colon) => {}
                    _ => panic!("Expected colon"),
                }
                let type_ = match tokens.next() {
                    Some(Token::Name { name }) => match name.as_str() {
                        "Int" => Type::Int,
                        _ => panic!("Unknown type"),
                    },
                    _ => panic!("Expected type"),
                };
                args.push(ArgumentDefinition {
                    name: name.clone(),
                    type_,
                });
                match tokens.peek() {
                    Some(Token::Comma) => {
                        tokens.next();
                    }
                    Some(Token::RightParen) => {}
                    _ => panic!("Expected comma or right paren"),
                }
            }
            _ => panic!("Expected name or right paren"),
        }
    }

    args
}

// let x: Int = 5; is an example of a statement. It is a variable declaration statement that
// starts with the let keyword and ends with a semicolon
fn parse_statements(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Vec<Statement> {
    let mut statements = Vec::new();

    match tokens.next() {
        Some(Token::LeftBrace) => {}
        _ => panic!("Expected left brace"),
    }

    loop {
        match tokens.peek() {
            Some(Token::RightBrace) => {
                tokens.next();
                break;
            }
            Some(Token::Let) => {
                tokens.next();
                let name = match tokens.next() {
                    Some(Token::Name { name }) => name,
                    _ => panic!("Expected name"),
                };
                match tokens.next() {
                    Some(Token::Colon) => {}
                    _ => panic!("Expected colon"),
                }
                match tokens.next() {
                    Some(Token::Name { name }) => match name.as_str() {
                        "Int" => {}
                        _ => panic!("Unknown type"),
                    },
                    _ => panic!("Expected type"),
                }
                match tokens.next() {
                    Some(Token::Equal) => {}
                    _ => panic!("Expected equal"),
                }
                let value = parse_expression(tokens);
                match tokens.next() {
                    Some(Token::SemiColon) => {}
                    _ => panic!("Expected semicolon"),
                }
                statements.push(Statement::Let {
                    name: name.clone(),
                    type_: Type::Int,
                    value,
                });
            }
            Some(Token::Name { name }) => match name.as_str() {
                "print_int" => {
                    tokens.next();
                    match tokens.next() {
                        Some(Token::LeftParen) => {}
                        _ => panic!("Expected left paren"),
                    }
                    let value = parse_expression(tokens);
                    match tokens.next() {
                        Some(Token::RightParen) => {}
                        _ => panic!("Expected right paren"),
                    }
                    match tokens.next() {
                        Some(Token::SemiColon) => {}
                        _ => panic!("Expected semicolon"),
                    }
                    statements.push(Statement::Print(value));
                }
                "return" => {
                    tokens.next();
                    let value = parse_expression(tokens);
                    match tokens.next() {
                        Some(Token::SemiColon) => {}
                        _ => panic!("Expected semicolon"),
                    }
                    statements.push(Statement::Return(value));
                }
                _ => panic!("Expected let, print_int, or return"),
            },
            _ => panic!("Expected name"),
        }
    }

    statements
}

fn parse_expression(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Expression {
    let expression = match tokens.next() {
        Some(Token::Int { value }) => Expression::Int(value.parse().unwrap()),
        Some(Token::Name { name }) => match tokens.peek() {
            Some(Token::LeftParen) => {
                let args = parse_call_arguments(tokens);
                Expression::Call(name.clone(), args)
            }
            _ => Expression::Name(name.clone()),
        },
        _ => panic!("Expected int or name"),
    };

    expression
}

fn parse_call_arguments(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Vec<Expression> {
    let mut args = Vec::new();

    match tokens.next() {
        Some(Token::LeftParen) => {}
        _ => panic!("Expected left paren"),
    }

    loop {
        match tokens.peek() {
            Some(Token::RightParen) => {
                tokens.next();
                break;
            }
            _ => {
                args.push(parse_expression(tokens));
                match tokens.peek() {
                    Some(Token::Comma) => {
                        tokens.next();
                    }
                    Some(Token::RightParen) => {}
                    _ => panic!("Expected comma or right paren"),
                }
            }
        }
    }

    args
}
#[derive(Debug, PartialEq)]
struct Module {
    name: String,
    functions: Vec<Function>,
}

#[derive(Debug, PartialEq)]
struct Function {
    name: String,
    args: Vec<ArgumentDefinition>,
    body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
struct ArgumentDefinition {
    name: String,
    type_: Type,
}

#[derive(Debug, PartialEq)]
enum Type {
    Int,
}

#[derive(Debug, PartialEq)]
enum Statement {
    Let {
        name: String,
        type_: Type,
        value: Expression,
    },
    Return(Expression),
    Print(Expression),
}

#[derive(Debug, PartialEq)]
enum Expression {
    Int(i32),
    Name(String),
    Add(Box<Expression>, Box<Expression>),
    Call(String, Vec<Expression>),
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            'a'..='z' | 'A'..='Z' => {
                let mut name = String::new();
                name.push(c);
                while let Some(&c) = chars.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            name.push(c);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                match name.as_str() {
                    "let" => tokens.push(Token::Let),
                    "fn" => tokens.push(Token::Fn),
                    _ => tokens.push(Token::Name { name }),
                }
            }
            '0'..='9' => {
                let mut value = String::new();
                value.push(c);
                while let Some(&c) = chars.peek() {
                    match c {
                        '0'..='9' => {
                            value.push(c);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                tokens.push(Token::Int { value });
            }
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '=' => tokens.push(Token::Equal),
            ';' => tokens.push(Token::SemiColon),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            _ => {}
        }
    }

    tokens.push(Token::EndOfFile);
    tokens
}

#[derive(Debug, PartialEq)]
enum Token {
    Name { name: String },
    UpName { name: String },
    Int { value: String },
    Colon,
    Comma,
    Equal,
    SemiColon,
    Let,
    Fn,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    EndOfFile,
}
