#![allow(non_snake_case)]


use std::fs::File;
use std::io::Write;
use std::io;
use std::fs::metadata;
use std::fs::OpenOptions;
use std::fs::{self, DirEntry};
use std::ffi::OsStr;
use rand::Rng;
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use std::error::Error;


#[derive(Debug, Deserialize, Serialize,Clone)]
struct Files{
     Files: Vec<FileObj>
}

#[derive(Debug, Deserialize, Serialize,Clone)]
struct FileObj{
    Original: String,
    Renamed: String
}

fn main() {
    
    let mut input = String::new(); 
    loop{
        println!("Enter a directory");
        io::stdin().read_line(&mut input).expect("AHHHHHHHHHHHHHHH"); //gets directory
        let test = Path::new(&input.trim()).exists(); //checks if directory exists
        
        if test{
            break; //if directory exists breaks loop
        }
        input = String::new(); //resets input string to blank
    }
    let directory = input.trim().to_string(); //trims new line character
    let logPath = format!("{}/output.json",directory); //path to output file
    let logCheck = Path::new(&logPath).exists(); //checks if output exists
    let files = Files{Files: vec![FileObj{Original: "0".to_string(), Renamed: "0".to_string()}]}; //sets up an dummy object to store json
    if !logCheck{
        let mut _log = File::create(&logPath); //creates output file
        let mut file = OpenOptions::new().write(true).open(format!("{}\\output.json", directory)).expect("cannot open file"); //opens output file
        writeln!(file, "{}", serde_json::to_string(&files).unwrap()).expect("failed write");   //writes object to file
    }

        let mut file = OpenOptions::new().write(true).open(format!("{}\\output.json", directory)).expect("cannot open file"); //opens output file

    

    let filesResult    = read_name_from_file(logPath); //reads output file for existing json
    let files: &mut Files = &mut filesResult.unwrap();

    if files.Files[0].Original == "0"{ //if dummy object is in json pops it out of the vector
        files.Files.pop();
    }
    loop{
        println!("1 to randomize, 2 to revert");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("AHHHHHHHHHHHHHHH");

        if input.trim() == "1"{
            let paths = fs::read_dir(directory.clone()).unwrap(); //reads directory for all file paths
            for path in paths {
                    if !metadata(path.as_ref().unwrap().path()).unwrap().is_dir(){ //if file path is not a directory
                        renameFile(path.unwrap(), directory.clone(), files);
                    }
                    
                
            }

            writeln!(file, "{}", serde_json::to_string(&files).unwrap()).expect("failed write"); //writes file changes to output
            break;
        }

        if input.trim() == "2"{
            revertFiles(files,format!("{}\\output.json", directory));
            break;
        }

    }
}

fn read_name_from_file<P: AsRef<Path>>(path: P) -> Result<Files, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

fn revertFiles(files : &mut Files, logPath : String){
    
    let mut reverseFiles: Vec<FileObj> = Vec::new(); //creates a vector to store the reverse of the vector of files
    
    for i in 0..files.Files.len(){
        reverseFiles.push(files.Files[files.Files.len()-i-1].clone()); 
    }
    
   // for i in 0..reverseFiles.len(){
   //     let _ = fs::rename(&reverseFiles[i].Renamed, &reverseFiles[i].Original); //renames the files in reverse order
   // }

   for item in &reverseFiles{
        let _ = fs::rename(&item.Renamed, &item.Original);
   }
    
    let _ = fs::remove_file(logPath); //removes the output file

}

fn renameFile(filePath: DirEntry, directory: String, files : &mut Files){
    let mut renamePath: bool = true;
    let originalFile = filePath.path().display().to_string(); //stores the path of the current file

    let extensionCheck = Path::new(&originalFile).extension().and_then(OsStr::to_str); //checks the extension of the file
    let extension: String = if extensionCheck.is_none(){
        String::new() //if file has no extension
    }else{
        format!(".{}",extensionCheck.unwrap()) //if file has extension 
    };

    while renamePath{
        let i = rand::thread_rng().gen_range(1..=1000000);
        let newFile = format!("{directory}/{i}{}",&extension); //generates new file path
        renamePath = Path::new(&newFile).exists(); //checks if new file path exists

        if !renamePath && originalFile != format!("{}\\output.json", directory){ //if new file path doesnt exist and isnt the output file
            let _ =fs::rename(&originalFile, &newFile); //renames file
            let testOBJ =FileObj{ //creats object of original and renamed path
                Original: originalFile.to_owned(), 
                Renamed: newFile.to_owned()
            };
            
            files.Files.push(testOBJ);   //adds object to vector     
            

        }   
    }
}