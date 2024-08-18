use leptos::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{FromRow, Row};

#[cfg(feature = "ssr")]
static CODE: &'static str = "7721";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParticipantForm {
    pub age: i32,
    pub sex: i32,
    pub major: String,
    pub alcohol: i32,
    pub alcohol_freq: Option<i32>,
    pub drugs: i32,
    pub drugs_freq: Option<i32>,
    pub disorder: i32,
    pub disorder_input: Option<String>,
    pub injury: i32,
    pub injury_location: Option<String>,
    pub injury_treated: Option<i32>,
    pub loss: i32,
}

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Participant {
    pub id: i64,
    pub age: i32,
    pub sex: i32,
    pub major: String,
    pub alcohol: bool,
    pub alcohol_frequency: Option<i32>,
    pub drugs: bool,
    pub drugs_frequency: Option<i32>,
    pub disorder: Option<String>,
    pub injury: bool,
    pub injury_treated: Option<bool>,
    pub injury_location: Option<String>,
    pub abuse: i32,
    pub abuse_other: Option<String>,
    pub shortage: i32,
    pub loss: i32,
}

#[cfg(feature = "ssr")]
impl Participant {
    pub fn from_form(
        id: i64,
        form: ParticipantForm,
        abuse: i32,
        shortage: i32,
        abuse_other: Option<String>,
    ) -> Self {
        let injury_treated = match form.injury_treated {
            Some(x) => Some(x == 1),
            None => None,
        };

        Self {
            id,
            age: form.age,
            sex: form.sex,
            major: form.major,
            alcohol: form.alcohol == 1,
            alcohol_frequency: form.alcohol_freq,
            drugs: form.drugs == 1,
            drugs_frequency: form.drugs_freq,
            disorder: form.disorder_input,
            injury: form.injury == 1,
            injury_location: form.injury_location,
            injury_treated,
            abuse,
            abuse_other,
            shortage,
            loss: form.loss,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Abuse {
    pub psychological: Option<i32>,
    pub physical: Option<i32>,
    pub sexual: Option<i32>,
    pub bullying: Option<i32>,
    pub finantial: Option<i32>,
    pub _none: Option<i32>,
    pub other: Option<String>,
}

impl Abuse {
    pub fn get_value(&self) -> i32 {
        let mut result = 0;
        result |= self.psychological.unwrap_or(0);
        result |= self.physical.unwrap_or(0);
        result |= self.psychological.unwrap_or(0);
        result |= self.psychological.unwrap_or(0);
        result |= self.psychological.unwrap_or(0);
        result += if self.other.is_some() { 32 } else { 0 };

        result
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Shortage {
    pub economic: Option<i32>,
    pub social: Option<i32>,
    pub emotional: Option<i32>,
    pub _none: Option<i32>,
}

impl Shortage {
    pub fn get_value(&self) -> i32 {
        let mut result = 0;
        result |= self.economic.unwrap_or(0);
        result |= self.social.unwrap_or(0);
        result |= self.emotional.unwrap_or(0);

        result
    }
}

#[server(ProcessQuestions)]
pub async fn process_questions(
    participant: ParticipantForm,
    abuse: Option<Abuse>,
    shortage: Option<Shortage>,
    code: String,
) -> Result<(), ServerFnError> {
    use crate::app::ssr::*;
    use http::{header, HeaderValue};
    println!("AAA");
    if let Err(_) = verify_code(code).await {
        return Err(ServerFnError::new("Bad code"));
    }

    let response = expect_context::<leptos_axum::ResponseOptions>();
    let conn = &mut db().await.unwrap();
    let mut id: i64 = sqlx::query("SELECT COUNT(*) from participantes;")
        .fetch_one(conn)
        .await
        .map(|row| row.get(0))
        .unwrap_or(0);

    let abuse_value = match &abuse {
        None => 0,
        Some(a) => a.get_value(),
    };

    let shortage_value = match &shortage {
        None => 0,
        Some(a) => a.get_value(),
    };

    let abuse_other = match &abuse {
        None => None,
        Some(a) => a.other.clone(),
    };

    let participant =
        Participant::from_form(id, participant, abuse_value, shortage_value, abuse_other);

    loop {
        let conn = &mut db().await.unwrap();
        id = sqlx::query("SELECT COUNT(*) from participantes;")
            .fetch_one(conn)
            .await
            .map(|row| row.get(0))
            .unwrap_or(0);

        let conn = &mut db().await.unwrap();

        if let Ok(_) = sqlx::query(
            "INSERT INTO participantes VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9,
                    $10, $11, $12, $13, $14, $15, $16)",
        )
        .bind(id)
        .bind(participant.age)
        .bind(participant.sex)
        .bind(&participant.major)
        .bind(participant.alcohol)
        .bind(participant.alcohol_frequency)
        .bind(participant.drugs)
        .bind(participant.drugs_frequency)
        .bind(&participant.disorder)
        .bind(participant.injury)
        .bind(participant.injury_treated)
        .bind(&participant.injury_location)
        .bind(participant.abuse)
        .bind(&participant.abuse_other)
        .bind(participant.shortage)
        .bind(participant.loss)
        .execute(conn)
        .await
        {
            break;
        }
    }

    let id_cookie = cookie::Cookie::new("p_id", id.to_string());
    response.insert_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&id_cookie.to_string()).unwrap(),
    );

    Ok(())
}

#[server(VerifyCode)]
pub async fn verify_code(code: String) -> Result<(), ServerFnError> {
    if code == CODE {
        println!("Code OK :)");
        return Ok(());
    } else {
        println!("Code Not OK :(");
        return Err(ServerFnError::new(""));
    }
}
