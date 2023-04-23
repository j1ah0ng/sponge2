use libm::fabs;
use libm::fmod;

pub struct ColorFSM {
    pub state: u32,
}

pub fn get_hex<'life>(color_state: &mut ColorFSM) -> String {

    color_state.state += 10;
    color_state.state %= 360;

    let mut rgb_color: u32 = 0;
    let chroma: f64 = 200.0;
    let m: f64 = 200.0 - chroma;
    let x: f64 = chroma * (1.0 - fabs(fmod(color_state.state as f64 / 60.0, 2.0) - 1.0));

    if color_state.state <= 60 {
        rgb_color += chroma as u32 + m as u32;
        rgb_color <<= 8;
        rgb_color += x as u32 + m as u32;
        rgb_color <<= 8;
        rgb_color += m as u32;
        rgb_color <<= 8;
    } else if color_state.state > 60 && color_state.state <= 120 {
        rgb_color += x as u32 + m as u32;
        rgb_color <<= 8;
        rgb_color += chroma as u32 + m as u32;
        rgb_color <<= 8;
        rgb_color += m as u32;
        rgb_color <<= 8;
    } else if color_state.state > 120 && color_state.state <= 180 {
        rgb_color += m as u32;
        rgb_color <<= 8;
        rgb_color += chroma as u32 + m as u32;
        rgb_color <<= 8;
        rgb_color += x as u32 + m as u32;
        rgb_color <<= 8;
    } else if color_state.state > 180 && color_state.state <= 240 {
        rgb_color += m as u32;
        rgb_color <<= 8;
        rgb_color += x as u32 + m as u32;
        rgb_color <<= 8;
        rgb_color += chroma as u32 + m as u32;
        rgb_color <<= 8;
    } else if color_state.state > 240 && color_state.state <= 300 {
        rgb_color += x as u32 + m as u32;
        rgb_color <<= 8;
        rgb_color += m as u32;
        rgb_color <<= 8;
        rgb_color += chroma as u32 + m as u32;
        rgb_color <<= 8;
    } else if color_state.state > 300 && color_state.state <= 360 {
        rgb_color += chroma as u32 + m as u32;
        rgb_color <<= 8;
        rgb_color += m as u32;
        rgb_color <<= 8;
        rgb_color += x as u32 + m as u32;
        rgb_color <<= 8;
    } else {
        rgb_color += m as u32;
        rgb_color <<= 8;
        rgb_color += m as u32;
        rgb_color <<= 8;
        rgb_color += m as u32;
        rgb_color <<= 8;
    }

    let mut rgb_hex = format!("{:x}", rgb_color);
    if (rgb_hex.len() != 6) {
        rgb_hex.truncate(rgb_hex.len() - 2);
    }

    return rgb_hex;
}
