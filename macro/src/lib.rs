use proc_macro2::{
    TokenStream,
    TokenTree,
    Ident,
    Delimiter,
    Span,
};
use quote::{quote, quote_spanned};


#[proc_macro_attribute]
pub fn wasm_test(
    _attr: proc_macro::TokenStream,
    body: proc_macro::TokenStream
    ) -> proc_macro::TokenStream 
{
    let body = TokenStream::from(body);
    match wasm_test_with_error(body){
        Ok(t) => t.into(),
        Err(t) => t.into()
    }
}

fn wasm_test_with_error(body: TokenStream) 
    -> Result<TokenStream, TokenStream> {

    let mut body = body.into_iter();

    let mut result : Vec<TokenTree> = Vec::new();

    let panic_strategy = collect_tokens_before_function(&mut body, &mut result)?;
    if panic_strategy == PanicStrategy::ShouldPanic {
        return Err(compile_error(
                Span::call_site(),
                "don't support `should_panic` yet"
                )
            )
    }

    let func_name = find_ident(&mut body)
        .expect("cannot find function identifier");

    let generated_name = Ident::new(
        &format!("__wasm_test_{}", func_name),
        Span::call_site()
    );

    let generated_func = quote!{
        #[test]
        #[no_mangle]
        pub extern "C" fn #generated_name
    };

    result.extend(generated_func.into_iter());
    result.extend(body);
    Ok(result
        .into_iter()
        .collect::<TokenStream>()
    )
}

#[derive(PartialEq)]
enum PanicStrategy {
    Classic,
    ShouldPanic,
}

fn collect_tokens_before_function (
    body: &mut impl Iterator<Item=TokenTree>,
    tokens: &mut Vec<TokenTree>) 
    -> Result<PanicStrategy, TokenStream>
{
    let mut strategy = PanicStrategy::Classic;

    while let Some(x) = body.next() {
        match x {
            TokenTree::Ident(token) if token=="fn" => return Ok(strategy),
            TokenTree::Ident(_)  => return Err(compile_error(
                x.span(),
                "the only valid keyword here is `fn`"
                )),
            TokenTree::Punct(p) if p.as_char() == '#' => {
                let attribute = body.next().expect("# punctuation here makes no sense");
                
                if is_should_panic(&attribute)? {
                    strategy = PanicStrategy::ShouldPanic;
                }
                else {
                    tokens.push(attribute);
                }
            },
            _ => tokens.push(x),
        }
    }
    Err(compile_error(Span::call_site(), "function don't have a body"))
}

fn is_should_panic(t: &TokenTree) -> Result<bool, TokenStream> {
    match t {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
            let mut content = group.stream().into_iter();
            match content.next().unwrap() {
                TokenTree::Ident(s) if s == "should_panic" => {
                    if content.next().is_none(){
                        Ok(true)
                    }
                    else {
                        Err(compile_error(
                            group.span(),
                            "don't support arguments given to `should_panic`"
                        ))
                    }
                },
                _ => Ok(false)
            }
        }
        _ => Ok(false)
    }
}

fn find_ident(iter: &mut impl Iterator<Item = TokenTree>) -> Option<Ident> {
    match iter.next()? {
        TokenTree::Ident(i) => Some(i),
        TokenTree::Group(g) if g.delimiter() == Delimiter::None => {
            find_ident(&mut g.stream().into_iter())
        }
        _ => None,
    }
}

fn compile_error(span: Span, msg: &str) -> TokenStream {
    quote_spanned! { span => compile_error!(#msg); }.into()
}
