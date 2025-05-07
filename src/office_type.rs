use std::str::FromStr;

#[derive(Debug, Eq, Clone, Copy, PartialEq)]
pub enum OfficeType {
    Doc,
    Docx,
    Txt,
    Xlsx,
    Other,
}

impl FromStr for OfficeType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "doc" => OfficeType::Doc,
            "docx" => OfficeType::Docx,
            "txt" => OfficeType::Txt,
            "xlsx" => OfficeType::Xlsx,
            _ => OfficeType::Other,
        })
    }
}
