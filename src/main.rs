
const MAD_LIBS_DIR: &'static str = "mad_libs";


struct MadLib {
    mad_lib :String,
    fill_in_words: Vec<String>,
    filled_in_words: Vec<String>
}

enum MadLibsParseError {
    NoFillInWords,
    MissingClosingBrackets
}

impl MadLib {
    fn parse_mad_lib(&mut self) -> Result<(), MadLibsParseError>{
        let mut is_in_brackets: bool = false;
        let mut current_word: String = String::new();

        for ch in self.mad_lib.chars(){
            match ch {
                '[' => {
                    is_in_brackets = true;
                    continue;
                },
                ']' => {
                    is_in_brackets = false;
                    self.fill_in_words.push(current_word.clone());
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
        else if self.fill_in_words.len() <= 0 {
            return Err(MadLibsParseError::NoFillInWords);
        };
        
        Ok(())

    }

    fn fill_in_words(&mut self) -> Result<(), std::io::Error> {
        
        let mut current_word: String = String::new();

        let input = std::io::stdin();

        for fill_in_word in self.fill_in_words.iter() {

            println!("Please enter a {fill_in_word}");
            match input.read_line(&mut current_word) {
                Ok(_) => (),

                Err(e) => {
                    return Err(e);
                }
            }
            
            // This is to remove the \n char at the end of the string
            current_word.pop();

            self.filled_in_words.push(current_word.clone());
            current_word.clear();
        };
        return Ok(())

    }

    fn display_mad_lib(&mut self) {
        let mut iter_count: usize = 0;
        
        for word in self.filled_in_words.iter_mut() {
            if word == "" {
                *word = String::from("Word Not Entered");
            };

            let mut pattern :String = String::new();
            
            pattern.push('[');
            pattern.push_str(&self.fill_in_words[iter_count]);
            pattern.push(']');

            self.mad_lib = self.mad_lib.replacen(&pattern, word, 1);

            pattern.clear();

            println!("Fill in word is: {}", &self.fill_in_words[iter_count]);

            iter_count += 1;

        }

        println!("{}", self.mad_lib);
    }
}

fn main() {
    println!("Welcome to Skoshi's Mad Lib Terminal game!");

    let mad_libs_dir = std::path::Path::new(MAD_LIBS_DIR);

    match check_mad_lib_dir(&mad_libs_dir) {
        Ok(_) => (),

        Err(e) => {
            println!("{e}");
            return;
        }
    }

    //Getting an iterator of all the files in mad_libs/
    let mad_libs_iter: std::fs::ReadDir;

    match std::fs::read_dir(mad_libs_dir){
        Ok(entries) => mad_libs_iter = entries,

        Err(e) => {
            println!("Could not read {MAD_LIBS_DIR}: {e}");
            return;
        }
    }

    //Iterating over the iterator and saving each directory entry into a Vec
    let mut mad_libs :Vec<std::fs::DirEntry> = vec![];
    for entry in mad_libs_iter {
        match entry {
            Ok(entry) => {
                mad_libs.push(entry);
            }
            Err(e) => {
                println!("Error: {e}");
                return;
            }
        }
    }

    play_mad_lib(&mad_libs);

    let mut user_input: String = std::string::String::new();
    loop {
        println!("Would you like to play again? [Y/N]");

        std::io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
        
        match user_input.trim().to_lowercase().as_str() {
            
            "y" => play_mad_lib(&mad_libs),

            "n" => break,

            _ => {
                println!("Unknown input")
            }
        }
    }

    println!("Thank you for playing my Mad Lib game!")

}

fn select_mad_lib(mad_libs_files: &Vec<std::fs::DirEntry>) -> usize {
    let mut user_input_str: String = String::new();
    let user_input_usize: usize;

    //Prints out the number and name of each file for the user to select
    for (x, file) in mad_libs_files.iter().enumerate() {
        println!("{}. {:?}", x+1, file.file_name())
    }
    
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
                    if num <= mad_libs_files.len() && num > 0 {
                        user_input_usize = num - 1;
                        break;
                    }
                    else {
                        println!("Number is either too long or too big! Please make sure to enter 
                        a number greater than 0 but less than {}", mad_libs_files.len());
                        continue;
                    }
                }
                Err(_) => continue,
            };
        };
        return user_input_usize;
}

fn check_mad_lib_dir(mad_libs_dir: &std::path::Path) -> Result<(), std::io::Error>{
    if !mad_libs_dir.exists() {
            println!("No {MAD_LIBS_DIR} directory found...");
            println!("Making {MAD_LIBS_DIR} directory...");

            match std::fs::create_dir(mad_libs_dir){
                Ok(_) => println!("Successful made {MAD_LIBS_DIR}"),

                Err(e) => {
                    println!("Failed to create {MAD_LIBS_DIR}: {e}");
                    return Err(e);
                }
            }
        }

        if !mad_libs_dir.is_dir(){
            println!("{MAD_LIBS_DIR} found, but it appears not to be a directory, 
                please delete or rename it so the program can make the correct {MAD_LIBS_DIR}");
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Path exists but is not a directory"));
        }

        Ok(())
}

fn play_mad_lib(mad_libs: &Vec<std::fs::DirEntry>) {
    
    let user_input_usize: usize = select_mad_lib(&mad_libs);

    let mut mad_lib: MadLib = MadLib { 
        mad_lib: String::new(), 
        fill_in_words: vec![], 
        filled_in_words: vec![]
    };

    match std::fs::read_to_string(mad_libs[user_input_usize].path()){
        Ok(s) => mad_lib.mad_lib = s,

        Err(_) => {
            println!("Mad Lib File could not be read!");
            return
        }
    }
    
    match mad_lib.parse_mad_lib() {
        Ok(_) => (),

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

    match mad_lib.fill_in_words() {

        Ok(_) => (),

        Err(e) => {
            println!("Error getting fill in words: {e}");
            return;
        }

    }

    println!("Filled in words are: {:?}", mad_lib.filled_in_words.clone());

    mad_lib.display_mad_lib();
}