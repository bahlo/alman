use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Fehl" => "Err",
        "Ok" => "Ok",
        "Zeichenkette" => "String",
        "Wörterbuch" => "HashMap",
        "Standard" => "Default",
        "Fehler" => "Error",
        "Vielleicht" => "Option",
        "Etwas" => "Some",
        "Nichts" => "None",
        "Ergebnis" => "Result",
        "Selbst" => "Self",
        "druck" => "println",
        "abbruch" => "break",
        "asynchrone" => "async",
        "warte" => "await",
        "wiederhole" => "loop",
        "schiebe" => "move",
        "kiste" => "crate",
        "als" => "as",
        "konstante" => "const",
        "zuordne" => "match",
        "unsicher" => "unsafe",
        "in" => "in",
        "von" => "from",
        "dynamisch" => "dyn",
        "auspacken" => "unwrap",
        "standard" => "default",
        "io" => "io",
        "extern" => "extern",
        "falsch" => "false",
        "funktion" => "fn",
        "super" => "super",
        "implementierung" => "impl",
        "einfügen" => "insert",
        "eigenschaft" => "trait",
        "holen" => "get",
        "baustein" => "mod",
        "wandelbar" => "mut",
        "neu" => "new",
        "wo" => "where",
        "für" => "for",
        "hol_oder_füg_ein_mit" => "get_or_insert_with",
        "haupt" => "main",
        "öffentlich" => "pub",
        "keins" => None?,
        "zurückgebe" => "return",
        "wenn" => "if",
        "sonst" => "else",
        "selbst" => "self",
        "lass" => "let",
        "staisch" => "static",
        "struktur" => "struct",
        "erwarte" => "expect",
        "während" => "while",
        "nutze" => "use",
        "wahr" => "true",
        "aufzählung" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn alman(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
