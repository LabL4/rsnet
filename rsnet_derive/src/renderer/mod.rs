use proc_macro::TokenStream;
use syn::Expr;

pub fn include_shader(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as Expr);
    let shader_path = match input {
        Expr::Lit(lit) => match lit.lit {
            syn::Lit::Str(s) => s.value(),
            _ => panic!("Expected a string literal"),
        },
        _ => panic!("Expected a string literal"),
    };

    let shader_base_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/shaders/");
    let include_base_path = format!("{}{}", shader_base_path, "/include/");

    let shader_path = format!("{}{}", shader_base_path, shader_path);

    let orig_shader_src = std::fs::read_to_string(shader_path).unwrap();
    let mut shader_src = String::new();

    for line in orig_shader_src.lines() {
        if line.starts_with("//!include") {
            let split_line = line.split_whitespace().collect::<Vec<&str>>();
            let include_file = split_line.get(1).unwrap();
            let mut include_str = std::fs::read_to_string(format!(
                "{}{}{}",
                include_base_path, include_file, ".wgsl"
            ))
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to read include file: {}",
                    format!("{}{}", include_file, ".wgsl")
                )
            });

            if split_line.len() > 2 {
                // Parse line of the type $bg=0 and substite $bg
                split_line[2..].iter().for_each(|&s| {
                    let split = s.split('=').collect::<Vec<&str>>();
                    let var = split.get(0).unwrap();
                    let value = split.get(1).unwrap();
                    include_str = include_str.replace(var, value);
                });
            }

            shader_src += &include_str;
        } else {
            shader_src += line;
        }

        shader_src += "\n";
    }

    let output = quote::quote! {
        #shader_src
    };
    output.into()
}
