

pub mod text_formater{
    extern crate crossterm;
    use crossterm::style::*;

    pub fn print_cyan(text:&str){    print!( "{}", text.bold().on( Color::Rgb { r:0,g:0,b:20 }   ).cyan()    ); }
    pub fn print_white(text:&str){   print!( "{}", text.bold().on( Color::Rgb { r:0,g:0,b:20 }   ).grey()   ); }
    pub fn print_separator(){        print!( "{}", "\n".bold().on( Color::Rgb { r:0,g:0,b:100 }  ).white()  ); }
}

 

pub mod bash_commands{
    use crate::helpers::filepaths::get_root;
    use std::process::Command;
    use std::env;
    use std::path::Path;
    use std::fs;
    
    pub fn execute_cmd(cmd:&str, arg:&str){   Command::new(cmd).arg(arg).status();   }

    pub fn start_patnic_with_calc(){   Command::new("sh").arg("startPatnicRoom.sh").status().expect(" ");   }
    
    pub fn build_Wiki(){   
        let filepath_root = format!("{}CLI\\Src",get_root());
        let filepath_to_docs = format!("{}Docs\\Wiki",get_root());
        
        env::set_current_dir(filepath_to_docs);
        Command::new("mkdocs").arg("build").status().expect(" ");
        env::set_current_dir(filepath_root);
    }
    pub fn build_API(){
        let filepath_root = format!("{}CLI\\Src",get_root());
        let filepath_to_docs = format!("{}Docs\\API",get_root());

        env::set_current_dir(filepath_to_docs);
        Command::new("mkdocs").arg("build").status().expect(" ");
        env::set_current_dir(filepath_root);
    }
    
    pub fn remove_cached_project(){
        let to_delete     = format! ("{}Cache",get_root());
        Command::new("rm").arg("-r").arg("-f").arg("../Cache").status();   
        Command::new("rm").arg("-r").arg("-f").arg("../../Cache").status();   
        Command::new("cd").arg("..").status(); 
    }

    pub fn push_repo(filepath:&str,commit_message:&str){
        env::set_current_dir(filepath);
       // Command::new("git").arg("fetch").status().expect(" ");  
       
       let test = Command::new("git").arg("add").arg(".").status().expect(" ");   
        print!("ADD! \n\n{}",test.to_string());

        let test = Command::new("git").arg("commit").arg(format!("-m \"{}\"",commit_message).as_str()).status().expect(" ");   
       // print!("COMMIT! \n\n{}",test.to_string());
           
        let test = Command::new("git").arg("push").status().expect(" ");   
        //print!("PUSH! \n\n{}",test.to_string())
    }

    pub fn fetch_repo(filepath:&str){
        env::set_current_dir(filepath);
        let test = Command::new("git").arg("fetch").status().expect(" ");   
        print!("{}",test.to_string())
    }

}



pub mod cli_formater{
    use crossterm::terminal::*;
    use std::io::{stdout, Write};
    use crossterm::*;
    use crossterm::style::*;

    
    pub fn set_cmd_box_format()-> Result<()>{
        for i in 0..50 {   println!("")   }  
        
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            SetBackgroundColor(Color::Rgb { r:0,g:0,b:20 }),
            SetSize(110, 30),
           ScrollUp(50)
        )?;Ok(())
    }

}



pub mod input_validator{
    use std::io;
    use crate::helpers::text_formater;

  
    pub fn sanitice()-> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
    
        if let Some('\n')=input.chars().next_back() {input.pop();}
        if let Some('\r')=input.chars().next_back() {input.pop();}
        
        return input
    }  
}



pub mod filepaths{
    use std::env;
    

    pub fn get_root() -> String{
        let path = env::current_dir().unwrap().to_str().unwrap().to_string();
        let splitted_root: Vec<&str> = path.split("CLI").collect();
        let to_return = splitted_root[0];
        return to_return.to_string();
    }

    pub fn get_parent() -> String{
        let path = env::current_dir().unwrap().to_str().unwrap().to_string();
        let splitted_root: Vec<&str> = path.split("PatnicRoom").collect();
        let to_return = splitted_root[0];
        return to_return.to_string();
    }


    pub fn get_game_init() -> String{   return format!("{}Src\\tools\\gameinit.txt", get_root());   }
    pub fn get_godot_exe() -> String{   return format!("{}CLI\\gameinit.txt",get_root());   }
    pub fn get_docs()      -> String{   return format!("{}docs",get_root());   }


}


