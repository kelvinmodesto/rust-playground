fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{:#?}", element);
    // }

    // Iterators Consumers
    // elements
    //     .iter()
    //     .for_each(|element| println!("{:#?}", element));

    // Iterators Adapters
    elements
        .iter()
        .map(|element| format!("{} {}", element, element))
        .for_each(|element| println!("{:#?}", element));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(fallback), |el| el.to_string())
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    let mut colors_iter = colors.iter();

    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());

    shorten_strings(&mut colors);
    print_elements(&colors);

    let uppercased = to_uppercase(&colors);
    print_elements(&uppercased);

    let mut dest = vec![];
    move_elements(colors, &mut dest);
    println!("{:#?}", dest);

    let bomb = explode(&colors);
    println!("{:#?}", bomb);

    let result = find_color_or(&colors, "asdf", "orange");
    println!("{}", result);
}
