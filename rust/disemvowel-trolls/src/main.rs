fn main() {
    let s = "This website is for losers LOL!";
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut s_without_vowels = String::from("");

    for s_char in s.chars() {
        if !vowels.contains(&s_char.to_ascii_lowercase()) {
            s_without_vowels.push(s_char);
        }
    }

    println!("{}", s_without_vowels);
}
