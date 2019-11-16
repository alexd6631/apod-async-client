/// A simple date representation
pub enum Date {
    Today,
    Date { day: u8, month: u8, year: u16 },
}

impl Date {
    pub(crate) fn as_param(&self) -> Option<String> {
        match self {
            Date::Today => None,
            Date::Date { day, month, year } => Some(format!("{}-{:02}-{:02}", year, month, day)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Date;

    #[test]
    fn test_today_as_param() {
        let d = Date::Today;
        assert_eq!(d.as_param(), None)
    }

    #[test]
    fn test_date() {
        let d = Date::Date {
            day: 9,
            month: 6,
            year: 1986,
        };
        assert_eq!(d.as_param(), Some("1986-06-09".to_owned()))
    }
}
