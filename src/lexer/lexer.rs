use super::super::token;

const ZERO_CHAR: char = '\u{0}';

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
  input: std::str::Chars<'a>,
  cur_char: char,
  peek_char: char,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    let mut l = Lexer {
      input: input.chars(),
      cur_char: ' ',
      peek_char: ' ',
    };

    l.read_char();
    l.read_char();
    l
  }

  pub fn next_token(&mut self) -> token::token::Token {
    match self.cur_char {
      '#' => self.hash_lexer(),
      '!' => self.img_lexer(),
      '[' => self.link_lexer(),
      '`' => self.code_lexer(),
      '-' => {
        self.read_char(); // self
        self.read_char(); // space
        let str = self.read_str();
        token::token::Token::List(str)
      }
      '*' => {
        self.read_char(); // self
        self.read_char(); // space
        let str = self.read_str();
        token::token::Token::List(str)
      }
      '>' => {
        self.read_char(); // self
        self.read_char(); // space
        let str = self.read_str();
        token::token::Token::Blockquote(str)
      }
      ZERO_CHAR => token::token::Token::Eof,
      _ => {
        let str = self.read_str();
        token::token::Token::Identifier(str)
      }
    }
  }

  //
  fn read_char(&mut self) -> char {
    let return_char = self.cur_char;
    self.cur_char = self.peek_char;
    self.peek_char = match self.input.next() {
      Some(c) => c,
      None => ZERO_CHAR,
    };
    return_char
  }

  fn read_str(&mut self) -> String {
    let mut str = String::new();
    while self.cur_char != '\n' {
      str.push(self.cur_char);
      self.read_char();
    }
    self.read_char();
    str
  }

  fn read_str_till(&mut self, ch: char) -> String {
    let mut str = String::new();
    while self.cur_char != ch {
      str.push(self.cur_char);
      self.read_char();
    }
    self.read_char();
    str
  }

  // LEXERs

  pub fn hash_lexer(&mut self) -> token::token::Token {
    if self.peek_char != ' ' && self.peek_char != '#' {
      self.read_str();
      return token::token::Token::Illegal;
    }

    let mut hashes: usize = 1;
    self.read_char();
    if self.cur_char == '#' {
      // ##
      self.read_char();
      hashes = 2;
      if self.cur_char == '#' {
        // ###
        hashes = 3;
        self.read_char();
      }
    }

    self.read_char(); // space

    let str = self.read_str();
    match hashes {
      1 => token::token::Token::H1(str),
      2 => token::token::Token::H2(str),
      3 => token::token::Token::H3(str),
      _ => panic!("Invalid number of hashes"),
    }
  }

  pub fn link_lexer(&mut self) -> token::token::Token {
    self.read_char(); // [
    let alt = self.read_str_till(']');
    self.read_char(); // (
    let src = self.read_str_till(')');
    self.read_char();
    token::token::Token::Link { alt, src }
  }

  pub fn img_lexer(&mut self) -> token::token::Token {
    if self.peek_char != '[' {
      return token::token::Token::Identifier(self.read_str());
    }

    self.read_char(); // self
    self.read_char(); // [

    let alt = self.read_str_till(']');
    self.read_char(); // (
    let src = self.read_str_till(')');
    self.read_char();
    token::token::Token::Image { src, alt }
  }

  pub fn code_lexer(&mut self) -> token::token::Token {
    if self.peek_char != '`' {
      return token::token::Token::Identifier(self.read_str());
    }
    self.read_char(); // `
    self.read_char(); // `
    self.read_char(); // `
    let language = self.read_str_till('\n');
    let mut codes: Vec<String> = Vec::new();

    while self.cur_char != '`' {
      let code = self.read_str_till('\n');
      codes.push(code);
    }

    self.read_str(); // ```
    token::token::Token::Code { language, codes }
  }
}
