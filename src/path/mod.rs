mod element;
mod name;

pub use self::{
    element::{Element, IndexedField, Indexes},
    name::Name,
};

use core::{
    fmt::{self, Write},
    str::FromStr,
};

use itertools::Itertools;

use crate::{
    condition::{
        attribute_type::Type, equal, greater_than, greater_than_or_equal, less_than,
        less_than_or_equal, not_equal, AttributeType, BeginsWith, Between, Condition, Contains, In,
    },
    key::Key,
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
};

/// Represents a DynamoDB [document path][1]. For example, `foo[3][7].bar[2].baz`.
///
/// When used in an [`Expression`], attribute names in a [`Path`] are
/// automatically handled as [expression attribute names][2], allowing for names
/// that would not otherwise be permitted by DynamoDB. For example,
/// `foo[3][7].bar[2].baz` would become something similar to `#0[3][7].#1[2].#2`,
/// and the names would be in the `expression_attribute_names`.
///
/// See also: [`Element`], [`Name`], [`IndexedField`]
///
/// # Examples
///
/// The safest way to construct a [`Path`] is to [parse] it.
/// ```
/// use dynamodb_expression::path::Path;
/// # use pretty_assertions::assert_eq;
///
/// let path: Path = "foo".parse().unwrap();
/// let path: Path = "foo[3]".parse().unwrap();
/// let path: Path = "foo[3][7]".parse().unwrap();
/// let path: Path = "foo[3][7].bar".parse().unwrap();
/// let path: Path = "bar.baz".parse().unwrap();
/// let path: Path = "baz[0].foo".parse().unwrap();
/// ```
///
/// This makes the common assumption that each path element is separated by a
/// period (`.`). For example, the path `foo.bar` gets treated as if `foo` is a
/// top-level attribute, and `bar` is a sub-attribute of `foo`. However, `.` [is
/// also a valid character in an attribute name][3].
///
/// If you have an attribute name with a `.` in it, and need it to not be
/// treated as a separator, you can construct the [`Path`] a few different ways.
/// Here are some ways you can correctly construct a [`Path`] using `attr.name`
/// as the problematic attribute name.
/// ```
/// use dynamodb_expression::path::{Element, Path};
/// # use pretty_assertions::assert_eq;
///
/// // As a top-level attribute name:
/// let path = Path::name("attr.name");
///
/// // If the top-level attribute, `foo`, has a sub-attribute named `attr.name`:
/// let path = Path::from_iter([
///     Element::name("foo"),
///     Element::name("attr.name"),
/// ]);
///
/// // If top-level attribute `foo`, item 3 (i.e., `foo[3]`) has a sub-attribute
/// // named `attr.name`:
/// let path = Path::from_iter([
///     Element::indexed_field("foo", 3),
///     Element::name("attr.name"),
/// ]);
///
/// // If top-level attribute `foo`, item 3, sub-item 7 (i.e., `foo[3][7]`) has
/// // an attribute named `attr.name`:
/// let path = Path::from_iter([
///     Element::indexed_field("foo", [3, 7]),
///     Element::name("attr.name"),
/// ]);
/// ```
///
/// Each of these are ways to create a [`Path`] instance for `foo[3][7].bar[2].baz`.
/// ```
/// use dynamodb_expression::{path::{Element, Path}};
/// # use pretty_assertions::assert_eq;
/// #
/// # let expected = Path::from_iter([
/// #     Element::indexed_field("foo", [3, 7]),
/// #     Element::indexed_field("bar", 2),
/// #     Element::name("baz"),
/// # ]);
///
/// // A `Path` can be parsed from a string
/// let path: Path = "foo[3][7].bar[2].baz".parse().unwrap();
/// # assert_eq!(expected, path);
///
/// // `Path` implements `FromIterator` for items that are `Element`s.
/// let path = Path::from_iter([
///     Element::indexed_field("foo", [3, 7]),
///     Element::indexed_field("bar", 2),
///     Element::name("baz"),
/// ]);
/// # assert_eq!(expected, path);
///
/// // Of course, that means you can `.collect()` into a `Path`.
/// let path: Path = [
///     Element::indexed_field("foo", [3, 7]),
///     Element::indexed_field("bar", 2),
///     Element::name("baz"),
/// ]
/// .into_iter()
/// .collect();
/// # assert_eq!(expected, path);
///
/// // `Element` can be converted into from string/index tuples. Where the
/// // string is the attribute name. In this case, an "index" is an array,
/// // slice, `Vec` of, or a single `usize`.
/// //
/// // It's smart about it, though. If if there's one or zero indexes it'll do
/// // the right thing. This helps when you're chaining iterator adapters and
/// // the results are values with inconsistent numbers of indexes.
/// let path = Path::from_iter(
///     [
///         ("foo", vec![3, 7]),
///         ("bar", vec![2]),
///         ("baz", vec![]),
///     ]
///     .map(Element::from),
/// );
/// # assert_eq!(expected, path);
///
///
/// // `Path` implements `FromIterator` for items that are `Into<Element>`.
/// // So, the above example can be simplified.
/// let path = Path::from_iter([
///     ("foo", vec![3, 7]),
///     ("bar", vec![2]),
///     ("baz", vec![]),
/// ]);
/// # assert_eq!(expected, path);
/// ```
///
/// If you have a document path where an [attribute name includes a period][3]
/// (`.`), you will need to explicitly create the [`Element`]s for that `Path`.
/// ```
/// # use dynamodb_expression::path::{Element, Path};
/// # use pretty_assertions::assert_eq;
/// // If the attribute name is `foo.bar`:
/// let path = Path::from(Element::name("foo.bar"));
/// # assert_eq!(Path::from_iter([Element::name("foo.bar")]), path);
///
/// // If the item at `foo[3]` has an attribute named `bar.baz`:
/// let path = Path::from_iter([
///     Element::indexed_field("foo", 3),
///     Element::name("bar.baz")
/// ]);
/// # assert_eq!("foo[3].bar.baz", path.to_string());
/// ```
///
/// A [`Name`] can be converted into a `Path`.
/// ```
/// use dynamodb_expression::path::{Element, Name, Path};
///
/// let name = Name::from("foo");
/// let path = Path::from(name);
/// assert_eq!(Path::from(Element::name("foo")), path);
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.NestedElements.DocumentPathExamples
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html
/// [3]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.TopLevelAttributes
/// [parse]: str::parse
/// [`Expression`]: crate::expression::Expression
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path {
    pub(crate) elements: Vec<Element>,
}

impl Path {
    /// Constructs a [`Path`] for a single attribute name (with no indexes or
    /// sub-attributes). If you have a attribute name with one or more indexes,
    /// use [`Path::indexed_field()`].
    ///
    /// [`Path::indexed_field()`]: Self::indexed_field
    pub fn name<T>(name: T) -> Self
    where
        T: Into<Name>,
    {
        Self {
            elements: vec![Element::name(name)],
        }
    }

    /// Constructs a [`Path`] for a single attribute name (with no indexes or
    /// sub-attributes). If you have a attribute name with no indexes, you can
    /// pass an empty collection, or use [`Path::name`].
    ///
    /// `indexes` here can be an array, slice, `Vec` of, or single `usize`.
    /// ```
    /// # use dynamodb_expression::path::Path;
    /// # use pretty_assertions::assert_eq;
    /// #
    /// assert_eq!("foo[3]", Path::indexed_field("foo", 3).to_string());
    /// assert_eq!("foo[3]", Path::indexed_field("foo", [3]).to_string());
    /// assert_eq!("foo[3]", Path::indexed_field("foo", &[3]).to_string());
    /// assert_eq!("foo[3]", Path::indexed_field("foo", vec![3]).to_string());
    ///
    /// assert_eq!("foo[7][4]", Path::indexed_field("foo", [7, 4]).to_string());
    /// assert_eq!("foo[7][4]", Path::indexed_field("foo", &[7, 4]).to_string());
    /// assert_eq!("foo[7][4]", Path::indexed_field("foo", vec![7, 4]).to_string());
    ///
    /// assert_eq!("foo", Path::indexed_field("foo", []).to_string());
    /// assert_eq!("foo", Path::indexed_field("foo", &[]).to_string());
    /// assert_eq!("foo", Path::indexed_field("foo", vec![]).to_string());
    /// ```
    ///
    /// See also: [`IndexedField`], [`Element::indexed_field`]
    ///
    /// [`Path::name`]: Self::name
    pub fn indexed_field<N, I>(name: N, indexes: I) -> Self
    where
        N: Into<Name>,
        I: Indexes,
    {
        Self {
            elements: vec![Element::indexed_field(name, indexes)],
        }
    }

    /// Appends another [`Path`] to the end of this one.
    ///
    /// ```
    /// use dynamodb_expression::path::Path;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let mut path: Path = "foo[2]".parse().unwrap();
    /// let sub_path: Path = "bar".parse().unwrap();
    /// path.append(sub_path);
    /// assert_eq!("foo[2].bar".parse::<Path>().unwrap(), path);
    /// ```
    pub fn append(&mut self, mut other: Path) {
        self.elements.append(&mut other.elements);
    }
}

/// Methods relating to building condition and filter expressions.
impl Path {
    /// Check if the value at this [`Path`] is equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn equal<T>(self, right: T) -> Condition
    where
        T: Into<Operand>,
    {
        equal(self, right).into()
    }

    /// Check if the value at this [`Path`] is not equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn not_equal<T>(self, right: T) -> Condition
    where
        T: Into<Operand>,
    {
        not_equal(self, right).into()
    }

    /// Check if the value at this [`Path`] is greater than the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn greater_than<T>(self, right: T) -> Condition
    where
        T: Into<Operand>,
    {
        greater_than(self, right).into()
    }

    /// Check if the value at this [`Path`] is greater than or equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn greater_than_or_equal<T>(self, right: T) -> Condition
    where
        T: Into<Operand>,
    {
        greater_than_or_equal(self, right).into()
    }

    /// Check if the value at this [`Path`] is less than the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn less_than<T>(self, right: T) -> Condition
    where
        T: Into<Operand>,
    {
        less_than(self, right).into()
    }

    /// Check if the value at this [`Path`] is less than or equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn less_than_or_equal<T>(self, right: T) -> Condition
    where
        T: Into<Operand>,
    {
        less_than_or_equal(self, right).into()
    }

    /// `self BETWEEN b AND c` - true if `self` is greater than or equal to `b`, and less than or equal to `c`.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
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
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn in_<I, T>(self, items: I) -> Condition
    where
        I: IntoIterator<Item = T>,
        T: Into<Operand>,
    {
        In::new(self.into(), items).into()
    }

    /// True if the item contains the attribute specified by `path`.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn attribute_exists(self) -> Condition {
        Condition::AttributeExists(self.into())
    }

    /// True if the attribute specified by `path` does not exist in the item.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn attribute_not_exists(self) -> Condition {
        Condition::AttributeNotExists(self.into())
    }

    /// True if the attribute at the specified `path` is of a particular data type.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn attribute_type(self, attribute_type: Type) -> Condition {
        AttributeType::new(self, attribute_type).into()
    }

    /// True if the attribute specified by `path` begins with a particular substring.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
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
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn contains<V>(self, operand: V) -> Condition
    where
        V: Into<Scalar>,
    {
        Contains::new(self, operand).into()
    }

    /// Returns a number representing an attribute's size.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
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

impl Path {
    /// Turns this [`Path`] into a [`Key`], for building a [key condition expression][1].
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.KeyConditionExpressions.html
    pub fn key(self) -> Key {
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

impl From<Path> for String {
    fn from(path: Path) -> Self {
        path.elements
            .into_iter()
            .map(String::from)
            .collect_vec()
            .join(".")
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

impl From<Path> for Vec<Element> {
    fn from(path: Path) -> Self {
        path.elements
    }
}

impl TryFrom<Path> for Name {
    type Error = Path;

    /// A [`Path`] consisting of a single, unindexed attribute can be converted
    /// into a [`Name`].
    /// ```
    /// use dynamodb_expression::path::{Element, Name, Path};
    ///
    /// let path: Path = "foo".parse().unwrap();
    /// let name = Name::try_from(path).unwrap();
    /// assert_eq!(Name::from("foo"), name);
    ///```
    ///
    /// If the `Path` has indexes, or has sub-attributes, it cannot be
    /// converted, and the original `Path` is returned.
    /// ```
    /// # use dynamodb_expression::path::{Element, Name, Path};
    /// #
    /// let path: Path = "foo[0]".parse().unwrap();
    /// let err = Name::try_from(path.clone()).unwrap_err();
    /// assert_eq!(path, err);
    ///
    /// let path: Path = "foo.bar".parse().unwrap();
    /// let err = Name::try_from(path.clone()).unwrap_err();
    /// assert_eq!(path, err);
    /// ```
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

/// A [`Path`] (or [`Element`] of a path) failed to parse.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("invalid document path")]
pub struct PathParseError;

#[cfg(test)]
mod test {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::num_value;

    use super::{Element, Name, Path, PathParseError};

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

        // From a usize
        let path = Element::indexed_field("foo", 42);
        assert_str_eq!("foo[42]", path.to_string());

        // From an array of usize
        let path = Element::indexed_field("foo", [42]);
        assert_str_eq!("foo[42]", path.to_string());

        // From a slice of usize
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
                .equal(num_value(0))
                .to_string()
        );
    }
}
