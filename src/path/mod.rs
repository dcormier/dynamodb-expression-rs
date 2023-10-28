mod element;
mod name;

pub use self::{
    element::{Element, IndexedField, Indexes},
    name::{name, Name},
};

use core::{
    fmt::{self, Write},
    str::FromStr,
};

use itertools::Itertools;

use crate::{
    condition::{
        attribute_type::Type, AttributeType, BeginsWith, Between, Comparison, Condition, Contains,
        In,
    },
    operand::{Operand, Size},
    update::{
        add::AddValue,
        set::{
            if_not_exists::Builder as IfNotExistsBuilder,
            list_append::Builder as ListAppendBuilder, math::Builder as MathBuilder,
        },
        Add, Assign, Delete, IfNotExists, ListAppend, Math, Remove,
    },
    value::{self, Scalar, Value},
    Comparator,
};

/// Represents a DynamoDB [document path][1]. For example, `foo[3][7].bar[2].baz`.
///
/// Anything that can be turned into a `Name` can be turned into a [`Path`]
///
/// When used in an [`Expression`], attribute names in a `Path` are
/// automatically handled as [expression attribute names][2], allowing for names
/// that would not otherwise be permitted by DynamoDB. For example,
/// `foo[3][7].bar[2].baz` would become something similar to `#0[3][7].#1[2].#2`,
/// and the names would be in the `expression_attribute_names`.
///
/// See also: [`Name`]
///
/// # Examples
///
/// Each of these are ways to create a `Path` instance for `foo[3][7].bar[2].baz`.
/// ```
/// use dynamodb_expression::{path::{Element, Path}};
/// # use pretty_assertions::assert_eq;
/// #
/// # let expected: Path = [
/// #     Element::indexed_field("foo", [3, 7]),
/// #     Element::indexed_field("bar", 2),
/// #     Element::name("baz"),
/// # ]
/// # .into_iter()
/// # .collect();
///
/// // A `Path` can be parsed from a string
/// let path: Path = "foo[3][7].bar[2].baz".parse().unwrap();
/// # assert_eq!(expected, path);
///
/// // `Path` implements `FromIterator` for items that are `Into<Element>`.
/// let path = Path::from_iter([("foo", vec![3, 7]), ("bar", vec![2]), ("baz", vec![])]);
/// # assert_eq!(expected, path);
///
/// // Of course, that means you can `.collect()` into a `Path`.
/// let path: Path = [("foo", vec![3, 7]), ("bar", vec![2]), ("baz", vec![])]
///     .into_iter()
///     .collect();
/// # assert_eq!(expected, path);
///
/// // `Element` can be converted into from `Into<String>`, as well as
/// // string/index tuples. In this case, an "index" is an array, slice,
/// // `Vec` of, or a single `u32`.
/// let path: Path = [
///     Element::from(("foo", [3, 7])),
///     Element::from(("bar", 2)),
///     Element::from("baz"),
/// ]
/// .into_iter()
/// .collect();
/// # assert_eq!(expected, path);
///
/// // It's smart about it, though. If if there's one or zero indexes it'll do
/// // the right thing. This helps when you're chaining iterator adapters and
/// // the results are values with inconsistent numbers of indexes.
/// let path: Path = [
///     Element::from(("foo", [3, 7])),
///     Element::from(("bar", [2])),
///     Element::from(("baz", [])),
/// ]
/// .into_iter()
/// .collect();
/// # assert_eq!(expected, path);
///
/// // You can also explicitly construct `Element`s.
/// let path: Path = [
///     Element::indexed_field("foo", [3, 7]),
///     Element::indexed_field("bar", 2),
///     Element::name("baz"),
/// ]
/// .into_iter()
/// .collect();
/// # assert_eq!(expected, path);
/// ```
///
/// If you have a document path where an [attribute name includes a period][3]
/// (`.`), you will need to explicitly create the [`Element`]s for that `Path`.
/// ```
/// # use dynamodb_expression::path::{Element, Path};
/// # use pretty_assertions::assert_eq;
/// let path: Path = Element::name("foo.bar").into();
/// # assert_eq!(Path::from_iter([Element::name("foo.bar")]), path);
/// let path = Path::from_iter([Element::indexed_field("foo", 3), Element::name("bar.baz")]);
/// # assert_eq!("foo[3].bar.baz", path.to_string());
/// ```
///
/// `Name` and `Path` can be converted between each other.
/// ```
/// use dynamodb_expression::path::{Element, Name, Path};
///
/// // A `Name` can be converted into a `Path`
/// let name = Name::from("foo");
/// let path = Path::from(name);
/// assert_eq!(Path::from(Element::name("foo")), path);
///
/// // A `Path` consisting of a single, unindexed field can be converted into a `Name`.
/// let path: Path = "foo".parse().unwrap();
/// let name = Name::try_from(path).unwrap();
/// assert_eq!(Name::from("foo"), name);
///
/// // If the `Path` has more elements, or has indexes, it cannot be converted
/// // and the original `Path` is returned.
/// let path: Path = "foo[0]".parse().unwrap();
/// let err = Name::try_from(path.clone()).unwrap_err();
/// assert_eq!(path, err);
///
/// let path: Path = "foo.bar".parse().unwrap();
/// let err = Name::try_from(path.clone()).unwrap_err();
/// assert_eq!(path, err);
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.NestedElements.DocumentPathExamples
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html
/// [3]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.TopLevelAttributes
/// [`Expression`]: crate::expression::Expression
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path {
    pub elements: Vec<Element>,
}

/// Methods relating to building condition and filter expressions.
impl Path {
    /// Compare two values.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    // TODO: Operator-specific methods instead of this.
    pub fn comparison<R>(self, cmp: Comparator, right: R) -> Condition
    where
        R: Into<Operand>,
    {
        Condition::Comparison(Comparison {
            left: self.into(),
            cmp,
            right: right.into(),
        })
    }

    /// `self BETWEEN b AND c` - true if `self` is greater than or equal to `b`, and less than or equal to `c`.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn between<L, U>(self, lower: L, upper: U) -> Condition
    where
        L: Into<Operand>,
        U: Into<Operand>,
    {
        Condition::Between(Between {
            op: self.into(),
            lower: lower.into(),
            upper: upper.into(),
        })
    }

    /// `self IN (b[, ..])` — true if `self` is equal to any value in the list.
    ///
    /// The list can contain up to 100 values. It must have at least 1.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn in_<I, T>(self, items: I) -> Condition
    where
        I: IntoIterator<Item = T>,
        T: Into<Operand>,
    {
        In::new(self.into(), items).into()
    }

    /// True if the item contains the attribute specified by `path`.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn attribute_exists(self) -> Condition {
        Condition::AttributeExists(self.into())
    }

    /// True if the attribute specified by `path` does not exist in the item.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn attribute_not_exists(self) -> Condition {
        Condition::AttributeNotExists(self.into())
    }

    /// True if the attribute at the specified `path` is of a particular data type.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn attribute_type(self, attribute_type: Type) -> Condition {
        AttributeType::new(self, attribute_type).into()
    }

    /// True if the attribute specified by `path` begins with a particular substring.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn begins_with<T>(self, prefix: T) -> Condition
    where
        T: Into<String>,
    {
        BeginsWith::new(self, prefix).into()
    }

    /// True if the attribute specified by `path` is one of the following:
    /// * A `String` that contains a particular substring.
    /// * A `Set` that contains a particular element within the set.
    /// * A `List` that contains a particular element within the list.
    ///
    /// The operand must be a `String` if the attribute specified by path is a `String`.
    /// If the attribute specified by path is a `Set`, the operand must be the set's element type.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn contains<V>(self, operand: V) -> Condition
    where
        V: Into<Scalar>,
    {
        Contains::new(self, operand).into()
    }

    /// Returns a number representing an attribute's size.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn size(self) -> Size {
        self.into()
    }
}

/// Methods relating to building update expressions.
impl Path {
    /// See [`Assign`]
    pub fn assign<T>(self, value: T) -> Assign
    where
        T: Into<Value>,
    {
        Assign::new(self, value)
    }

    /// Sets this as the destination in a [`Math`] builder.
    pub fn math(self) -> MathBuilder {
        Math::builder(self)
    }

    /// Sets this as the destination in a [`ListAppend`] builder.
    pub fn list_append(self) -> ListAppendBuilder {
        ListAppend::builder(self)
    }

    /// Sets this as the destination in an [`IfNotExists`] builder.
    pub fn if_not_exists(self) -> IfNotExistsBuilder {
        IfNotExists::builder(self)
    }

    /// See [`Delete`]
    pub fn delete<T>(self, set: T) -> Delete
    where
        T: Into<value::Set>,
    {
        Delete::new(self, set)
    }

    /// See [`Add`]
    #[allow(clippy::should_implement_trait)]
    pub fn add<T>(self, value: T) -> Add
    where
        T: Into<AddValue>,
    {
        Add::new(self, value)
    }

    /// See [`Remove`]
    ///
    /// [`Remove`]: crate::update::Remove
    pub fn remove(self) -> Remove {
        self.into()
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        self.elements.iter().try_for_each(|elem| {
            if first {
                first = false;
            } else {
                f.write_char('.')?;
            }

            elem.fmt(f)
        })
    }
}

impl<T> From<T> for Path
where
    T: Into<Element>,
{
    fn from(value: T) -> Self {
        Path {
            elements: vec![value.into()],
        }
    }
}

impl<T> FromIterator<T> for Path
where
    T: Into<Element>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            elements: iter.into_iter().map(Into::into).collect(),
        }
    }
}

impl FromStr for Path {
    type Err = PathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            elements: s.split('.').map(Element::from_str).try_collect()?,
        })
    }
}

impl TryFrom<Path> for Name {
    type Error = Path;

    /// If the [`Path`] is just a single [`Name`] element, this will return `Ok`
    /// with that `Name`. Otherwise, the original `Path` is returned in the `Err`.
    fn try_from(path: Path) -> Result<Self, Self::Error> {
        let element: [_; 1] = path
            .elements
            .try_into()
            .map_err(|elements| Path { elements })?;

        if let [Element::Name(name)] = element {
            Ok(name)
        } else {
            Err(Path {
                elements: element.into(),
            })
        }
    }
}

/// A [`Path`] (or path [`Element`] of a path) failed to parse.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("invalid document path")]
pub struct PathParseError;

#[cfg(test)]
mod test {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::{num_value, Comparator};

    use super::{Element, IndexedField, Name, Path, PathParseError};

    #[test]
    fn parse_path() {
        let path: Path = "foo".parse().unwrap();
        assert_eq!(Path::from(Name::from("foo")), path);

        let path: Path = "foo[0]".parse().unwrap();
        assert_eq!(Path::from(("foo", 0)), path);

        let path: Path = "foo[0][3]".parse().unwrap();
        assert_eq!(Path::from(("foo", [0, 3])), path);

        let path: Path = "foo[42][37][9]".parse().unwrap();
        assert_eq!(Path::from(("foo", [42, 37, 9])), path);

        let path: Path = "foo.bar".parse().unwrap();
        assert_eq!(Path::from_iter(["foo", "bar"].map(Name::from)), path);

        let path: Path = "foo[42].bar".parse().unwrap();
        assert_eq!(
            Path::from_iter([Element::indexed_field("foo", 42), Element::name("bar")]),
            path
        );

        let path: Path = "foo.bar[37]".parse().unwrap();
        assert_eq!(
            Path::from_iter([Element::name("foo"), Element::indexed_field("bar", 37)]),
            path
        );

        let path: Path = "foo[42].bar[37]".parse().unwrap();
        assert_eq!(Path::from_iter([("foo", 42), ("bar", 37)]), path);

        let path: Path = "foo[42][7].bar[37]".parse().unwrap();
        assert_eq!(
            Path::from_iter([("foo", vec![42, 7]), ("bar", vec![37])]),
            path
        );

        let path: Path = "foo[42].bar[37][9]".parse().unwrap();
        assert_eq!(
            Path::from_iter([("foo", vec![42]), ("bar", vec![37, 9])]),
            path
        );

        let path: Path = "foo[42][7].bar[37][9]".parse().unwrap();
        assert_eq!(Path::from_iter([("foo", [42, 7]), ("bar", [37, 9])]), path);

        for prefix in ["foo", "foo[0]", "foo.bar", "foo[0]bar", "foo[0]bar[1]"] {
            for bad_index in ["[9", "[]", "][", "[", "]"] {
                let input = format!("{prefix}{bad_index}");

                match input.parse::<Path>() {
                    Ok(path) => {
                        panic!("Should not have parsed invalid input {input:?} into: {path:?}");
                    }
                    Err(PathParseError) => { /* Got the expected error */ }
                }
            }
        }

        // A few other odds and ends not covered above.

        // Missing the '.' between elements.
        "foo[0]bar".parse::<Path>().unwrap_err();
        "foo[0]bar[3]".parse::<Path>().unwrap_err();

        // A stray index without a name for the field.
        "[0]".parse::<Path>().unwrap_err();
    }

    /// Demonstration/proof of how a `Path` can be expressed to prove usability.
    #[test]
    fn express_path() {
        let _: IndexedField = ("foo", 0).into();
        let _: Element = ("foo", 0).into();
        let _: Path = ("foo", 0).into();
    }

    #[test]
    fn display_name() {
        let path = Element::name("foo");
        assert_str_eq!("foo", path.to_string());
    }

    #[test]
    fn display_indexed() {
        // Also tests that `Element::indexed_field()` can accept a few different types of input.

        // From a u32
        let path = Element::indexed_field("foo", 42);
        assert_str_eq!("foo[42]", path.to_string());

        // From an array of u32
        let path = Element::indexed_field("foo", [42]);
        assert_str_eq!("foo[42]", path.to_string());

        // From a slice of u32
        let path = Element::indexed_field("foo", &([42, 37, 9])[..]);
        assert_str_eq!("foo[42][37][9]", path.to_string());
    }

    #[test]
    fn display_path() {
        let path: Path = ["foo", "bar"].into_iter().map(Name::from).collect();
        assert_str_eq!("foo.bar", path.to_string());

        let path = Path::from_iter([Element::name("foo"), Element::indexed_field("bar", 42)]);
        assert_str_eq!("foo.bar[42]", path.to_string());

        // TODO: I'm not sure this is a legal path based on these examples:
        //       https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.NestedElements.DocumentPathExamples
        //       Test whether it's valid and remove this comment or handle it appropriately.
        let path = Path::from_iter([Element::indexed_field("foo", 42), Element::name("bar")]);
        assert_str_eq!("foo[42].bar", path.to_string());
    }

    #[test]
    fn size() {
        assert_str_eq!(
            "size(a) = 0",
            "a".parse::<Path>()
                .unwrap()
                .size()
                .comparison(Comparator::Eq, num_value(0))
                .to_string()
        );
    }
}
