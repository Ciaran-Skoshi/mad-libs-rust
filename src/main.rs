

const MAD_LIBS_DIR: &'static str = "mad_libs";

fn main() {
    println!("Welcome to Skoshi's Mad Lib Terminal game!");

    let mad_libs_dir = std::path::Path::new(MAD_LIBS_DIR);

    if !mad_libs_dir.exists() {
        println!("No {MAD_LIBS_DIR} directory found...");
        println!("Making {MAD_LIBS_DIR} directory...");

        match std::fs::create_dir(mad_libs_dir){
            Ok(_) => println!("Successful made {MAD_LIBS_DIR}"),

            Err(e) => println!("Failed to create {MAD_LIBS_DIR}: {e}")
        }
    }

    if !mad_libs_dir.is_dir(){
        println!("{MAD_LIBS_DIR} found, but it appears not to be a directory, 
            please delete or rename it so the program can make the correct {MAD_LIBS_DIR}")
    }

    let mad_libs_iter: std::fs::ReadDir;

    match std::fs::read_dir(mad_libs_dir){
        Ok(entries) => mad_libs_iter = entries,

        Err(e) => {
            println!("Could not read {MAD_LIBS_DIR}: {e}");
            return;
        }
    }

    let mut x :i32 = 0;
    let mut mad_libs :Vec<std::fs::DirEntry> = vec![];
    for entry in mad_libs_iter {
        x += 1;
        match entry {
            Ok(entry) => {
                println!("{x}. {:?}", entry.file_name());
                mad_libs.push(entry);
            }
            Err(e) => {
                println!("Error: {e}");
                return;
            }
        }
    }

    let mut user_input_str: String = String::new();
    let mut user_input_usize: usize;
    loop {
        println!("Please enter the number of the Mad Lib you want to play");
        std::io::stdin()
                .read_line(&mut user_input_str)
                .expect("Failed to read line");
            
            /*
            ::<usize> is to denote the type we want from .parse()

            We are using usize because mad_libs.len returns a usize and we need to compare user_input_usize to it
            */
            match user_input_str.trim().parse::<usize>() {
                Ok(num) => {
                    if num <= mad_libs.len() && num > 0 {
                        user_input_usize = num - 1;
                        break;
                    }
                    else {
                        println!("Number is either too long or too big! Please make sure to enter 
                        a number greater than 0 but less than {}", mad_libs.len());
                        continue;
                    }
                }
                Err(_) => continue,
            };
        }


    let mut mad_lib: String = std::string::String::new();

    match std::fs::read_to_string(mad_libs[user_input_usize].path()){
        Ok(s) => mad_lib = s,

        Err(_) => {}
    }
    
    let mut fill_in_words: Vec<String> = vec![];

    match parse_mad_lib_file(mad_lib.clone()) {
        Ok(fill_in) => fill_in_words = fill_in,

        Err(e) => {
            match e {
                MadLibsParseError::NoFillInWords => {
                    println!("No fill in words found in the Mad Lib!");
                    return
                },
                MadLibsParseError::MissingClosingBrackets => {
                    println!("The last [ was not closed in the Mad Lib!");
                    return
                }
            }
        }
    }

    let mut filled_in_words :Vec<String> = vec![];

    match get_fill_in_words(fill_in_words.clone()) {

        Ok(ans) => filled_in_words = ans,

        Err(e) => {
            println!("Error getting fill in words: {e}");
            return;
        }

    }

    println!("Filled in words are: {:?}", filled_in_words.clone());

    display_mad_lib(mad_lib, fill_in_words, &mut filled_in_words);



}

enum MadLibsParseError {
    NoFillInWords,
    MissingClosingBrackets
}


fn parse_mad_lib_file(raw_mad_lib: String) -> Result<Vec<String>, MadLibsParseError> {
    let mut is_in_brackets: bool = false;
    let mut current_word: String = String::new();
    let mut fill_in_words: Vec<String> = vec![];

    for ch in raw_mad_lib.chars(){
        match ch {
            '[' => {
                is_in_brackets = true;
                continue;
            },
            ']' => {
                is_in_brackets = false;
                fill_in_words.push(current_word.clone());
                current_word.clear();
                continue;
            },

            _ => {}
        };

        if is_in_brackets {
            current_word.push(ch);
        }

    }

    
    if is_in_brackets {
        return Err(MadLibsParseError::MissingClosingBrackets)
    }
    else if fill_in_words.len() <= 0 {
        return Err(MadLibsParseError::NoFillInWords);
    }
    else {
        return Ok(fill_in_words);
    }
}

fn get_fill_in_words(fill_in_words: Vec<String>) -> Result<Vec<String>, std::io::Error> {

    let mut filled_in_words: Vec<String> = vec![];

    let mut current_word: String = String::new();

    let input = std::io::stdin();

    for fill_in_word in fill_in_words.iter() {

        println!("Please enter a {fill_in_word}");
        match input.read_line(&mut current_word) {
            Ok(_) => (),

            Err(e) => {
                return Err(e);
            }
        }
        
        // This is to remove the \n char at the end of the string
        current_word.pop();

        filled_in_words.push(current_word.clone());
        current_word.clear();

    };

    return Ok(filled_in_words);

    
}

fn display_mad_lib(mut raw_mad_lib: String, fill_in_words: Vec<String>, filled_in_words: &mut Vec<String>) {
    
    let mut iter_count: usize = 0;
    
    for word in filled_in_words.iter_mut() {
        if word == "" {
            *word = String::from("Word Not Entered");
        };

        let mut pattern :String = String::new();
        
        pattern.push('[');
        pattern.push_str(&fill_in_words[iter_count]);
        pattern.push(']');

        raw_mad_lib = raw_mad_lib.replacen(&pattern, word, 2);

        pattern.clear();

        println!("Fill in word is: {}", &fill_in_words[iter_count]);

        iter_count += 1;

    }

    println!("{}", raw_mad_lib);
}