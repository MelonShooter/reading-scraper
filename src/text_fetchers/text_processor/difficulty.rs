pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Copy, Clone)]
pub enum Language {
    English,
    Spanish,
}

impl Language {
    pub(crate) fn get_text_difficulty(&self, text: &str) -> Difficulty {
        match self {
            Language::English => todo!(),
            Language::Spanish => todo!(),
        }
    }
}
