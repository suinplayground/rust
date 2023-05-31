fn main() {
    let err: Result<(), anyhow::Error> = stack1();
    eprintln!("{err:?}");
}

fn stack1() -> Result<(), anyhow::Error> {
    stack2()?;
    Ok(())
}

fn stack2() -> Result<(), anyhow::Error> {
    parse_int()?;
    Ok(())
}

fn parse_int() -> Result<i32, anyhow::Error> {
    // 1. anyhow::ErrorをErrの型に指定する
    // 2. into()でanyhow::Errorに変換する
    // この両方を行うことでParseIntErrorに、
    "invalid".parse::<i32>().map_err(|e| e.into())
    //                                               ^^^^^^^^ ここのスタックがバックトレースに含まれるようになる
}
