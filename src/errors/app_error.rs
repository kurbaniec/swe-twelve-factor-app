use std::fmt::{Debug, Display};

pub trait AppError: Display + Debug {
    fn source(&self) -> Option<&(dyn AppError)>;
    fn description(&self) -> String;

    fn get_error_msg(&self) -> String;
    fn get_stacktrace(&self) -> String {
        let mut buffer = String::new();
        inner_stacktrace_processing(self.source(), &mut buffer);
        buffer.push_str(&*self.get_error_msg());
        buffer
    }

    fn print_error(&self) {
        println!("{}", self.get_error_msg());
    }

    fn print_stacktrace(&self) {
        println!("{}", self.get_stacktrace());
    }
}

pub fn inner_stacktrace_processing(error: Option<&(dyn AppError)>, buffer: &mut String) {
    if let Some(inner_error) = error {
        inner_stacktrace_processing(inner_error.source(), buffer);
        buffer.push_str(&*inner_error.get_error_msg());
        buffer.push_str("\n");
    }
}
