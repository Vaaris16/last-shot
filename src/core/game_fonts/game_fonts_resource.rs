
pub enum GameFonts {
    OxaniumBold,
    OxaniumMedium,
}

impl GameFonts {
    pub fn font_path(&self) -> &'static str {
        match self {
            Self::OxaniumMedium => "fonts/oxanium_font/static/Oxanium-Medium.ttf",
            Self::OxaniumBold => "fonts/oxanium_font/static/Oxanium-Bold.ttf",
        }
    }
}
