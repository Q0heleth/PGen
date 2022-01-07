mod query;
#[test]
fn reverse_words(str: &str) -> String {
    // your code here
    str.to_string().
        split(" ").
        map(|f|f.chars().rev().collect()).
        collect::<Vec<String>>().
        join(" ")

}