use std::collections::HashMap;

mod helpers;
mod menue;

mod backup_menue;
mod concatted_menue;
mod distribution_menue;
mod synchronice_menue;


fn main() {
    helpers::cli_formater::set_cmd_box_format();
    menue::print_very_cool_ascii();

    
    menue::print_menue(&[
        menue::build_menue_point("back", "cache the whole project"),
        menue::build_menue_point("dstr", "distribute configs etc."),
        menue::build_menue_point("sync", "synchronice Docs and DioJSONes"),
        menue::build_menue_point("coca", "concatted commands,recommended")
    ]);
    

    change_to_submenue()

}



fn change_to_submenue(){
    let mut input = helpers::input_validator::sanitice();

    match input.as_str() {
        "back" => backup_menue::set_up(),
        "dstr" => distribution_menue::set_up(),
        "coca" => concatted_menue::set_up(),
        "sync" => synchronice_menue::set_up(),
        "esc"  => print!(""),
         _     => print!("still WIP"),
    }
}