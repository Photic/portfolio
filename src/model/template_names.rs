#![allow(dead_code, unused_variables)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TemplateName {
    home,
    about,
    layout,
    not_found,
    arcade,
}

impl TemplateName {
    pub fn as_str(&self) -> &'static str {
        match self {
            TemplateName::home => "home",
            TemplateName::about => "about",
            TemplateName::layout => "layout",
            TemplateName::not_found => "not_found",
            TemplateName::arcade => "arcade",
        }
    }
}
