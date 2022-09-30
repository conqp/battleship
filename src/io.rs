use std::fmt::Debug;
use std::io::Write;
use std::str::FromStr;

pub fn read<T>(prompt: impl Into<String>) -> Result<T, &'static str>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    print!("{}", prompt.into());
    _ = std::io::stdout().flush();
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim().parse::<T>() {
            Ok(value) => Ok(value),
            Err(_) => Err("invalid value"),
        },
        Err(_) => Err("no value read"),
    }
}

pub fn read_repeat<T>(prompt: impl Into<String> + Copy) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    loop {
        match read::<T>(prompt) {
            Err(msg) => eprintln!("Error: {}", msg),
            Ok(value) => return value,
        }
    }
}
