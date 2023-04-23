use crate::default;
use crate::types::Opt;
use crate::color::ColorFSM;
use crate::color::get_hex;

pub fn process(out_str: &String, opt: &Opt, color_state: &mut ColorFSM) -> String {


    println!("{}",out_str);

    let mut fmt_str: String = "".to_string();

    if opt.latex {

        fmt_str += default::ENTER_BLOCK_MACRO;
    
        if opt.colorize {
            fmt_str += default::COLOR_MACRO;
        } else {
            fmt_str += default::TEXT_MACRO;
        }


        let parts = out_str.split(" ");

        for part in parts {

            if opt.colorize {

                fmt_str += "\\f{";
                let intermediate = get_hex(&mut color_state);
                fmt_str += &intermediate;
                fmt_str+= " }";
                fmt_str += "{";
                fmt_str += part;
                fmt_str+= " }";

            } else {
                fmt_str += "\\f{";
                fmt_str += part;
                fmt_str+= " }";
            }

        }


        fmt_str = fmt_str+default::EXIT_BLOCK_MACRO;

    }

    return fmt_str;


}
