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
        io::stdin().read_line(&mut input).expect("AHHHHHHHHHHHHHHH");
        let test = Path::new(&input.trim()).exists();
        println!("{}",input.trim());
        if test{
            break;
        }
        input = String::new();
    }
    let directory = format!("{}",input.trim());
    let logPath = format!("{}/output.json",directory);
    let logCheck = Path::new(&logPath).exists();
    let files = Files{Files: vec![FileObj{Original: "0".to_string(), Renamed: "0".to_string()}]};
    if !logCheck{
        let mut _log = File::create(&logPath);
        let mut file = OpenOptions::new().write(true).open(format!("{}\\output.json", directory)).expect("cannot open file");
        writeln!(file, "{}", serde_json::to_string(&files).unwrap()).expect("failed write");   
    }

        let mut file = OpenOptions::new().write(true).open(format!("{}\\output.json", directory)).expect("cannot open file");

    

    let filesResult    = read_name_from_file(logPath);
    let files: &mut Files = &mut filesResult.unwrap();

    if files.Files[0].Original == "0"{
        files.Files.pop();
    }
    loop{
        println!("1 to randomize, 2 to revert");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("AHHHHHHHHHHHHHHH");

        if input.trim() == "1"{
            let paths = fs::read_dir(directory.clone()).unwrap();
            for path in paths {
                    if metadata(path.as_ref().unwrap().path()).unwrap().is_dir() == false{
                        renameFile(path.unwrap(), directory.clone(), files);
                    }
                    
                
            }

            writeln!(file, "{}", serde_json::to_string(&files).unwrap()).expect("failed write");
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
    
    let mut reverseFiles: Vec<FileObj> = Vec::new();
    
    for i in 0..files.Files.len(){
        reverseFiles.push(files.Files[files.Files.len()-i-1].clone());
    }
    
    for i in 0..reverseFiles.len(){
        let _ = fs::rename(&reverseFiles[i].Renamed, &reverseFiles[i].Original);
    }
    
    let _ = fs::remove_file(logPath);

}

fn renameFile(filePath: DirEntry, directory: String, files : &mut Files){
    let mut renamePath: bool = true;
    let originalFile = filePath.path().display().to_string();

    let extensionCheck = Path::new(&originalFile).extension().and_then(OsStr::to_str);
    let extension;

    if extensionCheck == None{
        extension = format!("");
    }else{
        extension = format!(".{}",extensionCheck.unwrap());
    }
    while renamePath{
        let i = rand::thread_rng().gen_range(1..=1000000);
        let newFile = format!("{directory}/{i}{}",&extension);
        renamePath = Path::new(&newFile).exists();

        if renamePath == false && originalFile != format!("{}\\output.json", directory){
            let _ =fs::rename(&originalFile, &newFile);
            let testOBJ =FileObj{
                Original: originalFile.to_owned(),
                Renamed: newFile.to_owned()
            };
            
            files.Files.push(testOBJ);        
            

        }   
    }
}