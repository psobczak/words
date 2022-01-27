use structopt::StructOpt;
use words::{read_lines, Excluded, Word};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    if let Ok(lines) = read_lines("src/words.txt") {
        for line in lines {
            if let Ok(word) = line {
                if let Some(output) = opt.word.word_is_matching(&word) {
                    println!("{:?}", output);
                }
            }
        }
    };

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
}
