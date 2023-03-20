use std::fmt::Display;
use std::io::Write;
use std::str::FromStr;

pub fn get_input<T: Display>(message: T) -> Result<String, String> {
    print!("{message}");
    let mut input = String::new();
    std::io::stdout()
        .flush()
        .map_err(|e| format!("failed to write to terminal: {e}"))?;
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("failed to get input of terminal: {e}"))?;

    Ok(input.trim().to_string())
}

pub fn get_confirm<T: Display>(message: T, default: bool) -> Result<Option<bool>, String> {
    let resp = get_input(message)?;
    match resp.to_lowercase().as_str() {
        "" => Ok(Some(default)),
        "y" => Ok(Some(true)),
        "yes" => Ok(Some(true)),
        "n" => Ok(Some(false)),
        "no" => Ok(Some(false)),
        _ => Ok(None),
    }
}

pub fn get_input_with_null<T: Display>(message: T) -> Result<Option<String>, String> {
    let text = get_input(message)?;
    let value = match text.is_empty() {
        true => None,
        false => Some(text),
    };
    Ok(value)
}

pub fn get_input_number<T: Display + Copy, U: Display + Copy, V: FromStr + Display>(
    message: T,
    type_name: U,
) -> Result<(Option<String>, Option<V>), String> {
    if let Some(text) = get_input_with_null(message)? {
        Ok(text
            .parse::<V>()
            .map(|v| (None, Some(v)))
            .unwrap_or_else(|_| {
                (
                    Some(format!("failed to convert value {text} to {type_name}.")),
                    None,
                )
            }))
    } else {
        Ok((None, None))
    }
}
