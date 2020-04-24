
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorName {
    Bg,
    Fg,
    Fg0,
    Fg1,
    Fg2,
    Fg3,
    Fg4,
    Gray,
    LightGray,
    Red,
    LightRed,
    Green,
    LightGreen,
    Yellow,
    LightYellow,
    Blue,
    LightBlue,
    Purple,
    LightPurple,
    Aqua,
    LightAqua,
    Orange,
    LightOrange,
    Void,
    Stone0,
    Stone1,
    Stone2,
    Stone3,
    Stone4,
    Stone5,
    Stone6,
}

pub fn get_stone_color(val: &f64, min: &f64, max: &f64) -> ColorName {
    let min = *min;
    let max = *max;

    let step = (max - min) / 7.0;
    if (min..(min+step)).contains(&val) {
        ColorName::Void
    }
    else if (min+step..min+(2.0*step)).contains(&val) {
        ColorName::Stone1
    }
    else if (min+(2.0*step)..min+(3.0*step)).contains(&val) {
        ColorName::Stone2
    }
    else if (min+(3.0*step)..min+(4.0*step)).contains(&val) {
        ColorName::Stone3
    }
    else if (min+(4.0*step)..min+(5.0*step)).contains(&val) {
        ColorName::Stone4
    }
    //else if (min+(5.0*step)..min+(6.0*step)).contains(&val) {
    //    ColorName::Stone4
    //}
    else if (min+(5.0*step)..min+(6.0*step)).contains(&val) {
        ColorName::Stone5
    }
    else {
        ColorName::Stone6
    }
    
}

pub fn get_floor_color(val: &f64, min: &f64, max: &f64) -> ColorName {
    let min = *min;
    let max = *max;

    let step = (max - min) / 8.0;
    if (min..(min+step)).contains(&val) {
        ColorName::Stone6
    }
    else if (min+(2.0*step)..min+(3.0*step)).contains(&val) {
        ColorName::Stone5
    }
    else if (min+(3.0*step)..min+(4.0*step)).contains(&val) {
        ColorName::Stone4
    }
    else if (min+(4.0*step)..min+(5.0*step)).contains(&val) {
        ColorName::Stone3
    }
    else if (min+(5.0*step)..min+(6.0*step)).contains(&val) {
        ColorName::Stone2
    }
    else {
        ColorName::Void
    }
    
}

pub struct ColorScheme {
    pub bg: String,
    pub fg: String,
    pub fg0: String,
    pub fg1: String,
    pub fg2: String,
    pub fg3: String,
    pub fg4: String,
    pub gray: String,
    pub light_gray: String,
    pub red: String,
    pub light_red: String,
    pub green: String,
    pub light_green: String,
    pub yellow: String,
    pub light_yellow: String,
    pub blue: String,
    pub light_blue: String,
    pub purple: String,
    pub light_purple: String,
    pub aqua: String,
    pub light_aqua: String,
    pub orange: String,
    pub light_orange: String,
    pub void: String,
    pub stone0: String,
    pub stone1: String,
    pub stone2: String,
    pub stone3: String,
    pub stone4: String,
    pub stone5: String,
    pub stone6: String,
   
}

impl ColorScheme {
    pub fn get_color_code(&self, color_name: &ColorName) -> &String {
       match color_name {
           ColorName::Bg => &self.bg,
           ColorName::Fg => &self.fg,
           ColorName::Fg0 => &self.fg0,
           ColorName::Fg1 => &self.fg1,
           ColorName::Fg2 => &self.fg2,
           ColorName::Fg3 => &self.fg3,
           ColorName::Fg4 => &self.fg4,
           ColorName::Gray => &self.gray,
           ColorName::LightGray => &self.light_gray,
           ColorName::Red => &self.red,
           ColorName::LightRed => &self.light_red,
           ColorName::Green => &self.green,
           ColorName::LightGreen => &self.light_green,
           ColorName::Yellow => &self.yellow,
           ColorName::LightYellow => &self.light_yellow,
           ColorName::Blue => &self.blue,
           ColorName::LightBlue => &self.light_blue,
           ColorName::Purple => &self.purple,
           ColorName::LightPurple => &self.light_purple,
           ColorName::Aqua => &self.aqua,
           ColorName::LightAqua => &self.light_aqua,
           ColorName::Orange => &self.orange,
           ColorName::LightOrange => &self.light_orange,
           ColorName::Void => &self.void,
           ColorName::Stone0 => &self.stone0,
           ColorName::Stone1 => &self.stone1,
           ColorName::Stone2 => &self.stone2,
           ColorName::Stone3 => &self.stone3,
           ColorName::Stone4 => &self.stone4,
           ColorName::Stone5 => &self.stone5,
           ColorName::Stone6 => &self.stone6,
       }
    }
}
