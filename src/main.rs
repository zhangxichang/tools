mod run;
mod viewport;

use eyre::Result;

use crate::run::run;

fn main() -> Result<()> {
    run("张喜昌的工具站")
}
