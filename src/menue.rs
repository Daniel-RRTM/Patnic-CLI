

    use crate::helpers;


    pub struct Menue_Point{
        pub to_string : String,
        pub input     : String,
    }


    pub fn build_menue_point(to_string_para:&str, input_para:&str) -> Menue_Point {
        return Menue_Point {
            to_string : to_string_para.to_string(),
            input     : input_para.to_string(),
        }
    } 

    


    pub fn print_title(title:&str){   
        let to_print = format!(" --- {} MENUE --- \n\n",title);
        helpers::text_formater::print_white(&to_print);
    }


    pub fn print_menue(menue_points:&[Menue_Point]){
        for current_point in menue_points.iter() {
            helpers::text_formater::print_cyan("      ");
            helpers::text_formater::print_cyan(&current_point.to_string);
            helpers::text_formater::print_cyan("  >   ");
            helpers::text_formater::print_white(&current_point.input);
            helpers::text_formater::print_cyan("\n");
        }
        helpers::text_formater::print_separator();
        helpers::text_formater::print_cyan("\n");
    }

    
    pub fn print_chapter(chapter_name:&str){
        let middle_name = format!("   {}      \r\n",chapter_name);
    
        helpers::text_formater::print_cyan("\r\n\r\n\r\n");
        helpers::text_formater::print_cyan(" =============================    \r\n");
        helpers::text_formater::print_cyan(       &middle_name                );
        helpers::text_formater::print_cyan(" =============================    \r\n");
        helpers::text_formater::print_cyan("\r\n\r\n\r\n");
    }







    pub fn print_very_cool_ascii(){
        // look away, nothing to see !!!
        let  firstRow          = "                ____ ";        
        let  secondRow         = "               / __ \\";     
        let  thirdRow          = "/ /_/ /";       
        let  fourthRow         = "/ ____/";       
        let  fithRow           = "/ /    ";     
        let  sixthRow          = "/_/                  ";  

        let  firstRowTwo       ="  ______ ";
        let  secondRowTwo      =" / ____ \\ ";
        let  thirdRowTwo       ="/ / __ `/";
        let  forthRowTwo       ="/ / /_/ /";
        let  fithRowTwo        =" \\ \\____/_";
        let  sixthRowTwo       =" ";

        let  firstRowThree     = "    __           __                                               \r\n";        
        let  secondRowThree    = "__/ /_______   (_)._____ _____ ____   ____  _____ ___       \r\n              ";
        let  thirdRowThree     = "/_  ____/ __ \\ / // ____// ___// __ \\ / __ \\ / __ `__ \\     \r\n             ";
        let  fourthRowThree    = " /  /_  / / / // // |___ / /   / /_/ // /_/ // / / / / /        \r\n            ";
        let  fithRowThree      = "|_____|/_/";
        
        let  runr              = "_";
        let  laaaast           = "/_//_/";
        let  judbef            = "_";
        let pleaselast            = "\\____//_/";
        let  postpre           = "____";
        let  postpost          = "\\____/";
        let  twi               = "_";
        let  tes               = "\\____//_/";
        let  dwa               = "/_/";
        let  test              = "_";
        let  testete           = "/_/";

        let  sixthRowThree     = "      \\_____________________________________________________________________________\\           \r\n          ";
        let  postfithRowThree  = "______________\r\n              ";


        helpers::text_formater::print_white(firstRow );
        helpers::text_formater::print_cyan(firstRowTwo ); //@
        helpers::text_formater::print_white(firstRowThree );
        helpers::text_formater::print_white(secondRow );//@
        helpers::text_formater::print_cyan(secondRowTwo);
        helpers::text_formater::print_white(secondRowThree );
        helpers::text_formater::print_white(thirdRow );
        helpers::text_formater::print_cyan(thirdRowTwo );//@
        helpers::text_formater::print_white(thirdRowThree);
        helpers::text_formater::print_white(fourthRow);
        helpers::text_formater::print_cyan(forthRowTwo );
        helpers::text_formater::print_white(fourthRowThree );
        helpers::text_formater::print_white(fithRow );
        helpers::text_formater::print_cyan(fithRowTwo );//@
        helpers::text_formater::print_white(fithRowThree ,);
        helpers::text_formater::print_cyan(runr );
        helpers::text_formater::print_white(laaaast );
        helpers::text_formater::print_cyan(judbef );
        helpers::text_formater::print_white(pleaselast );  
        helpers::text_formater::print_cyan(postpre ); 
        helpers::text_formater::print_white(postpost ,);
        helpers::text_formater::print_cyan(twi ,);
        helpers::text_formater::print_white(tes ,);
        helpers::text_formater::print_cyan(test ,);
        helpers::text_formater::print_white(testete ,);
        helpers::text_formater::print_cyan(test ,);
        helpers::text_formater::print_white(dwa ,);
        helpers::text_formater::print_cyan(postfithRowThree ,);
        helpers::text_formater::print_cyan(sixthRowTwo ,);
        helpers::text_formater::print_cyan(sixthRowThree ,);
        helpers::text_formater::print_cyan("\n");
        helpers::text_formater::print_cyan("\n");
    }


