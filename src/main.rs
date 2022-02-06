use structopt::StructOpt;
use words::{read_lines, Excluded, Included, Word, WordsResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let mut result = WordsResult::new(opt.word);

    let excluded = match opt.excluded {
        Some(e) => e,
        None => Excluded(vec![]),
    };

    let included = match opt.included {
        Some(i) => i,
        None => Included(vec![]),
    };

    if let Ok(lines) = read_lines("src/words.txt") {
        for line in lines.flatten() {
            result.is_word_possible(line.as_str(), &excluded, &included);
        }
    };

    println!("{}", result);

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "words",
    about = "Simple program that helps you find anwser to wordle's word of the day."
)]
struct Opt {
    #[structopt(help = "5 character long word that you want to solve")]
    word: Word,
    #[structopt(short, long, help = "List of chars you want to omit")]
    excluded: Option<Excluded>,
    #[structopt(short, long, help = "List of chars you want to include")]
    included: Option<Included>,
}
