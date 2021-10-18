mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

/// parses text to json AST
#[wasm_bindgen]
pub fn parse_text(s: &str, enable_markdown: bool) -> JsValue {
    let ast = match enable_markdown {
        true => dc_message_parser::parser::parse_markdown_text(s),
        false => dc_message_parser::parser::parse_only_text(s),
    };
    JsValue::from_serde(&ast).expect("json serializes to string")
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type ParsedElement =
  | { t: "Text"; c: string }
  | { t: "Tag"; c: string }
  | { t: "Linebreak" }
  | { t: "Bold"; c: ParsedElement[] }
  | { t: "Italics"; c: ParsedElement[] }
  | { t: "StrikeThrough"; c: ParsedElement[] }
  | { t: "InlineCode"; c: { content: string } }
  | { t: "CodeBlock"; c: { language: null | string; content: string } }
  | { t: "EmailAddress"; c: string }
  | { t: "Link"; c: { destination: string } }
  | { t: "LabeledLink"; c: { label: ParsedElement[]; destination: string } };
"#;
