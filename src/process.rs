use crate::default;
use crate::types::Opt;
use std::fmt;


pub fn process(out_str: &String, opt: &Opt) -> String {


    println!("{}",out_str);

    let mut fmt_str: String = "".to_string();

    if opt.latex {

        fmt_str = fmt_str+default::ENTER_BLOCK_MACRO;
    
        if opt.colorize {
            
        }



        fmt_str = fmt_str+default::EXIT_BLOCK_MACRO;

    }

    println!("{}", fmt_str);
    
    return fmt_str;


}
