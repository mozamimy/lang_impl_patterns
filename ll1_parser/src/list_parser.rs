use ll1_lexer::list_lexer;
use ll1_lexer::token;

pub struct ListParser {
    input: list_lexer::ListLexer,
    lookahead: Option<token::Token>,
}

impl ListParser {
    pub fn new(mut input: list_lexer::ListLexer) -> ListParser {
        let initial_token = input.next_token();
        ListParser {
            input: input.clone(),
            lookahead: Some(initial_token.clone()),
        }
    }

    // list: '[' elements ']'
    pub fn list(&mut self) {
        self.match_token(token::TokenType::LBrack);
        self.elements();
        self.match_token(token::TokenType::RBrack);
    }

    // elements: element (',' element)*
    fn elements(&mut self) {
        self.element();
        while self.lookahead.clone().unwrap().token_type == token::TokenType::Comma {
            self.match_token(token::TokenType::Comma);
            self.element();
        }
    }

    // element: name | list;
    fn element(&mut self) {
        match self.lookahead.clone() {
            Some(ref l) => if l.token_type == token::TokenType::Name {
                                self.match_token(token::TokenType::Name);
                            } else if l.token_type == token::TokenType::LBrack {
                                self.list();
                            },
            _ => panic!("expecting name or list; found {:?}", self.lookahead),
        }
    }

    fn match_token(&mut self, token_type: token::TokenType) {
        match self.lookahead.clone() {
            Some(ref l) if l.token_type == token_type => self.consume(),
            _ => panic!("expecting {:?}; found {:?}", token_type, self.lookahead.clone()),
        }
    }

    fn consume(&mut self) {
        self.lookahead = Some(self.input.next_token());
    }
}
