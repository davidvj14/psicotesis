#![allow(unused)]
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CardShape {
    Square,
    Rhombus,
    Trapeze,
    Octagon,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CardColor {
    Blue,
    Brown,
    Cyan,
    Red,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CardNumber {
    One,
    Two,
    Three,
    Four,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
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

impl Card {
    pub fn compare(&self, card: Card) -> Option<Criterion> {
        if self.number == card.number {
            return Some(Criterion::Number)
        } else if self.color == card.color {
            return Some(Criterion::Color)
        } else if self.shape == card.shape {
            return Some(Criterion::Shape)
        }
        
        None
    }
}

struct TestResult {
    score: u64,
    errors: u64,
    m_errors: u64,
    perseverations: u64,
    deferred_p: u64,
    ttf: i64,
    tae: f64,
    time: i64,
}

impl TestResult {
    pub fn new(score: u64) -> Self {
        Self {
            score,
            errors: 0,
            m_errors: 0,
            perseverations: 0,
            deferred_p: 0,
            ttf: 0,
            tae: 0.0,
            time: 0,
        }
    }

    pub fn calc_perseverations(&mut self, answers: &Vec<Answer>) {
        let mut last_error = None;
        let mut perseverations = 0;
        for answer in answers {
            if let Grade::Incorrect(_, c) = answer.grade {
                if last_error == c {
                    perseverations += 1;
                } else {
                    last_error = c;
                }
            }
        }
        self.perseverations = perseverations
    }

    pub fn calc_deferred(&mut self, answers: &Vec<Answer>) {
        let mut last4 = ();
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Grade {
    First,
    Correct,
    Incorrect(Criterion, Option<Criterion>),
}

#[derive(Clone, Debug)]
pub struct Answer {
    pub grade: Grade,
    time_taken: i64,
}

impl Answer {
    pub fn new(grade: Grade, time_taken: i64) -> Self {
        Self { grade, time_taken }
    }
}

pub struct CriterionSet {
    criterion: Criterion,
    answers: Vec<Answer>,
}

pub fn eval_answer(
    current_criterion: Criterion,
    id: usize,
    card: Card,
    score: RwSignal<u64>,
    answers: RwSignal<Vec<Answer>>,
    time: i64,
) -> (Vec<Answer>, bool) {
    let mut answers = answers.get();
    match current_criterion {
        Criterion::Shape => {
            if card.shape == CRITERION_CARDS[id].shape {
                score.set(score.get() + 1);
                answers.push(Answer::new(Grade::Correct, time));
            } else {
                answers.push(Answer::new(Grade::Incorrect(Criterion::Shape, CRITERION_CARDS[id].compare(card)), time));
                return (answers, false);
            }
        },
        Criterion::Color => {
            if card.color == CRITERION_CARDS[id].color {
                score.set(score.get() + 1);
                answers.push(Answer::new(Grade::Correct, time));
            } else {
                answers.push(Answer::new(Grade::Incorrect(Criterion::Color, CRITERION_CARDS[id].compare(card)), time));
                return (answers, false);
            }
        },
        Criterion::Number => {
            if card.number == CRITERION_CARDS[id].number {
                score.set(score.get() + 1);
                answers.push(Answer::new(Grade::Correct, time));
            } else {
                answers.push(Answer::new(Grade::Incorrect(Criterion::Number, CRITERION_CARDS[id].compare(card)), time));
                return (answers, false);
            }
        },
    }
    (answers, true)
}

pub static CRITERIA: [Criterion; 6] = [
    Criterion::Color,
    Criterion::Shape,
    Criterion::Number,
    Criterion::Shape,
    Criterion::Number,
    Criterion::Color,
];

pub static CRITERION_CARDS: [Card; 4] = [
    Card {
        image: "card-sorting/init1.png",
        shape: CardShape::Square,
        color: CardColor::Cyan,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/init2.png",
        shape: CardShape::Octagon,
        color: CardColor::Red,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/init3.png",
        shape: CardShape::Rhombus,
        color: CardColor::Brown,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/init4.png",
        shape: CardShape::Trapeze,
        color: CardColor::Blue,
        number: CardNumber::Four,
    },
];

pub static CARDS: [Card; 64] = [
    Card {
        image: "card-sorting/card1.png",
        shape: CardShape::Square,
        color: CardColor::Blue,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card2.png",
        shape: CardShape::Rhombus,
        color: CardColor::Brown,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card3.png",
        shape: CardShape::Trapeze,
        color: CardColor::Cyan,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card4.png",
        shape: CardShape::Octagon,
        color: CardColor::Red,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card5.png",
        shape: CardShape::Rhombus,
        color: CardColor::Blue,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card6.png",
        shape: CardShape::Square,
        color: CardColor::Brown,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card7.png",
        shape: CardShape::Octagon,
        color: CardColor::Cyan,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card8.png",
        shape: CardShape::Rhombus,
        color: CardColor::Red,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card9.png",
        shape: CardShape::Octagon,
        color: CardColor::Blue,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card10.png",
        shape: CardShape::Trapeze,
        color: CardColor::Brown,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card11.png",
        shape: CardShape::Rhombus,
        color: CardColor::Cyan,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card12.png",
        shape: CardShape::Square,
        color: CardColor::Red,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card13.png",
        shape: CardShape::Trapeze,
        color: CardColor::Blue,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card14.png",
        shape: CardShape::Octagon,
        color: CardColor::Brown,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card15.png",
        shape: CardShape::Square,
        color: CardColor::Cyan,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card16.png",
        shape: CardShape::Trapeze,
        color: CardColor::Red,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card17.png",
        shape: CardShape::Octagon,
        color: CardColor::Cyan,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card18.png",
        shape: CardShape::Trapeze,
        color: CardColor::Red,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card19.png",
        shape: CardShape::Square,
        color: CardColor::Blue,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card20.png",
        shape: CardShape::Rhombus,
        color: CardColor::Brown,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card21.png",
        shape: CardShape::Rhombus,
        color: CardColor::Cyan,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card22.png",
        shape: CardShape::Octagon,
        color: CardColor::Red,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card23.png",
        shape: CardShape::Trapeze,
        color: CardColor::Blue,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card24.png",
        shape: CardShape::Square,
        color: CardColor::Brown,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card25.png",
        shape: CardShape::Trapeze,
        color: CardColor::Cyan,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card26.png",
        shape: CardShape::Square,
        color: CardColor::Red,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card27.png",
        shape: CardShape::Rhombus,
        color: CardColor::Blue,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card28.png",
        shape: CardShape::Octagon,
        color: CardColor::Brown,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card29.png",
        shape: CardShape::Square,
        color: CardColor::Cyan,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card30.png",
        shape: CardShape::Rhombus,
        color: CardColor::Red,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card31.png",
        shape: CardShape::Octagon,
        color: CardColor::Blue,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card32.png",
        shape: CardShape::Trapeze,
        color: CardColor::Brown,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card33.png",
        shape: CardShape::Octagon,
        color: CardColor::Brown,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card34.png",
        shape: CardShape::Rhombus,
        color: CardColor::Blue,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card35.png",
        shape: CardShape::Octagon,
        color: CardColor::Red,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card36.png",
        shape: CardShape::Square,
        color: CardColor::Cyan,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card37.png",
        shape: CardShape::Rhombus,
        color: CardColor::Brown,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card38.png",
        shape: CardShape::Octagon,
        color: CardColor::Blue,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card39.png",
        shape: CardShape::Square,
        color: CardColor::Red,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card40.png",
        shape: CardShape::Trapeze,
        color: CardColor::Cyan,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card41.png",
        shape: CardShape::Square,
        color: CardColor::Brown,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card42.png",
        shape: CardShape::Trapeze,
        color: CardColor::Blue,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card43.png",
        shape: CardShape::Rhombus,
        color: CardColor::Red,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card44.png",
        shape: CardShape::Octagon,
        color: CardColor::Cyan,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card45.png",
        shape: CardShape::Trapeze,
        color: CardColor::Brown,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card46.png",
        shape: CardShape::Square,
        color: CardColor::Blue,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card47.png",
        shape: CardShape::Octagon,
        color: CardColor::Red,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card48.png",
        shape: CardShape::Rhombus,
        color: CardColor::Cyan,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card49.png",
        shape: CardShape::Octagon,
        color: CardColor::Red,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card50.png",
        shape: CardShape::Trapeze,
        color: CardColor::Cyan,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card51.png",
        shape: CardShape::Rhombus,
        color: CardColor::Brown,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card52.png",
        shape: CardShape::Square,
        color: CardColor::Blue,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card53.png",
        shape: CardShape::Trapeze,
        color: CardColor::Red,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card54.png",
        shape: CardShape::Octagon,
        color: CardColor::Cyan,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card55.png",
        shape: CardShape::Square,
        color: CardColor::Brown,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card56.png",
        shape: CardShape::Rhombus,
        color: CardColor::Blue,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card57.png",
        shape: CardShape::Rhombus,
        color: CardColor::Red,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card58.png",
        shape: CardShape::Square,
        color: CardColor::Cyan,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card59.png",
        shape: CardShape::Octagon,
        color: CardColor::Brown,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card60.png",
        shape: CardShape::Trapeze,
        color: CardColor::Blue,
        number: CardNumber::Four,
    },
    Card {
        image: "card-sorting/card61.png",
        shape: CardShape::Square,
        color: CardColor::Red,
        number: CardNumber::One,
    },
    Card {
        image: "card-sorting/card62.png",
        shape: CardShape::Rhombus,
        color: CardColor::Cyan,
        number: CardNumber::Two,
    },
    Card {
        image: "card-sorting/card63.png",
        shape: CardShape::Trapeze,
        color: CardColor::Brown,
        number: CardNumber::Three,
    },
    Card {
        image: "card-sorting/card64.png",
        shape: CardShape::Octagon,
        color: CardColor::Blue,
        number: CardNumber::Four,
    },
];
