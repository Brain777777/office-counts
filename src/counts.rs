use crate::{CharCount, OfficeType};
use anyhow::Result;
use dotext::{Docx, MsDoc, Xlsx};
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
    str::FromStr,
};

pub fn read_office_counts(path: &Path) -> Result<CharCount> {
    let ext = path.extension();
    if let Some(ext) = ext {
        let ext = <&str>::try_from(ext)?;
        let doc_type = OfficeType::from_str(ext)?;
        let ct = match doc_type {
            OfficeType::Docx => read_docx_counts(path)?,
            OfficeType::Xlsx => read_xlsx_counts(path)?,
            OfficeType::Txt => read_txt_counts(path)?,
            _ => CharCount::default(),
        };
        Ok(ct)
    } else {
        Ok(CharCount::default())
    }
}

fn read_txt_counts(path: &Path) -> Result<CharCount> {
    let file = File::open(path)?;
    read_counts(file)
}

fn read_xlsx_counts(path: &Path) -> Result<CharCount> {
    let file = Xlsx::open(path)?;
    read_counts(file)
}

fn read_docx_counts(path: &Path) -> Result<CharCount> {
    let file = Docx::open(path)?;
    read_counts(file)
}

// fn read_doc_counts(path: &str) -> Result<usize> {
//     Ok(0)
// }

fn read_counts<T: Read>(file: T) -> Result<CharCount> {
    let reader = BufReader::new(file);
    let mut ct = CharCount::default();
    for line_result in reader.lines() {
        let line = line_result?;
        ct.count(line.chars());
    }
    Ok(ct)
}
