use std::{error::Error, fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Modifiers {
    pub super_key: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Char(char),
    Enter,
    Equal,
    Minus,
    Left,
    Right,
    Up,
    Down,
    Zero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub modifiers: Modifiers,
    pub key: Key,
}

impl FromStr for KeyBinding {
    type Err = KeyBindingParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_keybinding(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyBindingParseError {
    Empty,
    UnknownKey(String),
    MultipleKeys,
    MissingKey,
    DuplicateBinding {
        first: &'static str,
        second: &'static str,
    },
}

impl fmt::Display for KeyBindingParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(formatter, "keybinding is empty"),
            Self::UnknownKey(key) => write!(formatter, "unknown key: {key}"),
            Self::MultipleKeys => write!(formatter, "keybinding contains multiple keys"),
            Self::MissingKey => write!(formatter, "keybinding is missing a key"),
            Self::DuplicateBinding { first, second } => {
                write!(formatter, "duplicate keybinding for {first} and {second}")
            }
        }
    }
}

impl Error for KeyBindingParseError {}

fn parse_keybinding(input: &str) -> Result<KeyBinding, KeyBindingParseError> {
    if input.trim().is_empty() {
        return Err(KeyBindingParseError::Empty);
    }

    let mut modifiers = Modifiers::default();
    let mut key = None;

    for token in input.split('+') {
        let token = token.trim();
        if token.is_empty() {
            return Err(KeyBindingParseError::UnknownKey(token.to_string()));
        }

        match token.to_ascii_lowercase().as_str() {
            "super" => modifiers.super_key = true,
            "ctrl" | "control" => modifiers.ctrl = true,
            "alt" => modifiers.alt = true,
            "shift" => modifiers.shift = true,
            _ => {
                if key.is_some() {
                    return Err(KeyBindingParseError::MultipleKeys);
                }

                key = Some(parse_key(token)?);
            }
        }
    }

    let Some(key) = key else {
        return Err(KeyBindingParseError::MissingKey);
    };

    Ok(KeyBinding { modifiers, key })
}

fn parse_key(token: &str) -> Result<Key, KeyBindingParseError> {
    match token.to_ascii_lowercase().as_str() {
        "enter" => Ok(Key::Enter),
        "equal" => Ok(Key::Equal),
        "minus" => Ok(Key::Minus),
        "left" => Ok(Key::Left),
        "right" => Ok(Key::Right),
        "up" => Ok(Key::Up),
        "down" => Ok(Key::Down),
        "0" => Ok(Key::Zero),
        _ => parse_char_key(token),
    }
}

fn parse_char_key(token: &str) -> Result<Key, KeyBindingParseError> {
    let mut chars = token.chars();
    let Some(character) = chars.next() else {
        return Err(KeyBindingParseError::UnknownKey(token.to_string()));
    };

    if chars.next().is_some() {
        return Err(KeyBindingParseError::UnknownKey(token.to_string()));
    }

    if character.is_ascii_alphanumeric() {
        Ok(Key::Char(character.to_ascii_uppercase()))
    } else {
        Err(KeyBindingParseError::UnknownKey(token.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{Key, KeyBinding, KeyBindingParseError};

    #[test]
    fn parses_super_shift_q() {
        let binding = KeyBinding::from_str("Super+Shift+Q").unwrap();

        assert!(binding.modifiers.super_key);
        assert!(binding.modifiers.shift);
        assert_eq!(binding.key, Key::Char('Q'));
    }

    #[test]
    fn parses_super_enter() {
        let binding = KeyBinding::from_str("Super+Enter").unwrap();

        assert!(binding.modifiers.super_key);
        assert_eq!(binding.key, Key::Enter);
    }

    #[test]
    fn parses_super_zero() {
        let binding = KeyBinding::from_str("Super+0").unwrap();

        assert_eq!(binding.key, Key::Zero);
    }

    #[test]
    fn parses_super_ctrl_left() {
        let binding = KeyBinding::from_str("Super+Ctrl+Left").unwrap();

        assert!(binding.modifiers.super_key);
        assert!(binding.modifiers.ctrl);
        assert_eq!(binding.key, Key::Left);
    }

    #[test]
    fn rejects_empty_strings() {
        assert_eq!(
            KeyBinding::from_str("").unwrap_err(),
            KeyBindingParseError::Empty
        );
    }

    #[test]
    fn rejects_unknown_keys() {
        assert_eq!(
            KeyBinding::from_str("Super+PageDown").unwrap_err(),
            KeyBindingParseError::UnknownKey("PageDown".to_string())
        );
    }

    #[test]
    fn rejects_multiple_keys() {
        assert_eq!(
            KeyBinding::from_str("Super+Q+W").unwrap_err(),
            KeyBindingParseError::MultipleKeys
        );
    }
}
