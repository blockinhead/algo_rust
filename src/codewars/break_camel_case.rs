// use unicode_segmentation::UnicodeSgmentation;

fn solution_(s: &str) -> String {
    let mut vec:Vec<String> = Vec::new();
    for c in s.chars() {
        if !c.is_lowercase() {
            vec.push(" ".to_string());
        }
        vec.push(c.to_string());
    }
    // println!("{:?}", vec);
    vec.into_iter().collect()
}

fn solution(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_lowercase() { format!("{}", c)} else { format!(" {}", c) })
        .collect()
}

fn main() {
    assert_eq!(solution("camelCasing"), "camel Casing");
    assert_eq!(solution("camelCasingTest"), "camel Casing Test");

}
