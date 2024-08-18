pub struct Question {
    inverse: bool,
    pub kind: ImpulsivityType,
}

pub enum ImpulsivityType {
    Cognitive,
    Motor,
    Unplanned,
}

pub struct QuestionAnswer<'a> {
    pub q: &'a Question,
    pub answer: i64,
}

impl<'a> QuestionAnswer<'a> {
    pub fn get_answer(&self) -> i32 {
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

pub static QUESTIONS: [Question; 30] = [
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


