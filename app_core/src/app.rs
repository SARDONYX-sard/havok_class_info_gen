use crate::error::{CoreError, Result};

/// Add num1 + num2
///
/// # Error
/// out of range(0..=10)
pub fn add(num1: u64, num2: u64) -> Result<u64> {
    Ok(match num1 {
        0..=10 => match num2 {
            0..=10 => num1 + num2,
            num => return Err(CoreError::UnexpectedNumber(num)),
        },
        num => return Err(CoreError::UnexpectedNumber(num)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_add() {
        assert_eq!(add(1, 1), Ok(2));
    }

    #[test]
    fn should_err() {
        assert!(add(11, 1).is_err());
    }
}
