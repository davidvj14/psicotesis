use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CardShape {
    S1,
    S2,
    S3,
    S4,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CardColor {
    C1,
    C2,
    C3,
    C4,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CardNumber {
    N1,
    N2,
    N3,
    N4,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Criterion {
    Shape,
    Color,
    Number,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Card {
    pub image: &'static str,
    pub shape: CardShape,
    pub color: CardColor,
    pub number: CardNumber,
}

pub enum Grade {
    First,
    Correct,
    Incorrect(Criterion, Criterion),
}

pub struct Answer {
    pub result: Grade,
    time_taken: i64,
}

pub struct CriterionSet {
    criterion: Criterion,
    answers: Vec<Answer>,
}

pub fn eval_answer(
    current_criterion: Criterion,
    last_card: Option<Card>,
    dropped_card: Card,
    answers: &mut Vec<Grade>,
) {
    match current_criterion {
        Criterion::Shape => match last_card {
            None => answers.push(Grade::First),
            Some(card) => {
                if card.shape == dropped_card.shape {
                    answers.push(Grade::Correct);
                } else {
                    answers.push(Grade::Incorrect(Criterion::Shape, Criterion::Shape));
                }
            }
        },
        Criterion::Color => match last_card {
            None => answers.push(Grade::First),
            Some(card) => {
                if card.color == dropped_card.color {
                    answers.push(Grade::Correct);
                } else {
                    answers.push(Grade::Incorrect(Criterion::Shape, Criterion::Shape));
                }
            }
        },
        Criterion::Number => match last_card {
            None => answers.push(Grade::First),
            Some(card) => {
                if card.number == dropped_card.number {
                    answers.push(Grade::Correct);
                } else {
                    answers.push(Grade::Incorrect(Criterion::Shape, Criterion::Shape));
                }
            }
        },
    }
}

pub static CARDS: [Card; 64] = [
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
    Card {
        image: "card1.png",
        shape: CardShape::S1,
        color: CardColor::C1,
        number: CardNumber::N1,
    },
];
