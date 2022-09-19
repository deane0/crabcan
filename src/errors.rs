use std::{fmt, process::exit};

// Allows to display a variant with the format {:?}
#[derive(Debug)]
// Contains all possible errors in our tool
pub enum ErrorCode {
    ArgumentInvalid(&'static str),
}

#[allow(unreachable_patterns)]
// trait Display, allows ErrorCode enum to be display by:
//     println("{}", error)
// in this case, it calls the function "fmt", which we define the behavior below
impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define what behavior  for ech variant of the enum
        match &self {
            ErrorCode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl ErrorCode {
    // Translate an Error
    pub fn get_return_code(&self) -> i32 {
        1 // Everything != 0 will be treated as an error
    }
}

pub fn exit_with_return_code(res: Result<(), ErrorCode>) {
    match res {
        // If it's a success, return 0
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            exit(0);
        }
        Err(e) => {
            let return_code = e.get_return_code();
            log::error!("Error on exit:\n\t{}\n\tReturning {}", e, return_code);
            exit(return_code);
        }
    }
}
