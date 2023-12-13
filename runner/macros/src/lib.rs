use std::sync::atomic::{AtomicU64, Ordering};

use proc_macro::*;
use syn::{parse::Parse, parse_macro_input, Ident, ItemFn, Token};

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
                input: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), #path))
            };
        };

        #func
    }
    .into()
}

#[allow(unused)]
fn download_problem(year: u64, day: u64) {
    let path = format!("https://adventofcode.com/{year}/day/{day}/input");
    let output_path = format!("input/{year}/day{day}.txt");

    if std::fs::metadata(&output_path).is_ok() {
        //return;
    }

    let entry = keyring::Entry::new("aoc_runner", &whoami::username()).unwrap();
    let token = entry.get_password().expect("No token found");

    println!("{token}");
    //return;
    let mut c = std::process::Command::new("curl");
    c.arg("-s")
        .arg("--cookie")
        .arg(format!("\"session={token}\""))
        .arg("-o")
        .arg(output_path)
        .arg(path);

    println!("{c:?}");
    c.output().unwrap();
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

        let d = u64::from_str_radix(&d[3..], 10);
        let p = u64::from_str_radix(&p[4..], 10);
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
    rest: TokenStream,
}

impl Parse for Year {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let i = input.parse::<Ident>()?;
        if i.to_string() != "year" {
            return Err(syn::Error::new(i.span(), "Expected \"year\""));
        }

        input.parse::<Token![=]>()?;
        let yearl = input.parse::<proc_macro2::Literal>()?;
        let year = u64::from_str_radix(&yearl.to_string(), 10);
        let year = match year {
            Ok(u) => u,
            Err(_) => return Err(syn::Error::new(yearl.span(), "Expected number")),
        };

        input.parse::<Token![,]>()?;

        let rest = input.parse::<proc_macro2::TokenStream>()?;

        Ok(Self {
            year,
            rest: rest.into(),
        })
    }
}

#[proc_macro]
pub fn aoc_mod(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Year);
    CURRENT_YEAR.store(input.year, Ordering::SeqCst);
    input.rest
}
