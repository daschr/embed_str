extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn embed_str(in_stream: TokenStream) -> TokenStream {
    let lit = match in_stream.into_iter().next() {
        Some(TokenTree::Literal(l)) => l.to_string(),
        t => panic!("[embed_str2] {:?} not a string literal!", t),
    };

    let raw_lit = if lit.starts_with("\"") {
        lit.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap()
    } else if lit.starts_with("r#") {
        lit.strip_prefix("r#").unwrap().strip_suffix("#").unwrap()
    } else if lit.starts_with("r\"") {
        lit.strip_prefix("r\"").unwrap().strip_suffix("\"").unwrap()
    } else {
        &lit
    };

    let mut enc_lit = String::new();

    for c in raw_lit.chars() {
        enc_lit.push_str(&format!("\\\\x{:02x}", (c as u8) ^ 0xc8));
    }

    let payload = format!(
        "{{
        let mut s = String::new();
        decrypt((&mut s) as *mut _);

        asm!(\".string \\\"{}\\\"\");
        s
    }}",
        enc_lit
    );

    payload.parse().unwrap()
}
