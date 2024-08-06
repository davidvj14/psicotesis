#![allow(unused)
]
use leptos::*;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fmt::Display};

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
    pub fn compare_for_error(&self, card: Card) -> CardError {
        let mut matches = Vec::new();

        if self.number == card.number {
            matches.push(Criterion::Number);
        }
        if self.color == card.color {
            matches.push(Criterion::Color);
        }
        if self.shape == card.shape {
            matches.push(Criterion::Shape);
        }

        match matches.len() {
            0 => CardError::Other,
            1 => CardError::One(matches[0]),
            2 => CardError::Two(matches[0], matches[1]),
            _ => unreachable!("Error in compare_for_error"),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TestResult {
    pub score: i64,
    pub errors: i64,
    pub m_errors: i64,
    pub perseverations: i64,
    pub deferred_p: i64,
    pub ttf: i64,
    pub tae: i64,
    pub time: i64,
}

impl Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "Aciertos: {}\n\
            Errores: {}\n\
            Errores de mantenimiento: {}\n\
            Perseveraciones: {}\n\
            Perseveraciones diferidas: {}\n\
            Tiempo para primer movimiento: {}ms\n\
            Tiempo tras error (total): {}ms\n\
            Tiempo total: {}ms\n",
            self.score,
            self.errors,
            self.m_errors,
            self.perseverations,
            self.deferred_p,
            self.ttf,
            self.tae,
            self.time)
    }
}

impl TestResult {
    pub fn new() -> Self {
        Self {
            score: 0,
            errors: 0,
            m_errors: 0,
            perseverations: 0,
            deferred_p: 0,
            ttf: 0,
            tae: 0,
            time: 0,
        }
    }

    fn enqueue(queue: &mut VecDeque<Answer>, elem: Answer) {
        if (queue.len() == 4) {
            let _ = queue.pop_front();
        }
        queue.push_back(elem);
    }

    fn check_for_deferred(last4: &VecDeque<Answer>, grade: &Grade) -> bool {
        if let Some(g) = last4.get(0) {
            if g.grade == *grade {
                return true;
            }
        }
        if let Some(g) = last4.get(1) {
            if g.grade == *grade {
                return true;
            }
        }
        if let Some(g) = last4.get(2) {
            if g.grade == *grade {
                return true;
            }
        }
        false
    }

    fn check_for_perseveration(last4: &VecDeque<Answer>, grade: &Grade) -> bool {
        match last4.back() {
            None => false,
            Some(g) => match g.grade {
                Grade::Correct => false,
                Grade::Incorrect(_, _) => g.grade == *grade
            },
        }
    }

    fn check_for_merror(last4: &VecDeque<Answer>, grade: &Grade) -> bool {
        if let Some(Grade::Correct) = last4.get(0).and_then(|x| Some(x.grade.clone())) {
            if let Some(Grade::Correct) = last4.get(1).and_then(|x| Some(x.grade.clone())) {
                if let Some(Grade::Correct) = last4.get(2).and_then(|x| Some(x.grade.clone())) {
                    return true;
                }
            }
        }
        false
    }

    fn eval_step(&mut self, last4: &VecDeque<Answer>, answer: &Answer) {
        self.time += answer.time_taken;
        if let Some(Grade::Incorrect(_, _)) = last4.back().and_then(|x| Some(x.grade.clone())) {
            self.tae += answer.time_taken;
        }
        match answer.grade {
            Grade::Correct => self.score += 1,
            Grade::Incorrect(_, _) => {
                if Self::check_for_perseveration(last4, &answer.grade) {
                    self.perseverations += 1;
                } else if Self::check_for_deferred(last4, &answer.grade) {
                    self.deferred_p += 1;
                } else if Self::check_for_merror(last4, &answer.grade) {
                    self.m_errors += 1;
                } else {
                    self.errors += 1;
                }
            }
        }
    }

    pub fn eval(answers: &Vec<Answer>) -> Self {
        let mut result = Self::new();
        let mut previous_err: Option<CardError> = None;
        let mut last4: VecDeque<Answer> = VecDeque::with_capacity(4);
        logging::log!("{:?}", answers[0].time_taken);
        result.ttf = answers[0].time_taken;

        for answer in answers {
            logging::log!("{:?}", last4);
            result.eval_step(&last4, answer);
            Self::enqueue(&mut last4, answer.clone());
        }

        result
    }

}

#[derive(Clone, Copy, Debug)]
pub enum CardError {
    Other,
    One(Criterion),
    Two(Criterion, Criterion),
}

impl PartialEq for CardError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            CardError::Other => match other {
                CardError::Other => true,
                _ => false,
            },
            CardError::One(c1) => match other {
                CardError::Other => false,
                CardError::One(c2) => c1 == c2,
                CardError::Two(c2, c3) => c1 == c2 || c1 == c3,
            },
            CardError::Two(c1, c2) => match other {
                CardError::Other => false,
                CardError::One(c3) => c1 == c3 || c2 == c3,
                CardError::Two(c3, c4) => c1 == c3 || c1 == c4 || c2 == c3 || c2 == c4,
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Grade {
    Correct,
    Incorrect(Criterion, CardError),
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
                answers.push(Answer::new(
                    Grade::Incorrect(
                        Criterion::Shape,
                        CRITERION_CARDS[id].compare_for_error(card),
                    ),
                    time,
                ));
                return (answers, false);
            }
        }
        Criterion::Color => {
            if card.color == CRITERION_CARDS[id].color {
                score.set(score.get() + 1);
                answers.push(Answer::new(Grade::Correct, time));
            } else {
                answers.push(Answer::new(
                    Grade::Incorrect(
                        Criterion::Color,
                        CRITERION_CARDS[id].compare_for_error(card),
                    ),
                    time,
                ));
                return (answers, false);
            }
        }
        Criterion::Number => {
            if card.number == CRITERION_CARDS[id].number {
                score.set(score.get() + 1);
                answers.push(Answer::new(Grade::Correct, time));
            } else {
                answers.push(Answer::new(
                    Grade::Incorrect(
                        Criterion::Number,
                        CRITERION_CARDS[id].compare_for_error(card),
                    ),
                    time,
                ));
                return (answers, false);
            }
        }
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
