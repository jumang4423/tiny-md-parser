
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Identifier(String),
  H1(String),
  H2(String),
  H3(String),
  List(String),
  Blockquote(String),
  Code{
    language: String,
    codes: Vec<String>,
  },
  Image{
    src: String,
    alt: String,
  },
  Link{
    src: String,
    alt: String,
  },
  Illegal,
  Eof
}