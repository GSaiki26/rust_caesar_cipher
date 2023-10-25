// Libs
use std::io::stdin;

// Functions
fn main() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    // Get the user's input.
    println!("Type some text: ");
    let mut input: String = String::new();
    let _ = stdin().read_line(&mut input);

    // Loop through all the alphabet possibillities.
    for index in 0..26 {
        let mut phrase: String = String::new();

        // Loop through all the input's chars.
        for input_char in input.chars() {
            // Convert the char to lowercase and search by the index from alphabet.
            let char_index_in_alph: Option<usize> = alphabet.find(input_char.to_ascii_lowercase());
            if char_index_in_alph.is_none() {
                phrase.push(input_char);
                continue;
            }

            // Add X (current index from the alphabet) to the char index.
            let mut char_index_in_alph = char_index_in_alph.unwrap() + index;
            if char_index_in_alph >= 26 { char_index_in_alph -= 26; }

            // Add to the phrase.
            // println!("{}", char_index_in_alph);
            phrase.push(alphabet.chars().nth(char_index_in_alph).unwrap());
        }

        println!("Result [{0}]: {1}", index, phrase);
    }
}
