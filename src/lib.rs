use std::fmt::{Display, Error as FMTError, Formatter};
use std::io::Error as IOError;
use std::error::Error as TraitError;
mod questions;
pub use questions::QuestionInterface;
use questions::QuestionInterface as QuestIntf;

/**
 * the Error struct that we use during the loading 
 * phase of the application
 *
 * it contains msg which happens to a simple string literal
 * and a line No which we use to complete the message
 */
#[derive(Debug)]
pub struct LoadErrs {
    line_no: usize,
    msg: &'static str,
}

impl Display for LoadErrs {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FMTError> {
        write!(f, "Error on Line {}: {}", self.line_no, self.msg)
    }
}

impl TraitError for LoadErrs {
    fn description (&self) -> &str {
        self.msg
    }
}

/// I should at least generate some documenetantio
pub fn is_comment_line(line: &str) -> bool {
    let line = line.trim();
    line.is_empty()
}

fn generate_question(data: (usize, Vec<&str>)) 
                                -> Result<Box<QuestIntf>,LoadErrs> {
    let (line_no, fields) = data;
    let load_err = move |msg| LoadErrs{ line_no,msg};
    //makes error handling less verbose downstream
    gen_ques_inner(fields).map_err(load_err)
                            
}

fn gen_ques_inner(fields : Vec<&str>) -> 
                                    Result<Box<QuestIntf>,&'static str> {
    use questions::{MultipleChoice as MCQ,ShortAnswer as SAQ};

    if fields.len() < 4 {
        Err("not enough fields")
    } else {
        let lower_case_check = fields[0].to_ascii_lowercase();

        Ok(match lower_case_check.as_ref() {
            "sa" | "s" => Box::new(SAQ::new(fields[2], fields[3])),
            "mc" | "m" => Box::new(MCQ::new(&fields[..])?),
           // "tf" | "t" => Box::new(TFQ::new(&fields[..])?),

            _ => return Err("unknown question type"),  
        })
    }
}

/// this loads questions for the application
/// we should really see if this code is correct!
///
/// this is another test
pub fn load_questions(filename: &str) -> Result<Vec<Box<QuestIntf>>, IOError> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = BufReader::new(File::open(filename)?);

    //load file to memory and split it into lines ignoring file IO errors
    let list_lines = file.lines().collect::<Result<Vec<_>, _>>()?;

    // get rid of comments in file 
    let useful_lines = list_lines
        .iter()
        .enumerate() 
        .filter(|&(_, ref line)| !is_comment_line(line));

    // process items based on field information
    let question_process = useful_lines
        .map(|(num, ref line)| (num, line.split('|').collect()))
        .map(generate_question); //line info is used here!

    //force processing and partion based on if we got any errors
    let (ok, errors): (Vec<_>, Vec<_>) = question_process.
                                                    partition(Result::is_ok);

    //sadly partition isnt smart enough to destructure from 
    // result to the items we actually care about.
    // so we have to do it ourselves via unwrap
    let ok_values = ok.into_iter().map(Result::unwrap).collect::<Vec<_>>();

    //print all of the errors out
    for err in errors {
        //see comment on ok_values for unwrap_err
        eprintln!("{}", err.unwrap_err());
    }
    Ok(ok_values)
}

