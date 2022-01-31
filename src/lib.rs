use std::{
    fmt::Display,
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
pub struct Excluded(pub Vec<char>);

#[derive(Debug)]
pub struct Included(pub Vec<char>);

impl FromStr for Included {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Included(
            s.chars().map(|c| char::to_ascii_uppercase(&c)).collect(),
        ))
    }
}

impl FromStr for Excluded {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Excluded(
            s.chars().map(|c| char::to_ascii_uppercase(&c)).collect(),
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct Word(Vec<Character>);

impl Word {
    fn new(word: &str) -> Result<Self, WordError> {
        let output: Word = word.parse()?;
        Ok(output)
    }
}

#[derive(Debug)]
pub struct WordsResult {
    chosen_word: Word,
    pub possible_words: Vec<Word>,
}

impl<'a> WordsResult {
    pub fn new(chosen_word: Word) -> Self {
        Self {
            chosen_word,
            possible_words: Vec::new(),
        }
    }

    pub fn is_word_possible(
        &mut self,
        target: &'a str,
        excluded: &Excluded,
        included: &Included,
    ) -> bool {
        let target_word: Word = target.parse().unwrap();
        for (self_char, target_char) in self.chosen_word.0.iter().zip(target_word.0.iter()) {
            let self_character = match self_char {
                Character::Normal(c) => c,
                Character::Wildcard => continue,
            };

            let target_character = match target_char {
                Character::Normal(c) => c,
                _ => &' ',
            };

            if included.0.contains(target_character) {
                self.possible_words.push(target_word);
                return true;
            }

            if excluded.0.contains(self_character) {
                return false;
            };

            if self_char != target_char {
                return false;
            };
        }

        self.possible_words.push(target_word);
        true
    }
}

impl Display for WordsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "List of possible matching words:\n")?;
        for (i, word) in self.possible_words.iter().enumerate() {
            let str_word: Vec<&char> = word
                .0
                .iter()
                .map(|c| match c {
                    Character::Normal(value) => value,
                    _ => &' ',
                })
                .collect();

            write!(f, "{}. {:?}\t\n", i + 1, str_word)?;
        }

        Ok(())
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

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Character::Normal(v) => v.to_ascii_uppercase(),
                _ => ' ',
            }
        )
    }
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
        let word = "A?_*c";
        let actual: Word = word.parse().unwrap();
        assert_eq!(actual.0[0], Character::Normal('A'));
        assert_eq!(actual.0[1], Character::Wildcard);
        assert_eq!(actual.0[2], Character::Wildcard);
        assert_eq!(actual.0[3], Character::Wildcard);
        assert_eq!(actual.0[4], Character::Normal('C'));
    }

    #[test]
    fn should_return_matching_word() {
        let excluded = Excluded(vec!['w']);
        let included = Included(vec![]);
        let words = vec!["aahed", "aalii", "aargh", "zowie", "zorro"];
        let chosen_word = Word::new("aargh").unwrap();
        let mut result = WordsResult::new(chosen_word);

        assert_eq!(result.is_word_possible(words[0], &excluded, &included), false);
        assert_eq!(result.is_word_possible(words[1], &excluded, &included), false);
        assert_eq!(result.is_word_possible(words[2], &excluded, &included), true);
        assert_eq!(result.is_word_possible(words[3], &excluded, &included), false);
        assert_eq!(result.possible_words.len(), 1);
        assert_eq!(result.possible_words[0], Word::new("aargh").unwrap());
    }

    #[test]
    fn should_return_none_if_word_contains_excluded_char() {
        let excluded = Excluded(vec!['w']);
        let included = Included(vec![]);
        let words = vec!["zowie"];
        let chosen_word = Word::new("aargh");
        let mut result = WordsResult::new(chosen_word.unwrap());

        assert_eq!(result.is_word_possible(words[0], &excluded, &included), false);
        assert_eq!(result.possible_words.len(), 0);
    }

    #[test]
    fn should_return_both_words_if_excluded_char_is_wildcard() {
        let excluded = Excluded(vec!['m']);
        let included = Included(vec![]);
        let words = vec!["zorro", "morro"];
        let chosen_word = Word::new("*orro").unwrap();
        let mut result = WordsResult::new(chosen_word);

        assert_eq!(result.is_word_possible(words[0], &excluded, &included), true);
        assert_eq!(result.is_word_possible(words[1], &excluded, &included), true);

        assert_eq!(result.possible_words.len(), 2);
        assert_eq!(result.possible_words[0], Word::new("zorro").unwrap());
        assert_eq!(result.possible_words[1], Word::new("morro").unwrap());
    }

    #[test]
    fn should_return_word_if_it_matches_completly() {
        let excluded = Excluded(vec![]);
        let included = Included(vec![]);
        let words = vec!["zowie", "aaron"];
        let chosen_word = Word::new("zowie").unwrap();
        let mut result = WordsResult::new(chosen_word);

        assert_eq!(result.is_word_possible(words[0], &excluded, &included), true);
        assert_eq!(result.possible_words.len(), 1);
        assert_eq!(result.possible_words[0], Word::new("zowie").unwrap());
    }

    #[test]
    fn should_return_word_if_it_matches_with_wildcards() {
        let excluded = Excluded(vec![]);
        let included = Included(vec![]);
        let words = vec!["zowie", "aaron"];
        let chosen_word = Word::new("z?*ie").unwrap();
        let mut result = WordsResult::new(chosen_word);

        assert_eq!(result.is_word_possible(words[0], &excluded, &included), true);
        assert_eq!(result.possible_words.len(), 1);
        assert_eq!(result.possible_words[0], Word::new("zowie").unwrap());
    }

    #[test]
    fn should_return_words_containing_included_chars() {
        let excluded = Excluded(vec![]);
        let included = Included(vec!['i']);
        let words = vec!["light", "focus"];
        let chosen_word = Word::new("*****").unwrap();
        let mut result = WordsResult::new(chosen_word);

        assert_eq!(result.is_word_possible(words[0], &excluded, &included), true);
        assert_eq!(result.possible_words.len(), 1);
        assert_eq!(result.possible_words[0], Word::new("light").unwrap());
    }
}
