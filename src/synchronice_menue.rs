
use crate::menue;
use crate::helpers;
use std::process::Command;

pub fn set_up(){
    
    menue::print_title("SYNCHRONICE");
    menue::print_menue(&[

        menue::build_menue_point("api", "create API of MD-Files"),
        menue::build_menue_point("wiki", "create Wiki of MD-Files"),
        menue::build_menue_point("site", "inserts html of MD into website")

    ]);

    check_input()
}

fn check_input(){
    match helpers::input_validator::sanitice().as_str() {

        "api" => api()   ,
        "wiki" => wiki()   ,
        "site" => site(),
         _     => print!("still WIP"),
    }
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

