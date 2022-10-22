
use crate::menue;
use crate::helpers;
use crate::distribution_menue;

use std::fs;
use std::process::Command;

pub fn set_up(){
    
    menue::print_title("SYNCHRONICE");
    menue::print_menue(&[

        menue::build_menue_point("api", "create API of MD-Files"),
        menue::build_menue_point("wiki", "create Wiki of MD-Files"),
        menue::build_menue_point("site", "inserts html of MD into website"),
        menue::build_menue_point("fetd", "reset current Workspaces to dev"),
        menue::build_menue_point("inch", "get inserts for changelog"),
        menue::build_menue_point("----", "-------------------------------"),
        menue::build_menue_point("esc ", "go back to mainmenue"),

    ]);

    check_input()
}

fn check_input(){
    match helpers::input_validator::sanitice().as_str() {

        "api" => api()   ,
        "wiki" => wiki()   ,
        "site" => site(),
        "inch" => get_changelog_inserts(),
        "fetd" => fetch(),
        "esc" => menue::print_main_menue(), 
        _     =>  set_up(),
    }
}

pub fn fetch(){
    helpers::bash_commands::fetch_repo(&format!("{}\\{}\\ChangeLog.md",helpers::filepaths::get_root(),"Src"));
    helpers::bash_commands::fetch_repo(&format!("{}\\{}\\ChangeLog.md",helpers::filepaths::get_root(),"IDE"));
    helpers::bash_commands::fetch_repo(&format!("{}\\{}\\ChangeLog.md",helpers::filepaths::get_root(),"CLI"));
    helpers::bash_commands::fetch_repo(&format!("{}\\{}\\ChangeLog.md",helpers::filepaths::get_root(),"Docs"));
}




fn api(){
    helpers::bash_commands::build_API();        
}

pub fn wiki(){
    helpers::bash_commands::build_Wiki();        
}


pub fn site(){
    let root = helpers::filepaths::get_root();
    
    let api  = format!("{}Docs\\API\\site",root);
    let wiki = format!("{}Docs\\Wiki\\site",root);

    let site_api = format!("{}Website\\mkDocs\\Docs",root);
    let site_wiki = format!("{}Website\\mkDocs\\Wiki",root);

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite=true;
    
    fs_extra::copy_items(&[&api],&site_api,&options);
    fs_extra::copy_items(&[&wiki],&site_wiki,&options);

    //Command::new("rm").arg("-r").arg("-f").arg(wiki).status();
    //Command::new("rm").arg("-r").arg("-f").arg(api).status();
}





pub fn get_changelog_inserts(){
    print!("{}",_get_src_insert())
}

fn _get_src_insert()-> std::string::String {
    let path_change_log = format!("{}\\Src\\ChangeLog.md",helpers::filepaths::get_root());
    let mut contents = fs::read_to_string(&path_change_log).expect("Something went wrong reading the file");

    let mut isFirstEntryInFile = true;
    let mut commit_message : String;
    commit_message= "".to_owned();

    for line in contents.split("\n") {
        if line.contains("# Version") && !isFirstEntryInFile{ break }
        if line.contains("# Version") && isFirstEntryInFile{ isFirstEntryInFile = false; }
        if !isFirstEntryInFile{
            
            if line.contains("https://img.shields.io/badge/GAME-ADDED-brightgreen?style=for-the-badge"){
                commit_message = format!("{}\n **ADDED**", commit_message);
            }
            else if line.contains("https://img.shields.io/badge/GAME-CHANGED-yellow?style=for-the-badge"){
                commit_message = format!("{}\n **CHANGED**", commit_message);
            }
            else if line.contains("https://img.shields.io/badge/GAME-REMOVED-red?style=for-the-badge"){
                commit_message = format!("{}\n **REMOVED**", commit_message);
            }
            else if line.contains("CMS"){
                return commit_message
            }
            else{
                commit_message = format!("{}\n> {}", commit_message, line);
            }
        }
    }

    return commit_message
}


