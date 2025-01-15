
pub enum FilterType {
    Random,
    Newest,
    Frequent,
    Recent,
    Starred,
    AlphabeticalByName,
    AlphabeticalByArtist,
}

impl FilterType {
    pub fn get_name(&self) -> &str {
        match self {
            FilterType::Random => "Random",
            FilterType::Newest => "Newest",
            FilterType::Frequent => "Frequent",
            FilterType::Recent => "Recent",
            FilterType::Starred => "Starred",
            FilterType::AlphabeticalByName => "Alphabetical by Name",
            FilterType::AlphabeticalByArtist => "Alphabetical by Artist",
        }
    }

    pub fn get_type(&self) -> &str {
        match self {
            FilterType::Random => "random",
            FilterType::Newest => "newest",
            FilterType::Frequent => "frequent",
            FilterType::Recent => "recent",
            FilterType::Starred => "starred",
            FilterType::AlphabeticalByName => "alphabeticalByName",
            FilterType::AlphabeticalByArtist => "alphabeticalByArtist",
        }
    }

    pub fn from_string(value: &str) -> FilterType {
        match value {
            "random" => FilterType::Random,
            "newest" => FilterType::Newest,
            "frequent" => FilterType::Frequent,
            "recent" => FilterType::Recent,
            "starred" => FilterType::Starred,
            "alphabeticalByName" => FilterType::AlphabeticalByName,
            "alphabeticalByArtist" => FilterType::AlphabeticalByArtist,
            _ => FilterType::Random,
        }
    }
}