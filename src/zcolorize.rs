use owo_colors::{OwoColorize, Stream};

pub trait ZColorize {
    fn z_bright_blue(&self) -> String;
    fn z_magenta(&self) -> String;
    fn z_green(&self) -> String;
    fn z_yellow(&self) -> String;
}

impl ZColorize for str {
    fn z_bright_blue(&self) -> String {
        self.if_supports_color(Stream::Stdout, |text| text.bright_blue()).to_string()
    }
    fn z_magenta(&self) -> String {
        self.if_supports_color(Stream::Stdout, |text| text.magenta()).to_string()
    }
    fn z_green(&self) -> String {
        self.if_supports_color(Stream::Stdout, |text| text.green()).to_string()
    }
    fn z_yellow(&self) -> String {
        self.if_supports_color(Stream::Stdout, |text| text.yellow()).to_string()
    }
}
