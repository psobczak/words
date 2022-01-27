use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    str::FromStr,
};

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum WordError {
    #[error("Word must be 5 characters long. Given word has length of '{0}'")]
    InvalidWordLength(usize),
    #[error("Can not parse given char '{0}' as wildcar or normal char")]
    InvalidCharValue(char),
}

#[derive(Debug)]
pub struct Excluded(Vec<char>);

impl FromStr for Excluded {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Excluded(s.chars().collect()))
    }
}

#[derive(Debug)]
pub struct Word(Vec<Character>);

impl<'a> Word {
    pub fn word_is_matching(&self, target: &'a str) -> Option<&'a str> {
        let target_word: Word = target.parse().unwrap();
        for (self_char, target_char) in self.0.iter().zip(target_word.0.iter()) {
            if self_char != target_char {
                return None;
            }
        }
        Some(target)
    }
}

impl FromStr for Word {
    type Err = WordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(WordError::InvalidWordLength(s.len()));
        };

        let mut characters: Vec<Character> = Vec::new();

        for c in s.chars().into_iter() {
            characters.push(Character::try_from(c)?);
        }

        Ok(Word(characters))
    }
}

#[derive(Debug, PartialEq)]
enum Character {
    Normal(char),
    Wildcard,
}

impl TryFrom<char> for Character {
    type Error = WordError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '*' | '_' | '?' => Ok(Self::Wildcard),
            c if c.is_alphabetic() => Ok(Self::Normal(value.to_ascii_uppercase())),
            _ => Err(WordError::InvalidCharValue(value)),
        }
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_error_if_word_is_longer_than_5_chars() {
        let actual: Result<Word, WordError> = "absd__".parse();
        let expected = WordError::InvalidWordLength(6);
        assert_eq!(actual.unwrap_err(), expected);
    }

    #[test]
    fn should_return_error_if_word_is_shorter_than_5_chars() {
        let actual: Result<Word, WordError> = "absd".parse();
        let expected = WordError::InvalidWordLength(4);
        assert_eq!(actual.unwrap_err(), expected);
    }

    #[test]
    fn should_return_error_if_not_alpabetic_or_wildcard_char() {
        let actual = Character::try_from('-').unwrap_err();
        let expected = WordError::InvalidCharValue('-');
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_5_char_long_word() {
        let word = "A?_?c";
        let actual: Word = word.parse().unwrap();
        assert_eq!(actual.0[0], Character::Normal('A'));
        assert_eq!(actual.0[1], Character::Wildcard);
        assert_eq!(actual.0[2], Character::Wildcard);
        assert_eq!(actual.0[3], Character::Wildcard);
        assert_eq!(actual.0[4], Character::Normal('C'));
    }

    #[test]
    fn should_return_matching_word() {
        let words = vec!["aahed", "aalii", "aargh", "aaron"];
        let chosen_word: Word = "aargh".parse().unwrap();

        assert_eq!(chosen_word.word_is_matching(words[0]), None);
        assert_eq!(chosen_word.word_is_matching(words[1]), None);
        assert_eq!(chosen_word.word_is_matching(words[2]), Some("aargh"));
        assert_eq!(chosen_word.word_is_matching(words[3]), None);

        
    }
}
