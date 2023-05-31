use error_stack::{IntoReport, Result, ResultExt};

fn main() {
    stack1().unwrap();
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct Error1(String);

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct Error2(String);

fn stack1() -> Result<(), Error2> {
    stack2().map_err(|e| Error2(e.to_string())).into_report()?;
    Ok(())
}

fn stack2() -> Result<(), Error2> {
    parse_int().change_context_lazy(|| Error2("failed to call parse_int".to_string()))?;
    Ok(())
}

fn parse_int() -> Result<i32, Error1> {
    "invalid"
        .parse::<i32>()
        .into_report()
        .map_err(|e| Error1(format!("{e}")))
        .into_report()
}
