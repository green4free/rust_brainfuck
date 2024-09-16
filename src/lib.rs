extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Token, parse_macro_input, LitStr, LitInt, LitBool};
use syn::parse::{Parse, ParseStream};
use proc_macro2::Span; 

use std::collections::VecDeque;

struct MacroInput {
    fn_name: LitStr,
    brainfuck_code: LitStr,
    buffer_size: usize,
    debug: LitBool
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fn_name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let brainfuck_code: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let buffer_size_raw: LitInt = input.parse()?;
        let buffer_size_parsed: usize = buffer_size_raw.base10_parse::<usize>()?;
        let buffer_size: usize = if buffer_size_parsed > 0 {buffer_size_parsed} else {30000};
        input.parse::<Token![,]>()?;
        let debug: LitBool = input.parse()?;
        Ok(MacroInput {fn_name, brainfuck_code, buffer_size, debug})
    }
}

fn parse(start_index:usize, level:usize, debug: bool, code_input: &[u8], code_output: &mut Vec<proc_macro2::TokenStream>) -> usize {
    let mut index = start_index;
    while index < code_input.len() {
        let c = code_input[index];
        if debug {
            let debugc = c as char;
            code_output.push(quote! {println!("Debug {} {}: {} : {}", #debugc, #index, i, data[i]);});
        }
        index += 1;
        match c {
            b'>' => code_output.push(quote! {
                i+=1;
                if i >= data.len() {
                    panic!("Pointer out of bounds: i = {}! Level {}, Instruction {}", i, #level, #index);
                }
            }),
            b'<' => code_output.push(quote! {
                if i == 0 {
                    panic!("Attempt to move pointer below zero! Level {}, Instruction {}", #level, #index);
                }
                i-=1;
            }),
            b'+' => code_output.push(quote! {data[i]=data[i].wrapping_add(1);}),
            b'-' => code_output.push(quote! {data[i]=data[i].wrapping_sub(1);}),
            b'.' => code_output.push(quote! {out_data.push(data[i]);}),
            b',' => code_output.push(quote! {data[i] = in_data.pop_front().unwrap();}),
            b'[' => {
                let mut next_block:Vec<proc_macro2::TokenStream> = Vec::new();
                index = parse(index, level+1, debug, code_input, &mut next_block);
                code_output.push(quote! {
                    while (data[i] != 0){
                        #(#next_block)*
                    }
                })
            },
            b']' => {return index;},
            _ => {}
        }
    }

    if level != 0 {
        panic!("Brainfuck code ended without closing brackets! Level {}", level);
    }

    index
}


#[proc_macro]
pub fn make_brainfuck_function(input: TokenStream) -> TokenStream {
    let MacroInput {fn_name, brainfuck_code, buffer_size, debug} = parse_macro_input!(input as MacroInput);
    let fn_name_str = fn_name.value();
    let brainfuck_code_str = brainfuck_code.value();
    let fn_name_ident = syn::Ident::new(&fn_name_str, proc_macro2::Span::call_site());

    let mut code: Vec<proc_macro2::TokenStream> = Vec::new();
    parse(0, 0, debug.value(), & brainfuck_code_str.as_bytes(), &mut code);

    let expanded = quote! {
        fn #fn_name_ident(in_data: &mut VecDeque<u8>, out_data: &mut Vec<u8>) {
            let mut i: usize = 0;
            let mut data = vec![0 as u8; #buffer_size];
            #(#code)*
        }
    };
    TokenStream::from(expanded)
}
