use super::super::lexer;
use super::super::token;

#[derive(Debug)]
pub struct Parser<'a> {
  l: lexer::lexer::Lexer<'a>,
  cur_token: token::token::Token,
  peek_token: token::token::Token,
}

impl<'a> Parser<'a> {
  ////////////////////////////////////////////////////////////////////////////////
  ///
  /// // next_token
  ///
  ////////////////////////////////////////////////////////////////////////////////
  fn next_token(&mut self) {
    self.cur_token = self.peek_token.clone();
    self.peek_token = self.l.next_token();
  }

  // seek the lexer then make ast for that
  pub fn new(l: lexer::lexer::Lexer<'a>) -> Self {
    let mut p = Parser {
      l,
      cur_token: token::token::Token::Illegal,
      peek_token: token::token::Token::Illegal,
    };
    p.next_token();
    p.next_token();
    p
  }

  pub fn get_html(&mut self) -> String {
    let mut html = String::new();
    html.push_str("<body>\n");

    while self.cur_token != token::token::Token::Eof {
      html.push_str(self.parse_html().as_str());
      self.next_token();
    }

    html.push_str("</body>\n");
    html
  }

  pub fn parse_html(&mut self) -> String {
    let mut html = String::new();
    match &self.cur_token {
      token::token::Token::H1(str) => {
        html = format!("  <h1>{}</h1>\n", str);
      }
      token::token::Token::H2(str) => {
        html = format!("  <h2>{}</h2>\n", str);
      }
      token::token::Token::H3(str) => {
        html = format!("  <h3>{}</h3>\n", str);
      }
      token::token::Token::Identifier(str) => {
        if str == "" {
          html = format!("  <br/>\n");
        } else {
          html = format!("  <div>{}</div>\n", str);
        }
      }
      token::token::Token::List(str) => {
        html = format!("  <li> {} </li>\n", str);
      }
      token::token::Token::Code{language, codes} => {

        codes.iter().for_each(|code| {
          html.push_str("  <pre>\n");
          html.push_str("    <code class=\"language-");
          html.push_str(language);
          html.push_str("\">\n      ");
          html.push_str(code);
          html.push_str("\n    </code>\n");
          html.push_str("  </pre>\n");
        });
      }
      token::token::Token::Image { src, alt } => {
        html = format!("  <img src=\"{}\" alt=\"{}\"/>\n", src, alt);
      }
      token::token::Token::Link { src, alt } => {
        html = format!("  <a src=\"{}\" alt=\"{}\"/>\n", src, alt);
      }
      _ => {}
    }
    html
  }
}
