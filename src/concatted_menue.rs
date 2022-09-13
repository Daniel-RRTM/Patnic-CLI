use crate:: backup_menue;
use crate::menue;
use crate::helpers;
use crate::distribution_menue;
use crate::synchronice_menue;


pub fn set_up(){
    
    menue::print_title("CONCAT");
    menue::print_menue(&[
        menue::build_menue_point("back", "cache the whole project                         [dele,crea,load]"),
        menue::build_menue_point("docs", "creates Statistics and MD of current Entities   [dele,crea,calc,wiki,api,site]"),
        menue::build_menue_point("docs", "creates Statistics and MD of current Entities   [dele,crea,calc,wiki,api,site]"),
        menue::build_menue_point("refr", "Caches current Workspace and fetchs dev         [dele,crea,fetd]"),
        menue::build_menue_point("pusd", "pushes current Workspaces to dev                [ched,push]"),
        menue::build_menue_point("----", "------------------------------------------------------------------------------"),
        menue::build_menue_point("esc ", "go back to mainmenue"),
    ]);

    check_input()
}

fn check_input(){
    match helpers::input_validator::sanitice().as_str() {
        "back" => back_up_project(),
        "docs" => create_docs(),
        "refr" => cache_and_fetch(),
        "pusd" => distribution_menue::push(),
        "esc" => menue::print_main_menue(), 
        _     =>  set_up(),
    }
}

fn cache_and_fetch(){
    backup_menue::create();
    synchronice_menue::fetch();
}

fn back_up_project(){
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

