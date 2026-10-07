//! Evaluation of the numeric expressions that appear in gate arguments.

use std::f64::consts::PI;

/// Evaluate a non-negative integer, such as a qubit index.
///
/// Mirrors the VS Code parser: the text is trimmed and must be all digits.
pub(crate) fn evaluate_integer(text: &str) -> Option<usize> {
    let trimmed = text.trim();

    if trimmed.is_empty() || !trimmed.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }

    trimmed.parse().ok()
}

/// Evaluate a numeric expression, resolving the `pi` constant.
///
/// Mirrors the VS Code parser: only arithmetic, parentheses and `pi` reach the
/// evaluator, everything else is rejected.
pub(crate) fn evaluate_number(text: &str) -> Option<f64> {
    let normalized = text
        .replace("numpy.pi", "PI")
        .replace("np.pi", "PI")
        .replace("math.pi", "PI")
        .replace("pi", "PI");

    if !normalized.chars().all(is_number_character) {
        return None;
    }

    evaluate_expression(&normalized)
}

const fn is_number_character(character: char) -> bool {
    character.is_ascii_digit()
        || character.is_whitespace()
        || matches!(
            character,
            '+' | '-' | '*' | '/' | '(' | ')' | '.' | 'e' | 'E' | 'P' | 'I'
        )
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Number(f64),
    Pi,
    Plus,
    Minus,
    Star,
    StarStar,
    Slash,
    LeftParenthesis,
    RightParenthesis,
}

fn evaluate_expression(expression: &str) -> Option<f64> {
    let tokens = tokenize(expression)?;
    let mut parser = ExpressionParser {
        tokens,
        position: 0,
    };
    let value = parser.parse_additive()?;

    (parser.position == parser.tokens.len()).then_some(value)
}

fn tokenize(expression: &str) -> Option<Vec<Token>> {
    let characters: Vec<char> = expression.chars().collect();
    let mut tokens = Vec::new();
    let mut position = 0;

    while position < characters.len() {
        let character = characters[position];

        if character.is_whitespace() {
            position += 1;
            continue;
        }

        if character.is_ascii_digit() || character == '.' {
            let number = parse_number(&characters, &mut position)?;
            tokens.push(Token::Number(number));
            continue;
        }

        if character == 'P' && characters.get(position + 1) == Some(&'I') {
            tokens.push(Token::Pi);
            position += 2;
            continue;
        }

        let token = match character {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' if characters.get(position + 1) == Some(&'*') => {
                position += 1;
                Token::StarStar
            }
            '*' => Token::Star,
            '/' => Token::Slash,
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            _ => return None,
        };

        tokens.push(token);
        position += 1;
    }

    Some(tokens)
}

fn parse_number(characters: &[char], position: &mut usize) -> Option<f64> {
    let start = *position;
    let mut has_digits = false;

    while *position < characters.len() && characters[*position].is_ascii_digit() {
        *position += 1;
        has_digits = true;
    }

    if *position < characters.len() && characters[*position] == '.' {
        *position += 1;

        while *position < characters.len() && characters[*position].is_ascii_digit() {
            *position += 1;
            has_digits = true;
        }
    }

    if *position < characters.len() && matches!(characters[*position], 'e' | 'E') {
        *position += 1;

        if *position < characters.len() && matches!(characters[*position], '+' | '-') {
            *position += 1;
        }

        let exponent_start = *position;
        while *position < characters.len() && characters[*position].is_ascii_digit() {
            *position += 1;
        }

        if *position == exponent_start {
            return None;
        }
    }

    if !has_digits {
        return None;
    }

    characters[start..*position]
        .iter()
        .collect::<String>()
        .parse()
        .ok()
}

struct ExpressionParser {
    tokens: Vec<Token>,
    position: usize,
}

impl ExpressionParser {
    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn parse_additive(&mut self) -> Option<f64> {
        let mut value = self.parse_multiplicative()?;

        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.advance();
                    value += self.parse_multiplicative()?;
                }
                Some(Token::Minus) => {
                    self.advance();
                    value -= self.parse_multiplicative()?;
                }
                _ => break,
            }
        }

        Some(value)
    }

    fn parse_multiplicative(&mut self) -> Option<f64> {
        let mut value = self.parse_power()?;

        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.advance();
                    value *= self.parse_power()?;
                }
                Some(Token::Slash) => {
                    self.advance();
                    value /= self.parse_power()?;
                }
                _ => break,
            }
        }

        Some(value)
    }

    fn parse_power(&mut self) -> Option<f64> {
        let base = self.parse_unary()?;

        if self.peek() == Some(Token::StarStar) {
            self.advance();
            let exponent = self.parse_power()?;
            return Some(base.powf(exponent));
        }

        Some(base)
    }

    fn parse_unary(&mut self) -> Option<f64> {
        match self.peek() {
            Some(Token::Plus) => {
                self.advance();
                self.parse_unary()
            }
            Some(Token::Minus) => {
                self.advance();
                self.parse_unary().map(|value| -value)
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Option<f64> {
        match self.peek() {
            Some(Token::Number(number)) => {
                self.advance();
                Some(number)
            }
            Some(Token::Pi) => {
                self.advance();
                Some(PI)
            }
            Some(Token::LeftParenthesis) => {
                self.advance();
                let value = self.parse_additive()?;

                if self.peek() == Some(Token::RightParenthesis) {
                    self.advance();
                    Some(value)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_integer, evaluate_number};

    #[test]
    fn parses_integers() {
        assert_eq!(evaluate_integer("0"), Some(0));
        assert_eq!(evaluate_integer(" 42 "), Some(42));
        assert_eq!(evaluate_integer(""), None);
        assert_eq!(evaluate_integer("3.0"), None);
        assert_eq!(evaluate_integer("n"), None);
    }

    #[test]
    fn resolves_pi_constants() {
        let pi = std::f64::consts::PI;

        assert!((evaluate_number("numpy.pi / 4").unwrap() - pi / 4.0).abs() < 1e-12);
        assert!((evaluate_number("2 * math.pi / 3").unwrap() - 2.0 * pi / 3.0).abs() < 1e-12);
        assert!((evaluate_number("-np.pi / 2").unwrap() + pi / 2.0).abs() < 1e-12);
        assert!((evaluate_number("pi").unwrap() - pi).abs() < 1e-12);
    }

    #[test]
    fn evaluates_arithmetic() {
        assert!((evaluate_number("1 / 2").unwrap() - 0.5).abs() < 1e-12);
        assert!((evaluate_number("2**3").unwrap() - 8.0).abs() < 1e-12);
        assert!((evaluate_number("1e-3").unwrap() - 0.001).abs() < 1e-15);
        assert!((evaluate_number("2 * (3 + 4)").unwrap() - 14.0).abs() < 1e-12);
    }

    #[test]
    fn rejects_unresolvable_expressions() {
        assert_eq!(evaluate_number("x"), None);
        assert_eq!(evaluate_number("circuit.q(0)"), None);
        assert_eq!(evaluate_number("2e"), None);
        assert_eq!(evaluate_number("1 +"), None);
    }
}
