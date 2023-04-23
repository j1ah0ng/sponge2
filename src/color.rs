
pub struct ColorFSM {
    pub state: u32,
}

pub fn get_hex(color_state: &mut ColorFSM) -> u32 {

    color_state.state += 10;

    println!("{}", color_state.state);

    return 3;
}
