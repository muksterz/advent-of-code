use std::sync::atomic::{AtomicU64, Ordering};

use proc_macro::*;
use syn::{parse::Parse, parse_macro_input, ItemFn, Token};

static CURRENT_YEAR: AtomicU64 = AtomicU64::new(0);

#[proc_macro_attribute]
pub fn aoc(attr: TokenStream, input: TokenStream) -> TokenStream {
    let p = parse_macro_input!(attr as Problem);
    let func = parse_macro_input!(input as ItemFn);

    let year = CURRENT_YEAR.load(Ordering::SeqCst);
    let day = p.day;
    let part = p.part;
    //download_problem(year, day);

    let i = func.sig.ident.clone();

    let path = format!("/input/{year}/day{day}.txt");

    quote::quote! {

        const _: () = {
            #[::runner::__internals::linkme::distributed_slice(::runner::PROBLEMS)]
            pub static LINK: ::runner::Problem = ::runner::Problem {
                year: #year,
                day: #day,
                part: #part,
                f: |i| #i(i).to_string(),
                input: concat!(env!("CARGO_MANIFEST_DIR"), #path)
            };
        };

        #func
    }
    .into()
}

struct Problem {
    day: u64,
    part: u64,
}

impl Parse for Problem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let day = input.parse::<proc_macro2::Ident>()?;
        input.parse::<Token![,]>()?;
        let part = input.parse::<proc_macro2::Ident>()?;

        let d = day.to_string();
        let p = part.to_string();

        if !d.starts_with("day") {
            return Err(syn::Error::new(day.span(), "Expected day"));
        }

        if !p.starts_with("part") {
            return Err(syn::Error::new(part.span(), "Expected part"));
        }

        let d = d[3..].parse();
        let p = p[4..].parse();
        let d = match d {
            Ok(u) => u,
            Err(_) => return Err(syn::Error::new(day.span(), "expected number")),
        };

        let p = match p {
            Ok(u) => u,
            Err(_) => return Err(syn::Error::new(day.span(), "expected number")),
        };

        Ok(Problem { day: d, part: p })
    }
}

struct Year {
    year: u64,
}

impl Parse for Year {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let yearl = input.parse::<proc_macro2::Literal>()?;
        let year = yearl.to_string().parse();
        let year = match year {
            Ok(u) => u,
            Err(_) => return Err(syn::Error::new(yearl.span(), "Expected number")),
        };

        Ok(Self { year })
    }
}

#[proc_macro]
pub fn aoc_year(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Year);
    CURRENT_YEAR.store(input.year, Ordering::SeqCst);
    TokenStream::new()
}
