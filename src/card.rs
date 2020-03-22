pub trait Card: std::fmt::Display + std::convert::From<u8> {}

pub enum Royality {
    Jack,
    Queen,
    King,
}

pub enum Value {
    Numeric(u8),
    Royal(Royality),
    Ace,
    Joker,
}

pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

pub struct PlayingCard {
    value: Value,
    suit: Suit,
}

impl std::fmt::Display for Royality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Royality::Jack => "J",
                Royality::Queen => "Q",
                Royality::King => "K",
            }
        )
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Numeric(val) => format!("{}", val),
                Value::Royal(royality) => format!("{}", royality),
                Value::Ace => "A".to_string(),
                Value::Joker => "%".to_string(),
            }
        )
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Hearts => "H",
                Suit::Spades => "S",
                Suit::Diamonds => "D",
                Suit::Clubs => "C",
            }
        )
    }
}

impl std::fmt::Display for PlayingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.value)
    }
}

impl std::convert::From<u8> for Suit {
    fn from(value: u8) -> Suit {
        match value {
            0 => Suit::Clubs,
            1 => Suit::Diamonds,
            2 => Suit::Spades,
            _ => Suit::Hearts
        }
    }
}

impl std::convert::From<u8> for Value {
    fn from(value: u8) -> Value {
        match value {
            0..=8 => Value::Numeric(value + 2),
            9 => Value::Royal(Royality::Jack),
            10 => Value::Royal(Royality::Queen),
            11 => Value::Royal(Royality::King),
            12 => Value::Ace,
            _ => Value::Joker
        }
    }
}

impl std::convert::From<u8> for PlayingCard {
    fn from(value: u8) -> PlayingCard {
        PlayingCard {
            suit: Suit::from(value % 4),
            value: Value::from(value / 4),
        }
    }
}
