use strum_macros::Display;

#[derive(Display, Debug)]
pub enum DateFormat {
    #[strum(serialize = "%Y-%m-%d %H:%M:%S")]
    Normal,
}