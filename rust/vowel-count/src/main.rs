fn main() {
    let input = "abracadabra";
    let mut vowels_count: usize = 0;

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for input_char in input.chars() {
        if vowels.contains(&input_char) {
            vowels_count += 1
        }
    }

    println!("{}", vowels_count)
}
