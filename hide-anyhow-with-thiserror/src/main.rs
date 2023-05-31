fn main() {
    let err: Result<(), MyError> = stack1();
    eprintln!("{err:?}");
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct MyError(anyhow::Error);

fn stack1() -> Result<(), MyError> {
    stack2()?;
    Ok(())
}

fn stack2() -> Result<(), MyError> {
    parse_int()?;
    Ok(())
}

fn parse_int() -> Result<i32, MyError> {
    "invalid"
        .parse::<i32>()
        .map_err(|e| e.into()) // ParseIntErrorをanyhow::Errorに変換する。このタイミングでバックトレースが付与される。
        .map_err(MyError) // anyhow::ErrorをMyErrorに変換する
}
