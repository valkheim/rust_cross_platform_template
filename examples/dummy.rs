use std::process::ExitCode;

use dummies::dummy;

fn main() -> ExitCode {
    ExitCode::from(dummy::dummy())
}
