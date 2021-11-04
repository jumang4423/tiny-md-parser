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

    html.push_str("
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
<link rel=\"stylesheet\" href=\"https://raw.githubusercontent.com/sindresorhus/github-markdown-css/main/github-markdown-light.css\">
<style>
  .markdown-body {
    box-sizing: border-box;
    min-width: 200px;
    max-width: 980px;
    margin: 0 auto;
    padding: 45px;
  }

  @media (max-width: 767px) {
    .markdown-body {
      padding: 15px;
    }
  }
</style>\n");

    html.push_str("<article class=\"markdown-body\">\n");

    while self.cur_token != token::token::Token::Eof {
      html.push_str(self.parse_html().as_str());
      self.next_token();
    }

    html.push_str("</article>\n");
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
      token::token::Token::Blockquote(str) => {
        html = format!("  <blockquote><p>{}</p></blockquote>\n", str);
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
      token::token::Token::NumList{num, identifier} => {
        html = format!("  <ol class=\"numbered-list\">{}. {}</ol>\n", num, identifier);
      }
      token::token::Token::Code { language: _, codes } => {
        html.push_str("  <pre>\n");

        html.push_str("    <code>\n");
        codes.iter().for_each(|code| {
          html.push_str("       ");
          html.push_str(code);
          html.push_str("\n");
        });
        html.push_str("    </code>\n");
        html.push_str("  </pre>\n");
      }
      token::token::Token::Image { src, alt } => {
        html = format!("  <img width=\"100%\" src=\"{}\" alt=\"{}\"/>\n", src, alt);
      }
      token::token::Token::Link { src, alt } => {
        html = format!(
          "  <a href=\"{}\" alt=\"{}\" target=\"_blank\"> {} </a>\n",
          src, alt, alt
        );
      }
      _ => {
        html = format!("{:?}", self.cur_token);
      }
    }
    html
  }
}
