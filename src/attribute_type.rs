use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeType {
    String,
    StringSet,
    Number,
    NumberSet,
    Binary,
    BinarySet,
    Boolean,
    Null,
    List,
    Map,
}

impl fmt::Display for AttributeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::String => "S",
            Self::StringSet => "SS",
            Self::Number => "N",
            Self::NumberSet => "NS",
            Self::Binary => "B",
            Self::BinarySet => "BS",
            Self::Boolean => "BOOL",
            Self::Null => "NULL",
            Self::List => "L",
            Self::Map => "M",
        })
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use super::AttributeType::*;

    #[test]
    fn display_attribute_type() {
        assert_str_eq!("S", String.to_string());
        assert_str_eq!("SS", StringSet.to_string());
        assert_str_eq!("N", Number.to_string());
        assert_str_eq!("NS", NumberSet.to_string());
        assert_str_eq!("B", Binary.to_string());
        assert_str_eq!("BS", BinarySet.to_string());
        assert_str_eq!("BOOL", Boolean.to_string());
        assert_str_eq!("NULL", Null.to_string());
        assert_str_eq!("L", List.to_string());
        assert_str_eq!("M", Map.to_string());
    }
}
