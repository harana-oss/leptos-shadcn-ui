//! Procedural macros for tailwind-rs-core.

use quote::quote;
use syn::{parse_macro_input, Expr, Token};

/// Macro for creating type-safe Tailwind classes with compile-time validation.
/// 
/// # Examples
/// 
/// ```rust
/// use tailwind_rs_core_macros::classes;
///
/// // String literals are joined at compile time into a `&'static str`
/// let classes = classes! {
///     base: "px-4 py-2 rounded-md font-medium",
///     variant: "bg-blue-600 text-white hover:bg-blue-700",
///     responsive: "sm:text-sm md:text-base lg:text-lg",
/// };
/// assert_eq!(
///     classes,
///     "px-4 py-2 rounded-md font-medium bg-blue-600 text-white hover:bg-blue-700 sm:text-sm md:text-base lg:text-lg"
/// );
///
/// // Any other expression is converted with `to_string` at runtime
/// let shade = 600;
/// let classes = classes! {
///     base: "rounded-md",
///     background: format!("bg-blue-{shade}"),
/// };
/// assert_eq!(classes, "rounded-md bg-blue-600");
/// ```
#[proc_macro]
pub fn classes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ClassesInput);

    // Output order: base, variants, responsive, states, then everything else.
    let mut fields: Vec<(u8, Expr)> = input
        .fields
        .into_iter()
        .map(|field| {
            let rank = match field.name.as_str() {
                "base" => 0,
                "variant" | "variants" => 1,
                "responsive" | "responsive_classes" => 2,
                "state" | "states" | "hover" | "focus" | "active" | "disabled" => 3,
                _ => 4,
            };
            (rank, field.value)
        })
        .collect();
    fields.sort_by_key(|(rank, _)| *rank);
    let values: Vec<Expr> = fields.into_iter().map(|(_, value)| value).collect();

    // All string literals: join at compile time into a `&'static str`.
    if let Some(literals) = values.iter().map(string_literal).collect::<Option<Vec<_>>>() {
        let joined = join_non_empty(literals);
        return quote! { #joined }.into();
    }

    // Otherwise build a `String` at runtime from each value's `Display` output.
    quote! {
        {
            let parts: ::std::vec::Vec<::std::string::String> =
                ::std::vec![#(::std::string::ToString::to_string(&(#values))),*];
            parts
                .iter()
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .collect::<::std::vec::Vec<_>>()
                .join(" ")
        }
    }
    .into()
}

/// Returns the value of a string literal expression.
fn string_literal(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) => Some(s.value()),
        _ => None,
    }
}

/// Joins class strings with single spaces, skipping empty ones.
fn join_non_empty(parts: impl IntoIterator<Item = String>) -> String {
    parts
        .into_iter()
        .map(|part| part.trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Prefixes every whitespace-separated class in `classes` with `breakpoint:`.
fn prefix_classes(breakpoint: &str, classes: &str) -> String {
    classes
        .split_whitespace()
        .map(|class| format!("{breakpoint}:{class}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Macro for creating responsive Tailwind classes.
/// 
/// # Examples
/// 
/// ```rust
/// use tailwind_rs_core_macros::responsive;
/// 
/// let responsive_classes = responsive! {
///     sm: "text-sm",
///     md: "text-base font-medium",
///     lg: "text-lg",
/// };
/// assert_eq!(responsive_classes, "sm:text-sm md:text-base md:font-medium lg:text-lg");
/// ```
#[proc_macro]
pub fn responsive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ResponsiveInput);

    let literals: Option<Vec<String>> = input
        .fields
        .iter()
        .map(|field| string_literal(&field.value).map(|v| prefix_classes(&field.name, &v)))
        .collect();
    if let Some(literals) = literals {
        let joined = join_non_empty(literals);
        return quote! { #joined }.into();
    }

    let breakpoints: Vec<String> = input.fields.iter().map(|field| field.name.clone()).collect();
    let values: Vec<&Expr> = input.fields.iter().map(|field| &field.value).collect();
    quote! {
        {
            let parts: ::std::vec::Vec<(&str, ::std::string::String)> =
                ::std::vec![#((#breakpoints, ::std::string::ToString::to_string(&(#values)))),*];
            parts
                .iter()
                .flat_map(|(breakpoint, classes)| {
                    classes
                        .split_whitespace()
                        .map(move |class| ::std::format!("{}:{}", breakpoint, class))
                })
                .collect::<::std::vec::Vec<_>>()
                .join(" ")
        }
    }
    .into()
}

/// Macro for creating theme-based Tailwind classes.
/// 
/// # Examples
/// 
/// ```rust
/// use tailwind_rs_core_macros::theme;
/// 
/// let theme_classes = theme! {
///     variant: "primary",
///     size: "md",
///     additional: "rounded-md shadow-lg",
/// };
/// ```
#[proc_macro]
pub fn theme(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ThemeInput);
    
    let mut variant_classes = String::new();
    let mut size_classes = String::new();
    let mut additional_classes = String::new();
    
    for field in input.fields {
        match field.name.as_str() {
            "variant" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        variant_classes = match s.value().as_str() {
                            "primary" => "bg-blue-600 text-white hover:bg-blue-700".to_string(),
                            "secondary" => "bg-gray-200 text-gray-900 hover:bg-gray-300".to_string(),
                            "success" => "bg-green-600 text-white hover:bg-green-700".to_string(),
                            "warning" => "bg-yellow-600 text-white hover:bg-yellow-700".to_string(),
                            "error" => "bg-red-600 text-white hover:bg-red-700".to_string(),
                            "info" => "bg-sky-600 text-white hover:bg-sky-700".to_string(),
                            "outline" => "border-2 border-blue-600 text-blue-600 hover:bg-blue-50".to_string(),
                            "ghost" => "bg-transparent hover:bg-blue-100".to_string(),
                            "link" => "text-blue-600 underline hover:text-blue-700".to_string(),
                            "destructive" => "bg-red-600 text-white hover:bg-red-700".to_string(),
                            _ => String::new(),
                        };
                    }
                }
            }
            "size" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        size_classes = match s.value().as_str() {
                            "xs" => "px-2 py-1 text-xs".to_string(),
                            "sm" => "px-3 py-1.5 text-sm".to_string(),
                            "md" => "px-4 py-2 text-base".to_string(),
                            "lg" => "px-6 py-3 text-lg".to_string(),
                            "xl" => "px-8 py-4 text-xl".to_string(),
                            _ => String::new(),
                        };
                    }
                }
            }
            "additional" | "extra" | "custom" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        additional_classes = s.value();
                    }
                }
            }
            _ => {}
        }
    }
    
    let mut all_classes = vec![variant_classes, size_classes, additional_classes];
    all_classes.retain(|s| !s.is_empty());
    
    let final_classes = all_classes.join(" ");
    
    quote! {
        #final_classes
    }.into()
}

/// Macro for creating color-based Tailwind classes.
/// 
/// # Examples
/// 
/// ```rust
/// use tailwind_rs_core_macros::color;
/// 
/// let color_classes = color! {
///     color: "blue",
///     shade: 600,
///     variants: ["background", "text", "hover"],
/// };
/// ```
#[proc_macro]
pub fn color(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ColorInput);
    
    let mut color_name = String::new();
    let mut shade = 600u16;
    let mut variants = Vec::new();
    
    for field in input.fields {
        match field.name.as_str() {
            "color" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        color_name = s.value();
                    }
                }
            }
            "shade" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Int(i) = &lit.lit {
                        shade = i.base10_parse::<u16>().unwrap_or(600);
                    }
                }
            }
            "variants" => {
                if let Expr::Array(arr) = &field.value {
                    for elem in &arr.elems {
                        if let Expr::Lit(lit) = elem {
                            if let syn::Lit::Str(s) = &lit.lit {
                                variants.push(s.value());
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    
    let mut classes = Vec::new();
    
    for variant in variants {
        let class = match variant.as_str() {
            "background" | "bg" => format!("bg-{}-{}", color_name, shade),
            "text" => format!("text-{}-{}", color_name, shade),
            "border" => format!("border-{}-{}", color_name, shade),
            "hover" => format!("hover:bg-{}-{}", color_name, shade),
            "focus" => format!("focus:ring-{}-{}", color_name, shade),
            "shadow" => format!("shadow-{}-{}", color_name, shade),
            _ => continue,
        };
        classes.push(class);
    }
    
    let final_classes = classes.join(" ");
    
    quote! {
        #final_classes
    }.into()
}

/// Input structure for the classes macro.
struct ClassesInput {
    fields: Vec<ClassField>,
}

/// Input structure for the responsive macro.
struct ResponsiveInput {
    fields: Vec<ClassField>,
}

/// Input structure for the theme macro.
struct ThemeInput {
    fields: Vec<ClassField>,
}

/// Input structure for the color macro.
struct ColorInput {
    fields: Vec<ClassField>,
}

/// A field in a macro input.
struct ClassField {
    name: String,
    value: Expr,
}

impl syn::parse::Parse for ClassesInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();
        
        while !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: Expr = input.parse()?;
            
            fields.push(ClassField {
                name: name.to_string(),
                value,
            });
            
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        
        Ok(ClassesInput { fields })
    }
}

impl syn::parse::Parse for ResponsiveInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();
        
        while !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: Expr = input.parse()?;
            
            fields.push(ClassField {
                name: name.to_string(),
                value,
            });
            
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        
        Ok(ResponsiveInput { fields })
    }
}

impl syn::parse::Parse for ThemeInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();
        
        while !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: Expr = input.parse()?;
            
            fields.push(ClassField {
                name: name.to_string(),
                value,
            });
            
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        
        Ok(ThemeInput { fields })
    }
}

impl syn::parse::Parse for ColorInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();
        
        while !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: Expr = input.parse()?;
            
            fields.push(ClassField {
                name: name.to_string(),
                value,
            });
            
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        
        Ok(ColorInput { fields })
    }
}
