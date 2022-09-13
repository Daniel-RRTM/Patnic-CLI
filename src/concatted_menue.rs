use crate:: backup_menue;
use crate::menue;
use crate::helpers;
use crate::distribution_menue;
use crate::synchronice_menue;


pub fn set_up(){
    
    menue::print_title("CONCAT");
    menue::print_menue(&[
        menue::build_menue_point("back", "cache the whole project"),
        menue::build_menue_point("docs", "creates Statistics and MD of current Entities"),
    ]);

    check_input()
}

fn check_input(){
    match helpers::input_validator::sanitice().as_str() {
        "back" => back_up_project(),
        "docs" => create_docs(),
  
         _     => print!("still WIP"),
    }
}



fn back_up_project(){
    backup_menue::delete();
    backup_menue::create();
    backup_menue::load();
}


fn create_docs(){
    backup_menue::delete();
    backup_menue::create();

    distribution_menue::calculate();

    menue::print_chapter("building Markdowns...");
    helpers::bash_commands::build_Wiki();
    helpers::bash_commands::build_API();
    
    menue::print_chapter("Distributing HTMLs...");
    synchronice_menue::site();
}

