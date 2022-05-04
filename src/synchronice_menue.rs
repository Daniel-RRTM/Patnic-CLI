
use crate::menue;
use crate::helpers;


pub fn set_up(){
    
    menue::print_title("SYNCHRONICE");
    menue::print_menue(&[

        menue::build_menue_point("api", "create API of MD-Files"),
        menue::build_menue_point("wiki", "create Wiki of MD-Files")
    ]);

    check_input()
}

fn check_input(){
    match helpers::input_validator::sanitice().as_str() {

        "api" => api()   ,
        "wiki" => wiki()   ,
         _     => print!("still WIP"),
    }
}






fn api(){
    helpers::bash_commands::build_API();        
}

pub fn wiki(){
    helpers::bash_commands::build_Wiki();        
}