fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;
    for lang in languages {
        if found {
            return lang;
        }
        if lang == current {
            found = true;
        }
    }
    languages.last().unwrap()
}

fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest<'a>(text_a: &'a str, text_b: &'a str) -> &'a str {
    if text_a.len() > text_b.len() {
        text_a
    } else {
        text_b
    }
}

fn main() {
    let languages = vec![
        String::from("Java"),
        String::from("Rust"),
        String::from("TypeScript"),
        String::from("GoLang"),
    ];

    let next = next_language(&languages, "Java");
    let last = last_language(&languages);
    let longest = longest("go", "rust");

    println!("{}", next);
    println!("{}", last);
    println!("{}", longest);
}
