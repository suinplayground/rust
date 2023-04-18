fn main() {
    println!("== anyhow ==");
    with_anyhow::example();
    println!("== snafu ==");
    with_snafu::example();
}

/// This example uses the `anyhow` crate to add backtraces to custom errors.
mod with_anyhow {
    use anyhow::{ensure, Result};
    use thiserror::Error;

    pub fn example() {
        let err = is_valid_id(1).err().unwrap();
        // Note: `{:?}` prints the error and its backtrace. To see the
        //       backtrace, run with `RUST_BACKTRACE=1`,
        //       e.g. `RUST_BACKTRACE=1 cargo run`
        println!("{:?}", err);

        // Note: We can use `downcast_ref` to get the original error struct,
        //       and we can handle each error case. However this approach is
        //       little bit obscure, because we can't know the actual type of
        //       the error from the function signature.
        match err.downcast_ref::<CustomError>() {
            Some(CustomError::MustBeLessThanTen(id)) => {
                println!("You gave me an ID that was too small: {}", id);
            }
            None => {
                println!("Unknown error");
            }
        }
    }

    fn is_valid_id(id: u16) -> Result<()> {
        // Note: `ensure!` is a macro that returns `Err` with the given error
        //       if the condition is false.
        ensure!(id >= 10, CustomError::MustBeLessThanTen(id));
        Ok(())
    }

    #[derive(Error, Debug)]
    enum CustomError {
        #[error("ID may not be less than 10, but it was {0}")]
        MustBeLessThanTen(u16),
    }
}

mod with_snafu {
    use snafu::prelude::*;
    use snafu::Backtrace;

    pub fn example() {
        let err = is_valid_id(1).err().unwrap();
        // Note: `{:?}` prints the error and its backtrace.
        println!("{:#?}", err);
    }

    fn is_valid_id(id: u16) -> Result<(), CustomError> {
        // Note: `ensure!` is a macro that returns `Err` with the given error
        //       if the condition is false.
        // Note: we have to use `Snafu`-prefixed struct instead of the original
        //       struct `MustBeLessThanTen` here.
        ensure!(id >= 10, MustBeLessThanTenSnafu { id });
        Ok(())
    }

    #[derive(Debug, Snafu)]
    enum CustomError {
        #[snafu(display("ID may not be less than 10, but it was {id}"))]
        MustBeLessThanTen { id: u16, backtrace: Backtrace },
    }
}
