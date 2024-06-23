#[cfg(feature = "ssr")]
use std::num::ParseIntError;

use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

static QUESTIONS: [Question; 30] = [
    Question {
        // 1
        inverse: true,
        kind: ImpulsivityType::Unplanned,
    },
    Question {
        // 2
        inverse: false,
        kind: ImpulsivityType::Motor,
    },
    Question {
        // 3
        inverse: false,
        kind: ImpulsivityType::Unplanned,
    },
    Question {
        // 4
        inverse: false,
        kind: ImpulsivityType::Cognitive,
    },
    Question {
        // 5
        inverse: true,
        kind: ImpulsivityType::Unplanned,
    },
    Question {
        // 6
        inverse: true,
        kind: ImpulsivityType::Motor,
    },
    Question {
        // 7
        inverse: true,
        kind: ImpulsivityType::Cognitive,
    },
    Question {
        // 8
        inverse: true,
        kind: ImpulsivityType::Unplanned,
    },
    Question {
        // 9
        inverse: false,
        kind: ImpulsivityType::Motor,
    },
    Question {
        // 10
        inverse: true,
        kind: ImpulsivityType::Cognitive,
    },
    Question {
        // 11
        inverse: true,
        kind: ImpulsivityType::Unplanned,
    },
    Question {
        // 12
        inverse: false,
        kind: ImpulsivityType::Motor,
    },
    Question {
        // 13
        inverse: true,
        kind: ImpulsivityType::Cognitive,
    },
    Question {
        // 14
        inverse: false,
        kind: ImpulsivityType::Unplanned,
    },
    Question {
        // 15
        inverse: false,
        kind: ImpulsivityType::Unplanned,
    },
    Question {
        // 16
        inverse: false,
        kind: ImpulsivityType::Cognitive,
    },
    Question {
        // 17
        inverse: true,
        kind: ImpulsivityType::Unplanned,
    },
    Question {
        // 18
        inverse: false,
        kind: ImpulsivityType::Motor,
    },
    Question {
        // 19
        inverse: true,
        kind: ImpulsivityType::Cognitive,
    },
    Question {
        // 20
        inverse: false,
        kind: ImpulsivityType::Unplanned,
    },
    Question {
        // 21
        inverse: false,
        kind: ImpulsivityType::Motor,
    },
    Question {
        // 22
        inverse: true,
        kind: ImpulsivityType::Unplanned,
    },
    Question {
        // 23
        inverse: false,
        kind: ImpulsivityType::Motor,
    },
    Question {
        // 24
        inverse: false,
        kind: ImpulsivityType::Cognitive,
    },
    Question {
        // 25
        inverse: false,
        kind: ImpulsivityType::Unplanned,
    },
    Question {
        // 26
        inverse: false,
        kind: ImpulsivityType::Motor,
    },
    Question {
        // 27
        inverse: false,
        kind: ImpulsivityType::Cognitive,
    },
    Question {
        // 28
        inverse: false,
        kind: ImpulsivityType::Unplanned,
    },
    Question {
        // 29
        inverse: false,
        kind: ImpulsivityType::Motor,
    },
    Question {
        // 30
        inverse: true,
        kind: ImpulsivityType::Unplanned,
    },
];

pub enum ImpulsivityType {
    Cognitive,
    Motor,
    Unplanned,
}

#[derive(Clone)]
struct ImpulsivityResult {
    pub cognitive: i32,
    pub motor: i32,
    pub unplanned: i32,
}

impl<'a> ImpulsivityResult {
    pub fn eval(qa: &Vec<QuestionAnswer<'a>>) -> Self {
        let mut result = Self {
            cognitive: 0,
            motor: 0,
            unplanned: 0,
        };

        qa.iter().for_each(|qa| result.add_points(qa));

        result
    }

    fn add_points(&mut self, qa: &QuestionAnswer<'a>) {
        match qa.get_answer() {
            0 => (),
            x => match qa.q.kind {
                ImpulsivityType::Cognitive => self.cognitive += x,
                ImpulsivityType::Motor => self.motor += x,
                ImpulsivityType::Unplanned => self.unplanned += x,
            },
        }
    }
}

pub struct Question {
    inverse: bool,
    pub kind: ImpulsivityType,
}

pub struct QuestionAnswer<'a> {
    pub q: &'a Question,
    answer: u8,
}

impl<'a> QuestionAnswer<'a> {
    fn get_answer(&self) -> i32 {
        if self.q.inverse {
            match self.answer {
                1 => 4,
                2 => 3,
                3 => 1,
                4 => 0,
                _ => 0,
            }
        } else {
            match self.answer {
                1 => 0,
                2 => 1,
                3 => 3,
                4 => 4,
                _ => 0,
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BarrData {
    bq1: u8,
    bq2: u8,
    bq3: u8,
    bq4: u8,
    bq5: u8,
    bq6: u8,
    bq7: u8,
    bq8: u8,
    bq9: u8,
    bq10: u8,
    bq11: u8,
    bq12: u8,
    bq13: u8,
    bq14: u8,
    bq15: u8,
    bq16: u8,
    bq17: u8,
    bq18: u8,
    bq19: u8,
    bq20: u8,
    bq21: u8,
    bq22: u8,
    bq23: u8,
    bq24: u8,
    bq25: u8,
    bq26: u8,
    bq27: u8,
    bq28: u8,
    bq29: u8,
    bq30: u8,
}

impl<'a> BarrData {
    pub fn to_array(&self) -> [u8; 30] {
        [
            self.bq1, self.bq2, self.bq3, self.bq4, self.bq5, self.bq6, self.bq7, self.bq8,
            self.bq9, self.bq10, self.bq11, self.bq12, self.bq13, self.bq14, self.bq15, self.bq16,
            self.bq17, self.bq18, self.bq19, self.bq20, self.bq21, self.bq22, self.bq23, self.bq24,
            self.bq25, self.bq26, self.bq27, self.bq28, self.bq29, self.bq30,
        ]
    }

    pub fn construct_answers(ans: [u8; 30]) -> Vec<QuestionAnswer<'a>> {
        let mut answers = Vec::new();
        for i in 0..30 {
            answers.push(QuestionAnswer {
                q: &QUESTIONS[i],
                answer: ans[i],
            })
        }

        answers
    }
}

#[cfg(feature = "ssr")]
fn parse_id_cookie(cookie: &str) -> Result<i32, ParseIntError> {
    return Ok(cookie[5..].parse()?);
}

#[cfg(feature = "ssr")]
pub async fn get_id_cookie() -> Result<i32, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;

    let hm: HeaderMap = extract().await?;
    if let Some(cookie_header) = hm.get("cookie") {
        if let Ok(cookie_header_str) = cookie_header.to_str() {
            let cookie = parse_id_cookie(cookie_header_str)?;
            println!("{cookie:?}");
            return Ok(cookie);
        }
    }
    Err(ServerFnError::MissingArg(String::from("Bad p_id cookie")))
}

#[server(ProcessBarrat)]
pub async fn process_barrat(data: BarrData) -> Result<(), ServerFnError> {
    use crate::app::ssr::*;

    let cookie = get_id_cookie().await?;
    let arr = data.to_array();
    let answers = BarrData::construct_answers(arr);

    let answers_str: String = arr
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");
    let results = ImpulsivityResult::eval(&answers);
    let conn = &mut db().await.unwrap();

    sqlx::query("INSERT INTO barrat VALUES ($1, $2, $3, $4, $5)")
        .bind(cookie)
        .bind(results.cognitive)
        .bind(results.motor)
        .bind(results.unplanned)
        .bind(answers_str)
        .execute(conn)
        .await?;
    Ok(())
}
