use std::path::Path;
use std::fs;
use std::fs::Metadata;

fn main() {





    let permision_switch = true;
    let show_hidden = true;

    // if pfad == file || path == dir || path == symlink
    let path = Path::new("/home/shu/");
    let metadata = path.metadata().expect("metadata call failed");
    if metadata.is_symlink() || path.exists() == false{
        panic!("must be a directory")
    }
    else {
        println!("{:?}", metadata.file_type());
    }
    /*
        let path_name = path.file_name();
        println!("{}", path_name.unwrap().display() );

    let path_string = path.display().to_string();
    let path_length = path_string.len();


     */



    let mut hidden_files :Vec<String> = vec![];
    let mut hidden_folders:Vec<String> = vec![];
    let mut files :Vec<String> = vec![];
    let mut folders :Vec<String> = vec![];

    /*// todo checken welche brechtigungen der user hat
   if path.has_root() {
        println!("keine berechtigung")
    }

     */
    let file_stem = path.file_stem();
    println!("{}", file_stem.unwrap().display());

    // read folder inhalt
    for entry in path.read_dir().expect("read_dir call failed") {

        if let Ok(entry) = entry {
            let entry_a = entry.path();
            let entry_b = entry_a.file_name().unwrap().display().to_string();
            let first_char = entry_b.chars().next().unwrap();
            let entry_c = entry_b.clone();
            let hidden_char :bool;
            if first_char == '.' {
                hidden_char = true;
            }
            else {
                hidden_char = false;
            }



            let path_meta = entry.metadata().expect("metadata call failed");
            if path_meta.is_dir() == true && hidden_char == true && show_hidden == true {
                hidden_folders.push(entry_c);
            }
            else if path_meta.is_dir() == true &&  hidden_char == false {
                folders.push(entry_c);
            }
            else if path_meta.is_dir() == false &&  hidden_char == false {
                files.push(entry_c);
            }
            else if path_meta.is_dir() == false &&  hidden_char == true && show_hidden == true {
                hidden_files.push(entry_c);
            }


            /*
            if permision_switch == true {
                permission(entry_b);
            }
            else {
                println!("UwU")
            }

             */
        }
    }
    hidden_folders.sort();
    hidden_files.sort();
    folders.sort();
    files.sort();

    for hidden_folder in &hidden_folders {
        println!("{}", hidden_folder)
    }
    for folders in &folders {
        println!("{}", folders)
    }
    for hidden_files in &hidden_files {
        println!("{}", hidden_files)
    }
    for files in &files {
        println!("{}", files)
    }


    /*
     .:3
     .:3.txt
     OwO
     UwU.txt
     */



    fn permission(entry_b :String) -> std::io::Result<Metadata> {
        let permission = fs::metadata(entry_b);
        permission
    }

}





/*

1. alles klein schreiben
2. orte merken in vec
3. sort
4. wörter wieder groß machen

 */













