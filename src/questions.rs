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
            id: -1,
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
    use crate::extras::add_cookie;
    use http::{header, HeaderValue};
    use axum::extract::ConnectInfo;
    use leptos_axum::*;
    use std::net::SocketAddr;

    if let Err(_) = verify_code(code).await {
        return Err(ServerFnError::new("Bad code"));
    }

    let ConnectInfo(addr) = extract::<ConnectInfo<SocketAddr>>().await?;

    let response = expect_context::<leptos_axum::ResponseOptions>();
    let conn = &mut db().await.unwrap();

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
        Participant::from_form(participant, abuse_value, shortage_value, abuse_other);

    let conn = &mut db().await.unwrap();

    let id = sqlx::query(
        "INSERT INTO participantes (age, sex, major, alcohol, alcohol_frequency,
            drugs, drugs_frequency, disorder, injury, injury_treated, injury_location,
            abuse, abuse_other, shortage, loss, ip_addr)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9,
            $10, $11, $12, $13, $14, $15, $16) RETURNING id",
    )
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
    .bind(addr.ip().to_string())
    .fetch_one(conn)
    .await?;


    add_cookie("p_id", id.get::<i32, usize>(0).to_string(), &response).await;
    add_cookie("stage", String::from("barrat"), &response).await;

    Ok(())
}

#[server(VerifyCode)]
pub async fn verify_code(code: String) -> Result<(), ServerFnError> {
    if code == CODE {
        return Ok(());
    } else {
        return Err(ServerFnError::new(""));
    }
}
