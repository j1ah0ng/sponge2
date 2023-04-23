use copypasta_ext::prelude::*;
use copypasta_ext::x11_bin::ClipboardContext;

pub fn clip(in_str: &String) {
    let mut ctx = ClipboardContext::new().unwrap();
    //println!("{:?}", ctx.get_contents());
    ctx.set_contents(in_str.into()).unwrap();
}
