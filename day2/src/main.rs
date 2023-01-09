use enum_iterator::{all, Sequence};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Sequence)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn matching_move(self, theirs: Move) -> Move {
        match self {
            Outcome::Win => theirs.winning_move(),
            Outcome::Draw => theirs.draw_move(),
            Outcome::Loss => theirs.loosing_move(),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("not a valid outcome: {c:?}")),
        }
    }
}

impl Move {
    fn points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn winning_move(self) -> Self {
        all::<Move>()
            .find(|&m| m.beats(self))
            .expect("at least one move beats us")
    }

    fn loosing_move(self) -> Self {
        all::<Move>()
            .find(|&m| self.beats(m))
            .expect("at least one move beats us")
    }

    fn draw_move(self) -> Self {
        self
    }
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

impl Round {
    fn outcome(self) -> Outcome {
        if self.ours.beats(self.theirs) {
            Outcome::Win
        } else if self.theirs.beats(self.ours) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    fn our_score(self) -> usize {
        self.ours.points() + self.outcome().points()
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let theirs: char;
        let outcome: char;

        if let (Some(t), Some(' '), Some(o), None) =
            (chars.next(), chars.next(), chars.next(), chars.next())
        {
            theirs = t;
            outcome = o;
        } else {
            return Err(color_eyre::eyre::eyre!(
                "expected pattern <their><space><ours>EOF, got {s:?}"
            ));
        };

        let theirs = theirs.try_into()?;
        let outcome: Outcome = outcome.try_into()?;
        let ours = outcome.matching_move(theirs);

        Ok(Self { theirs, ours })
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let file_path: &str = "/Users/jijeshmohan/Learn/rust/aoc/day2/input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let results: usize = reader
        .lines()
        .filter_map(|f| f.ok())
        .map(|line| line.parse::<Round>())
        .map(|round| round.expect("Unable to parse to round").our_score())
        .sum();

    // let total_score: usize = results.iter().sum();

    dbg!(results);

    Ok(())
}
