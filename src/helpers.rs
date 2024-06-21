use anyhow::Result;

mod add;
mod show;
mod sign;

pub fn show() -> Result<()> {
    return show::show();
}

pub fn sign() -> Result<()> {
    return sign::sign();
}

pub fn add() -> Result<()> {
    return add::add();
}
