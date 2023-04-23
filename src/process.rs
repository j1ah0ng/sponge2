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

        if opt.newlines {
            fmt_str += "\\begin{array}{c}";
        }


        let parts = out_str.split(" ");

        for part in parts {

            if opt.colorize {

                fmt_str += "\\f{";
                let intermediate = get_hex(color_state);
                fmt_str += &intermediate;
                fmt_str += "}";
                fmt_str += "{";
                fmt_str += part;
                fmt_str += " }";

            } else {
                fmt_str += "\\f{";
                fmt_str += part;
                fmt_str += " }";
            }
            if  opt.newlines {
                fmt_str += "\\\\";
            }
        }

        fmt_str += "\\end{array}";


        fmt_str += default::EXIT_BLOCK_MACRO;

    } else {
        if opt.newlines {
            let parts = out_str.split(" ");
            for part in parts {
                fmt_str += part;
                fmt_str += "\n";
            }
        } else {
            fmt_str += out_str;
        }
    }

    return fmt_str;


}
