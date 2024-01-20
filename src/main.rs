use std::usize;
use std::io;

#[derive(Debug)]
struct WordCounter{
    text: String,
}

trait Words {
    fn new(text: &str) -> WordCounter;

    fn count_words(&self) -> usize;
 
}

impl Words for WordCounter {
    fn new(text: &str) -> WordCounter {
        let new_str = WordCounter{
            text:String::from(text)
        };
        new_str
    }

    fn count_words(&self) -> usize {
        let words: Vec<&str> = self.text.split_ascii_whitespace().collect();
        words.len()
    }
}


fn main() {
    println!("Please Enter a text: ");

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let word_counter = WordCounter::new(&user_input);

    let counter = word_counter.count_words();
    
    if counter == 0 {
        println!("The counter is zero!")
    }

}
