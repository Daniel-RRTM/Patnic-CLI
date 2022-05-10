
    use crate::menue;
    use crate::helpers;
    use std::process::Command;
    use std::fs;
    use std::env;


    pub fn set_up(){

        menue::print_title("BACKUP");
        menue::print_menue(&[
            menue::build_menue_point("crea", "create new cache"),
            menue::build_menue_point("dele", "delete current cache"),
            menue::build_menue_point("load", "load last cache")
        ]);

        check_input()
    }

    fn check_input(){
        match helpers::input_validator::sanitice().as_str() {
            "crea" => create(),
            "dele" => delete(),
            "load" => load(),
            _     => print!("still WIP"),
        }
    }




    pub fn create(){
        delete();

        menue::print_chapter("Set Back-up");
        print_folder_copy(&["config","DevCLI","docs","GameData","GameSrc","Media","tools","VSCode SetUp","Website","Misc."]);

        let root          = helpers::filepaths::get_root();
        let to_insert     = format!("{}",root);

        let mut options   = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        
        print!("{}",root);

        fs_extra::copy_items(&[format!("{}",root)],&to_insert,&options);
        fs::rename(format!("{}Workspace",root),format!("{}Cache",root));
    }






    pub fn delete(){
        menue::print_chapter("remove Back-up");
        print_folder_reset(&["config","DevCLI","docs","GameData","GameSrc","Media","tools","VSCode SetUp","Website","Misc."]);
        helpers::bash_commands::remove_cached_project();
    }



    pub fn load(){
        menue::print_chapter("Load Back-up");
        print_folder_load(&["config","DevCLI","docs","GameData","GameSrc","Media","tools","VSCode SetUp","Website","Misc."]);
        
        let parent  = helpers::filepaths::get_parent();
        let root    = helpers::filepaths::get_root();
        let back_up = format!("{}Cache",root);
        
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite=true;
        
       
        fs_extra::copy_items(&[&back_up],&parent,&options);
        fs_extra::copy_items(&[&back_up],&root,&options);
        Command::new("cp").arg("-r").arg("../../Cache/*").arg("../../PatnicRoom").status();

        Command::new("rm").arg("-r").arg("-f").arg("../../Cache").status();
    }







    fn print_folder_copy(workspace:&[&str]){
        for current_workspace in workspace.iter(){
            helpers::text_formater::print_white("copy ");
            helpers::text_formater::print_cyan(current_workspace);
            helpers::text_formater::print_white(" into Cache\n");
        }
    }

    fn print_folder_load(workspace:&[&str]){
        for current_workspace in workspace.iter(){
            helpers::text_formater::print_white("load ");
            helpers::text_formater::print_cyan(current_workspace);
            helpers::text_formater::print_white(" of ./Cache\n");
        }
    }

    fn print_folder_reset(workspace:&[&str]){
        for current_workspace in workspace.iter(){
            helpers::text_formater::print_white("renove ");
            helpers::text_formater::print_cyan(current_workspace);
            helpers::text_formater::print_white(" from ./Cache\n");
        }
    }