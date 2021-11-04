mod lexer;
mod parser;
mod token;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn render_md(_lines: String) -> String {
  // add \n in the last sentense in _lines
  let lines = _lines.clone() + "\n";
  // lexer lines
  let lexer = lexer::lexer::Lexer::new(lines.as_str());
  let mut p = parser::parser::Parser::new(lexer);

  p.get_html()
}