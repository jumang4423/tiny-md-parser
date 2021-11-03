
#[derive(Debug, Clone)]
pub enum Token {
  Identifier(String),
  H1(String),
  H2(String),
  H3(String),
  List(String),
  Code{
    lang: String,
    code: String,
  },
  Emphasis(String),
  Image{
    src: String,
    alt: String,
  },
  Link{
    src: String,
    alt: String,
  },
  Eof
}