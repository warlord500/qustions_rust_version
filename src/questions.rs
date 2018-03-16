use std::fmt::Debug;

/**
 * currently the highlevel interface for question instances
 *
 */
pub trait QuestionInterface: Debug {
    /// check if answer given is correct.
    ///
    /// this code returns a result for any conversions
    /// that might occur during the checking process.
    /// 
    /// this makes handling TF and MultipleChoice questions easier
    /// as well pontential future Question classes. 
    fn check_answer(&self, &str) -> Result<bool,&'static str>;
    fn question_text(&self) -> String;
    fn answer_text(&self) -> String;
}

/// this struct stores information for
/// ShortAnswer Questions
#[derive(Debug)]
pub struct ShortAnswer {
    question: String,
    answer: String,
}

#[derive(Debug)]
pub struct MultipleChoice {
    question: String,
    choices: Vec<String>,
    answer: u8,
}

//---------------------------------------------------------------------
// code for short answer type
//---------------------------------------------------------------------
impl ShortAnswer {
    pub fn new(question: &str, answer: &str) -> Self {
        ShortAnswer {
            question: String::from(question.trim()),
            answer: String::from(answer.trim()),
        }
    }
}

//---------------------------------------------------------------------
impl QuestionInterface for ShortAnswer {
    fn check_answer(&self, answer: &str) -> Result<bool,&'static str> {
        Ok(self.question == answer)
    }
    fn question_text(&self) -> String {
        self.question.clone()
    }
    fn answer_text(&self) -> String {
        self.answer.clone()
    }
}

//---------------------------------------------------------------------
// code for multiple choice  answer type
//---------------------------------------------------------------------

impl MultipleChoice {
    pub fn new(fields: &[&str]) -> Result<Self, &'static str> {
        if fields.len() != 5 {
            Err(" incorrect number of fields for multiple choice ")
        } else {
            let answer = convert_choice_to_number(fields[4])?;
            let choices = fields[3].split('|')
                                    .map(String::from)
                                    .collect::<Vec<_>>();

            //wow i dont know how else to write this
            //if else chain to be shorter
            if choices.len() < 3 {
                Err("not enough choices")
            } else if choices.len() >= 7 {
                Err("too many choices")
            } else if (answer as usize) > choices.len() {
                Err("answer must be a choice")
            } else {
                Ok(MultipleChoice {
                    question: String::from(fields[0]),
                    choices,
                    answer,
                })
            }
        } //outside if
    }
}

impl QuestionInterface for MultipleChoice {
    fn check_answer(&self,answer : &str) -> Result<bool, &'static str> {
        unimplemented!()
    }

    fn question_text(&self) -> String {
        unimplemented!()
    }

    fn answer_text(&self) -> String {
        unimplemented!()
    }
}

///converts choices to numbers for testing purposes!
///easier to test against and this function holds all possible errors
///that can happen during that conversion!
fn convert_choice_to_number(choice: &str) -> Result<u8, &'static str> {
    let choice: &str = choice.trim_right();

    if choice.len() > 1 {
        Err("choice is too long")
    } else if choice.len() == 0 {
        Err("must have a choice")
    } else {
        //get first char, this code should not panic!
        //but sadly i have no way to make that so 
        //via types
        let choice_char = choice.chars().next().unwrap();

        if !choice_char.is_ascii_alphabetic() {
            Err("the correct choice must be a letter")
        } else if (choice_char as u8) > ('g' as u8) {
            Err("choice must be between a-g")
        } else {
            Err("fail")
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_convert_choice_to_number() {
        let conv = convert_choice_to_number;
        assert!(conv("a"), 1);

    }
}
