use std::fmt;

#[derive(Debug)]
pub struct CalcError {
    err_type: ErrorCategory,
    err_message: String,
    err_cause: Option<char>,
}

impl CalcError {
    pub fn new(
        err_type: ErrorCategory,
        err_message: &str,
        err_cause: Option<char>,
    ) -> CalcError {
        let err_message = String::from(err_message);

        CalcError {
            err_type,
            err_message,
            err_cause,
        }
    }
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.err_cause {
            Some(cause) => write!(f, "{} with cause {}", self.err_message, cause),
            None => write!(f, "{}", self.err_message),
        }
    }
}

// impl fmt::Debug for CalcError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("CalcError")
//             .field("err_type", &self.err_type)
//             .field("err_message", &self.err_message)
//             .field("err_cause", &self.err_cause)
//             .finish()
//     }
// }

#[derive(Debug)]
pub enum ErrorCategory {
    UnkownOperatorError,
    SyntaxError,
}
