use walkdir::{DirEntry,WalkDir};
use std::ffi::OsString;
use std::path::Path;
use std::process::Command;
use std::fs;

#[derive(Debug)]
struct Folder{
    path: String,
    relative_path: String,
}


#[derive(Debug)]
struct File{
    path: String,
    name: String,
    extension: String,
    fullpath: String,
    relative_path: String,
}
impl File{
    fn new(entry: DirEntry, top_dir: &str) -> File{

        let path: String= entry.path().to_owned().into_os_string().into_string().unwrap_or("".to_string());
        let fullpath: String = path.clone();
        let relative_path = path.clone().strip_prefix(top_dir).unwrap().into();
       
        let path: Vec<&str> = path.split("/").collect();
        let filename: String = path[path.len()-1].to_string();
        let filename: Vec<&str> = filename.split(".").collect();
        let filename_count = filename.len();
        let name = filename[..filename_count-1].join(".").to_string();
        let extension = filename[filename_count-1].to_string();
        let path = path[..path.len()-1].join("/").to_string() + "/";
        File{
            path,
            name,
            extension,
            fullpath,
            relative_path,
        }
        
    }    

}


fn mkdir(dir: String){
    fs::create_dir_all(dir);
}
fn main(){
    let args: Vec<_> = std::env::args().collect();    
    if args.len() != 4 {
        panic!("there should only be 3 arguments")
    }
    let bitrate = &args[1];
    let input = &args[2];
    let output = &args[3];
    
    let files = scan(input);
    transcode_vec(files,output)
}

fn is_hidden(entry: &DirEntry
) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn scan(dir: &str) -> Vec<File> {
    let top_dir = "../../Music";
    println!("Hello, world!");
    let files = WalkDir::new(top_dir).follow_links(true).into_iter().filter_entry(|e| !is_hidden(e)).into_iter().filter_map(|e| e.ok()).into_iter().map(|entry| File::new(entry,top_dir));
    let dirs: Vec<_> = WalkDir::new(top_dir).follow_links(true).into_iter().filter_entry(|e| e.file_type().is_dir()).filter_map(|e| Some(String::from("music-transcoded") + e.ok().unwrap().path().to_owned().into_os_string().into_string().unwrap_or("".to_string()).strip_prefix(top_dir).unwrap())).into_iter().collect();    
    
   
    for i in dirs.clone(){
        mkdir(i);
    }

    println!("{:#?}",dirs);    
    files.collect()
    
}


fn transcode_vec(files: Vec<File>, out: &str) {
    for i in files{
        transcode(i,out);
    }
    
}
 
fn transcode(inp: File, out: &str){
    let outfile = format!("{}{}",&out.to_string(),inp.relative_path);
    match inp.extension.clone().as_str() {
    "flac" | "opus"| "mp3" | "aac" | "m4a" => { let command = Command::new("ffmpeg").args(["-i",format!(r#"{}"# ,inp.fullpath).as_str()]).args(["-b:a", "192k", "-c:a", "libopus"]).arg(format!(r#"{}"#,&outfile).as_str()).output();
    println!("{:#?}",command);}
        _ => {fs::copy(&inp.fullpath, &outfile);}
        }
    }
