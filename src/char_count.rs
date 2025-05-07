use std::str::Chars;

#[derive(Debug, Default)]
pub struct CharCount {
    chinese: usize,
    english: usize,
    digit: usize,
    punctuation: usize,
    whitespace: usize,
    other: usize,
}

impl CharCount {
    pub fn is_chinese_char(c: char) -> bool {
        ('\u{4e00}'..='\u{9fff}').contains(&c)
            || ('\u{3400}'..='\u{4dbf}').contains(&c)
            || ('\u{20000}'..='\u{2a6df}').contains(&c)
    }

    pub fn merge(&mut self, other: &Self) {
        self.chinese += other.chinese;
        self.english += other.english;
        self.digit += other.digit;
        self.punctuation += other.punctuation;
        self.whitespace += other.whitespace;
        self.other += other.other
    }

    pub fn count(&mut self, chars: Chars<'_>) {
        for c in chars {
            if CharCount::is_chinese_char(c) {
                self.chinese += 1;
            } else if c.is_ascii_alphabetic() {
                self.english += 1;
            } else if c.is_ascii_digit() {
                self.digit += 1;
            } else if c.is_ascii_punctuation() {
                self.punctuation += 1;
            } else if c.is_ascii_whitespace() {
                self.whitespace += 1;
            } else {
                self.other += 1;
            }
        }
    }

    pub fn total_non_whitespace(&self) -> usize {
        self.chinese + self.english + self.digit + self.punctuation + self.other
    }

    pub fn total(&self) -> usize {
        self.total_non_whitespace() + self.whitespace
    }

    pub fn get_display_str(&self) -> String {
        format!(
            "中文字符: {}\n英文字符: {}\n数字: {}\n标点符号: {}\n空格: {}\n其他: {}",
            self.chinese, self.english, self.digit, self.punctuation, self.whitespace, self.other
        )
    }
}
