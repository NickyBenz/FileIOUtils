use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write, Read};
///File Writer function 
///Used to write into files 
///# Examples:
/// writeToFile("text.txt", "Hi")

pub fn writeToFile(path: &str, text: String){
    let  file = OpenOptions::new()
        .write(true).open(path);
    match file{
        Ok(mut f) => {
            f.write_all(text.as_str().as_bytes()).expect("Unable to write to file, please check permissions"); 
        },
        Err(_) => println!("Please enter a valid file path, file not found or permissions invalid")
    }

}
///File Reader function 
///Used to read from files 
/// #Examples:
/// let message = readFromFile("text.txt").expect("Error reading from file")

pub  fn readFromFile(path: String)-> Result<String, &'static str> {
    let file = File::open(path.as_str());
    match file{
        Ok( mut f) => {
          let mut text= String::new();
          f.read_to_string(&mut text).expect("Unable to read from file, please check permissions");
          Ok(text)
        },
        Err(_) => Err("Please enter a valid file path, file not found or permissions invalid")
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn testWriteToFile(){
        let _ = std::fs::remove_file("text.txt");

        let mut write_file = File::create("text.txt").expect("Cannot create file here");

        writeToFile("text.txt", "Hi".to_string());

        let mut recieved_string = String::new();

        let mut read_file = File::open("text.txt").expect("Unable to read from file");

        read_file.read_to_string(&mut recieved_string).expect("Cannot read from file");

        assert_eq!(recieved_string.as_bytes(), "Hi".as_bytes())
    }
    #[test]
    pub fn readToFile(){
        let _ = std::fs::remove_file("text.txt");
          let mut file = OpenOptions::new()
        .write(true).create(true).open("text.txt").expect("Cannot create file");

        file.write_all(b"Hi");

        let message = readFromFile("text.txt".to_string()).expect("Failed to read from file");
        assert_eq!("Hi", message.as_str());

    }


}   
