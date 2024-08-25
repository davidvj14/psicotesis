use leptos::{server, ServerFnError, expect_context};
use serde::{Deserialize, Serialize};
use crate::barrat_extras::*;


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



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BarrData {
    bq1: i64,
    bq2: i64,
    bq3: i64,
    bq4: i64,
    bq5: i64,
    bq6: i64,
    bq7: i64,
    bq8: i64,
    bq9: i64,
    bq10: i64,
    bq11: i64,
    bq12: i64,
    bq13: i64,
    bq14: i64,
    bq15: i64,
    bq16: i64,
    bq17: i64,
    bq18: i64,
    bq19: i64,
    bq20: i64,
    bq21: i64,
    bq22: i64,
    bq23: i64,
    bq24: i64,
    bq25: i64,
    bq26: i64,
    bq27: i64,
    bq28: i64,
    bq29: i64,
    bq30: i64,
}

impl<'a> BarrData {
    pub fn to_array(&self) -> [i64; 30] {
        [
            self.bq1, self.bq2, self.bq3, self.bq4, self.bq5, self.bq6, self.bq7, self.bq8,
            self.bq9, self.bq10, self.bq11, self.bq12, self.bq13, self.bq14, self.bq15, self.bq16,
            self.bq17, self.bq18, self.bq19, self.bq20, self.bq21, self.bq22, self.bq23, self.bq24,
            self.bq25, self.bq26, self.bq27, self.bq28, self.bq29, self.bq30,
        ]
    }

    pub fn construct_answers(ans: [i64; 30]) -> Vec<QuestionAnswer<'a>> {
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


#[server(ProcessBarrat)]
pub async fn process_barrat(data: BarrData) -> Result<(), ServerFnError> {
    use crate::app::ssr::*;
    use leptos_axum::*;

    let cookie = crate::extras::get_id_cookie().await?;
    let arr = data.to_array();
    let answers = BarrData::construct_answers(arr);

    let results = ImpulsivityResult::eval(&answers);
    let conn = &mut db().await.unwrap();

    let _ = sqlx::query("INSERT INTO barrat VALUES ($1, $2, $3, $4, $5)")
        .bind(cookie)
        .bind(results.cognitive)
        .bind(results.motor)
        .bind(results.unplanned)
        .bind(arr)
        .execute(conn)
        .await?;

    let response = expect_context::<leptos_axum::ResponseOptions>();
    crate::extras::add_cookie("stage", String::from("card_sorting"), &response).await;
    Ok(())
}
