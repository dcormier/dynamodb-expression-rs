//! Types related to [DynamoDB document paths][1]. For more, see [`Path`].
//!
//! [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.NestedElements.DocumentPathExamples

mod element;
mod name;

pub use self::{
    element::{Element, IndexedField, Indexes},
    name::Name,
};

use core::{
    fmt::{self, Write},
    ops,
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
        if_not_exists::Builder as IfNotExistsBuilder, list_append::Builder as ListAppendBuilder,
        math::Builder as MathBuilder, Add, AddValue, Assign, Delete, IfNotExists, ListAppend, Math,
        Remove,
    },
    value::{self, StringOrRef, Value},
};

/// Represents a DynamoDB [document path][1]. For example, `foo[3][7].bar[2].baz`.
///
/// You can use the many methods on [`Path`] for building [DynamoDB
/// expressions][4].
/// For example, [`.set()`], [`.if_not_exists()`], or [`.remove()`] are some
/// methods for creating [update expressions][5]. [`.attribute_not_exists()`],
/// [`.less_than()`], and [`.contains()`] are some methods for creating
/// condition and filter expressions.
///
/// When you're ready to build an [`Expression`], use [`Expression::builder`].
///
/// When used in an [`Expression`], attribute names in a [`Path`] are
/// automatically handled as [expression attribute names][2], allowing for names
/// that would not otherwise be permitted by DynamoDB. For example,
/// `foo[3][7].bar[2].baz` would become something similar to `#0[3][7].#1[2].#2`,
/// and the names would be in the `expression_attribute_names`.
///
/// See also: [`Element`], [`Name`], [`IndexedField`]
///
/// # There are many ways to create a `Path`
///
/// For creating a new [`Path`]:
/// * Parse from a string, as seen [below](#parsing). This is the preferred way. The only
///   time when other constructors are needed is when you have an attribute name
///   with a `.` in it that must not be treated as a separator for sub-attributes.
/// * [`Path::new_name`] and [`Path::new_indexed_field`] constructors
/// * [`Path::from`] for converting anything that's `Into<Element>` into a [`Path`]
///   (see also: [`Element`])
///
/// For building a [`Path`] one step at a time:
/// * Use the [`+=`] operator
/// * Use the [`+`] operator
/// * [`Path::append`]
/// * [`Path::from_iter`]
///
/// ## Parsing
///
/// The safest way to construct a [`Path`] is to [parse] it. This treats `.` as a separator for
/// sub-attributes, and `[n]` as indexes into fields.
///
/// Since `.` [is a valid character in an attribute name][3], see
/// [below](#a-special-case-attribute-names-with--in-them) for examples of how
/// to construct a [`Path`] when an attribute name contains a `.`.
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use dynamodb_expression::{path::Element, Path};
/// # use pretty_assertions::assert_eq;
///
/// let path: Path = "foo".parse()?;
/// assert_eq!(
///     Path::from_iter([
///         Element::new_name("foo"),
///     ]),
///     path,
/// );
///
/// let path: Path = "foo[3]".parse()?;
/// assert_eq!(
///     Path::from_iter([
///         Element::new_indexed_field("foo", 3),
///     ]),
///     path,
/// );
///
/// let path: Path = "foo[3][7]".parse()?;
/// assert_eq!(
///     Path::from_iter([
///         Element::new_indexed_field("foo", [3, 7]),
///     ]),
///     path,
/// );
///
/// let path: Path = "foo[3][7].bar".parse()?;
/// assert_eq!(
///     Path::from_iter([
///         Element::new_indexed_field("foo", [3, 7]),
///         Element::new_name("bar"),
///     ]),
///     path,
/// );
///
/// let path: Path = "bar.baz".parse()?;
/// assert_eq!(Path::from_iter([
///         Element::new_name("bar"),
///         Element::new_name("baz"),
///     ]),
///     path,
/// );
///
/// let path: Path = "baz[0].foo".parse()?;
/// assert_eq!(
///     Path::from_iter([
///         Element::new_indexed_field("baz", 0),
///         Element::new_name("foo"),
///     ]),
///     path,
/// );
/// #
/// # Ok(())
/// # }
/// ```
///
/// ## A special case: attribute names with `.` in them
///
/// If you have an attribute name with a `.` in it, and need it to _not_ be
/// treated as a separator for sub-attributes (such as a domain name), you can
/// construct the [`Path`] a using [`Path::new_name`] that element of the path
/// using [`Element::new_name`].
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use dynamodb_expression::{path::Element, Path};
/// # use pretty_assertions::assert_eq;
///
/// let path = Path::new_name("example.com");
/// assert_eq!(
///     Path::from_iter([
///         Element::new_name("example.com"),
///     ]),
///     path,
/// );
///
/// let path = "foo".parse::<Path>()? + Path::new_name("example.com");
/// assert_eq!(
///     Path::from_iter([
///         Element::new_name("foo"),
///         Element::new_name("example.com"),
///     ]),
///     path,
/// );
///
/// let mut path: Path = "foo[3]".parse()?;
/// path += Element::new_name("example.com");
/// assert_eq!(
///     Path::from_iter([
///         Element::new_indexed_field("foo", 3),
///         Element::new_name("example.com"),
///     ]),
///     path,
/// );
/// #
/// # Ok(())
/// # }
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.NestedElements.DocumentPathExamples
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html
/// [3]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.TopLevelAttributes
/// [4]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.html
/// [5]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html
/// [`.set()`]: Self::set
/// [`.if_not_exists()`]: Self::if_not_exists
/// [`.remove()`]: Self::remove
/// [`.attribute_not_exists()`]: Self::attribute_not_exists
/// [`.less_than()`]: Self::less_than
/// [`.contains()`]: Self::contains
/// [`Expression`]: crate::expression::Expression
/// [`Expression::builder`]: crate::expression::Expression::builder
/// [parse]: str::parse
/// [`+=`]: #method.add_assign
/// [`+`]: #method.add-1
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path {
    pub(crate) elements: Vec<Element>,
}

impl Path {
    /// Constructs a [`Path`] for a single attribute name (with no indexes or
    /// sub-attributes). If you have a attribute name with one or more indexes,
    /// parse it from a string, or use [`Path::new_indexed_field`]. See the
    /// [`Path`] type documentation for more examples.
    ///
    /// This treats `.` as a part of the attribute name rather than as a
    /// separator for sub-attributes. To build a [`Path`] that contains a `.`
    /// that is treated as a separator, see the examples in the documentation on
    /// the [`Path`] type.
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::path::{Path, Element};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let path = Path::new_name("foo");
    /// assert_eq!(
    ///     Path::from_iter([
    ///         Element::new_name("foo"),
    ///     ]),
    ///     path,
    /// );
    ///
    /// let path = Path::new_name("foo.bar");
    /// assert_eq!(
    ///     Path::from_iter([
    ///         Element::new_name("foo.bar"),
    ///     ]),
    ///     path,
    /// );
    /// ```
    ///
    /// Contrast the above result of `Path::new_name("foo.bar")` with parsing,
    /// which treats `.` as a separator for sub-attributes:
    /// ```
    /// # use dynamodb_expression::path::{Path, Element};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let path = "foo.bar".parse().unwrap();
    /// assert_eq!(
    ///     Path::from_iter([
    ///         Element::new_name("foo"),
    ///         Element::new_name("bar"),
    ///     ]),
    ///     path,
    /// );
    /// ```
    pub fn new_name<T>(name: T) -> Self
    where
        T: Into<Name>,
    {
        Self {
            elements: vec![Element::new_name(name)],
        }
    }

    /// Constructs a [`Path`] for an indexed field element of a document path.
    /// For example, `foo[3]` or `foo[7][4]`. If you have a attribute name with
    /// no indexes, you can pass an empty collection, parse from a string, or
    /// use [`Path::new_name`]. See the [`Path`] type documentation for more
    /// examples.
    ///
    /// This treats `.` as a part of an attribute name rather than as a
    /// separator for sub-attributes. To build a [`Path`] that contains a `.`
    /// that is treated as a separator, see the examples in the documentation on
    /// the [`Path`] type.
    ///
    /// The `indexes` parameter, here, can be an array, slice, `Vec` of, or
    /// single `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::Path;
    /// # use pretty_assertions::assert_eq;
    ///
    /// assert_eq!("foo[3]", Path::new_indexed_field("foo", 3).to_string());
    /// assert_eq!("foo[3]", Path::new_indexed_field("foo", [3]).to_string());
    /// assert_eq!("foo[3]", Path::new_indexed_field("foo", &[3]).to_string());
    /// assert_eq!("foo[3]", Path::new_indexed_field("foo", vec![3]).to_string());
    ///
    /// assert_eq!("foo[7][4]", Path::new_indexed_field("foo", [7, 4]).to_string());
    /// assert_eq!("foo[7][4]", Path::new_indexed_field("foo", &[7, 4]).to_string());
    /// assert_eq!("foo[7][4]", Path::new_indexed_field("foo", vec![7, 4]).to_string());
    ///
    /// assert_eq!("foo", Path::new_indexed_field("foo", []).to_string());
    /// assert_eq!("foo", Path::new_indexed_field("foo", &[]).to_string());
    /// assert_eq!("foo", Path::new_indexed_field("foo", vec![]).to_string());
    /// ```
    ///
    /// See also: [`IndexedField`], [`Element::new_indexed_field`]
    pub fn new_indexed_field<N, I>(name: N, indexes: I) -> Self
    where
        N: Into<Name>,
        I: Indexes,
    {
        Self {
            elements: vec![Element::new_indexed_field(name, indexes)],
        }
    }

    /// Appends another [`Path`] to the end of this one, separated with a `.`.
    ///
    /// Notice that each of these examples produces the same [`Path`]: `foo[3][7].bar[2].baz`
    ///
    /// See also: [`Element`], [`Name`]
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use dynamodb_expression::{path::{Element, Name}, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// # let expected = Path::from_iter([
    /// #     Element::new_indexed_field("foo", [3, 7]),
    /// #     Element::new_indexed_field("bar", 2),
    /// #     Element::new_name("baz"),
    /// # ]);
    ///
    /// let mut path: Path = "foo[3][7]".parse()?;
    /// path.append("bar[2].baz".parse()?);
    /// assert_eq!("foo[3][7].bar[2].baz", path.to_string());
    /// # assert_eq!(expected, path);
    ///
    /// // You can start with an empty `Path` and append one element at a time.
    /// let mut path = Path::default();
    /// path.append(Element::new_indexed_field("foo", [3, 7]).into());
    /// path.append(Element::new_indexed_field("bar", 2).into());
    /// path.append(Element::new_name("baz").into());
    /// assert_eq!("foo[3][7].bar[2].baz", path.to_string());
    /// # assert_eq!(expected, path);
    ///
    /// let mut path = Path::default();
    /// path.append(("foo", [3, 7]).into());
    /// path.append(("bar", 2).into());
    /// path.append(Name::from("baz").into());
    /// assert_eq!("foo[3][7].bar[2].baz", path.to_string());
    /// # assert_eq!(expected, path);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn append(&mut self, mut other: Path) {
        self.elements.append(&mut other.elements)
    }

    /// Returns `true` if the [`Path`] contains no attributes.
    ///
    /// _Hint: you can use [`Path::append`] to add attributes to a [`Path`]._
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

/// Methods related to building condition and filter expressions.
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

    /// The [DynamoDB `BETWEEN` operator][1]. True if `self` is greater than or
    /// equal to `lower`, and less than or equal to `upper`.
    ///
    /// See also: [`Between`], [`Key::between`]
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let condition = Path::new_name("age").between(Num::new(10), Num::new(90));
    /// assert_eq!(r#"age BETWEEN 10 AND 90"#, condition.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators
    /// [`Key::between`]: crate::key::Key::between
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

    /// A [DynamoDB `IN` operation][1]. True if the value from the
    /// [`Operand`] (the `op` parameter) is equal to any value in the list (the
    /// `items` parameter).
    ///
    /// The DynamoDB allows the list to contain up to 100 values. It must have at least 1.
    ///
    /// See also: [`In`]
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use dynamodb_expression::{condition::In, operand::Operand, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let condition = "name".parse::<Path>()?.in_(["Jack", "Jill"]);
    /// assert_eq!(r#"name IN ("Jack","Jill")"#, condition.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators
    pub fn in_<I, T>(self, items: I) -> Condition
    where
        I: IntoIterator<Item = T>,
        T: Into<Operand>,
    {
        In::new(self, items).into()
    }

    /// The [DynamoDB `attribute_exists` function][1]. True if the item contains
    /// the attribute specified by [`Path`].
    ///
    /// ```
    /// use dynamodb_expression::Path;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let condition = Path::new_name("foo").attribute_exists();
    /// assert_eq!("attribute_exists(foo)", condition.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
    pub fn attribute_exists(self) -> Condition {
        Condition::AttributeExists(self.into())
    }

    /// The [DynamoDB `attribute_not_exists` function][1]. True if the item does
    /// not contain the attribute specified by [`Path`].
    ///
    /// ```
    /// use dynamodb_expression::Path;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let condition = Path::new_name("foo").attribute_not_exists();
    /// assert_eq!("attribute_not_exists(foo)", condition.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
    pub fn attribute_not_exists(self) -> Condition {
        Condition::AttributeNotExists(self.into())
    }

    /// The [DynamoDB `attribute_type` function][1]. True if the attribute at
    /// the specified [`Path`] is of the specified data type.
    ///
    /// ```
    /// use dynamodb_expression::{condition::attribute_type::Type, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let condition = Path::new_name("foo").attribute_type(Type::String);
    /// assert_eq!("attribute_type(foo, S)", condition.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
    pub fn attribute_type(self, attribute_type: Type) -> Condition {
        AttributeType::new(self, attribute_type).into()
    }

    /// The [DynamoDB `begins_with` function][1]. True if the attribute specified by
    ///  the [`Path`] begins with a particular substring.
    ///
    /// `begins_with` can take a string or a reference to an extended attribute
    /// value. Here's an example.
    ///
    /// See also: [`Ref`]
    ///
    /// ```
    /// use dynamodb_expression::{condition::BeginsWith, value::Ref, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let begins_with = Path::new_name("foo").begins_with("T");
    /// assert_eq!(r#"begins_with(foo, "T")"#, begins_with.to_string());
    ///
    /// let begins_with = Path::new_name("foo").begins_with(Ref::new("prefix"));
    /// assert_eq!(r#"begins_with(foo, :prefix)"#, begins_with.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
    /// [`Ref`]: crate::value::Ref
    pub fn begins_with<T>(self, prefix: T) -> Condition
    where
        T: Into<StringOrRef>,
    {
        BeginsWith::new(self, prefix).into()
    }

    /// The [DynamoDB `contains` function][1]. True if the attribute specified
    /// by [`Path`] is one of the following:
    /// * A `String` that contains a particular substring.
    /// * A `Set` that contains a particular element within the set.
    /// * A `List` that contains a particular element within the list.
    ///
    /// The operand must be a `String` if the attribute specified by path is a
    /// `String`. If the attribute specified by path is a `Set`, the operand
    /// must be the sets element type.
    ///
    /// See also: [`Contains`]
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use dynamodb_expression::{Num, Path};
    ///
    /// // String
    /// let condition = "foo".parse::<Path>()?.contains("Quinn");
    /// assert_eq!(r#"contains(foo, "Quinn")"#, condition.to_string());
    ///
    /// // Number
    /// let condition = "foo".parse::<Path>()?.contains(Num::new(42));
    /// assert_eq!(r#"contains(foo, 42)"#, condition.to_string());
    ///
    /// // Binary
    /// let condition = "foo".parse::<Path>()?.contains(Vec::<u8>::from("fish"));
    /// assert_eq!(r#"contains(foo, "ZmlzaA==")"#, condition.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
    pub fn contains<V>(self, operand: V) -> Condition
    where
        V: Into<Value>,
    {
        Contains::new(self, operand).into()
    }

    /// The [DynamoDB `size` function][1]. Returns a number representing an attributes size.
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let condition = Path::new_name("foo").size().greater_than(Num::new(0));
    /// assert_eq!("size(foo) > 0", condition.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
    pub fn size(self) -> Size {
        self.into()
    }
}

/// Methods related to building update expressions.
///
/// See also: [`Update`]
///
/// [`Update`]: crate::update::Update
impl Path {
    #[deprecated(since = "0.2.0-beta.6", note = "Use `.set(value)` instead")]
    pub fn assign<T>(self, value: T) -> Assign
    where
        T: Into<Value>,
    {
        self.set(value)
    }

    /// Represents assigning a value of a [attribute][1], [list][2], or [map][3].
    ///
    /// See also: [`Update`]
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path, update::Update};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let assign = Path::new_name("name").set("Jill");
    /// assert_eq!(r#"name = "Jill""#, assign.to_string());
    ///
    /// let update = Update::from(assign);
    /// assert_eq!(r#"SET name = "Jill""#, update.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.ModifyingAttributes
    /// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.AddingListElements
    /// [3]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.AddingNestedMapAttributes
    /// [`Update`]: crate::update::Update
    pub fn set<T>(self, value: T) -> Assign
    where
        T: Into<Value>,
    {
        Assign::new(self, value)
    }

    /// Use for doing [math on a numeric attribute][1].
    ///
    /// Sets this as the destination in a [`Math`] builder.
    ///
    /// See also: [`Update`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::{Path, update::Update};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let math = Path::new_name("foo").math().add(4);
    /// assert_eq!("foo = foo + 4", math.to_string());
    ///
    /// let math = Path::new_name("foo").math().src(Path::new_name("bar")).sub(7);
    /// assert_eq!("foo = bar - 7", math.to_string());
    ///
    /// let update = Update::from(math);
    /// assert_eq!("SET foo = bar - 7", update.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.IncrementAndDecrement
    /// [`Update`]: crate::update::Update
    pub fn math(self) -> MathBuilder {
        Math::builder(self)
    }

    /// Represents an update expression to [append elements to a list][1].
    ///
    /// See also: [`Update`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path, update::Update};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let list_append = "foo".parse::<Path>().unwrap().list_append().list([7, 8, 9].map(Num::new));
    /// assert_eq!("foo = list_append(foo, [7, 8, 9])", list_append.to_string());
    ///
    /// let update = Update::from(list_append);
    /// assert_eq!("SET foo = list_append(foo, [7, 8, 9])", update.to_string());
    /// ```
    ///
    /// If you want to add the new values to the _beginning_ of the list instead,
    /// use the [`.before()`] method.
    /// ```
    /// use dynamodb_expression::{Num, Path, update::Update};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let list_append = "foo".parse::<Path>().unwrap().list_append().before().list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append([1, 2, 3], foo)", list_append.to_string());
    ///
    /// let update = Update::from(list_append);
    /// assert_eq!("SET foo = list_append([1, 2, 3], foo)", update.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.UpdatingListElements
    /// [`Update`]: crate::update::Update
    /// [`.before()`]: ListAppendBuilder::before
    pub fn list_append(self) -> ListAppendBuilder {
        ListAppend::builder(self)
    }

    /// Represents an update expression to [set an attribute if it doesn't exist][1].
    ///
    /// See also: [`Update`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path, update::Update};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let if_not_exists = "foo".parse::<Path>().unwrap().if_not_exists().set(Num::new(7));
    /// assert_eq!("foo = if_not_exists(foo, 7)", if_not_exists.to_string());
    ///
    /// let update = Update::from(if_not_exists);
    /// assert_eq!("SET foo = if_not_exists(foo, 7)", update.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.PreventingAttributeOverwrites
    /// [`Update`]: crate::update::Update
    pub fn if_not_exists(self) -> IfNotExistsBuilder {
        IfNotExists::builder(self)
    }

    /// Creates a [`DELETE` statement for an update expression][1], for removing
    /// one or more items from a value that is a [set][2].
    ///
    /// See also: [`Update`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::{Path, update::Update, value::StringSet};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let delete = Path::new_name("foo").delete(StringSet::new(["a", "b", "c"]));
    /// assert_eq!(r#"DELETE foo ["a", "b", "c"]"#, delete.to_string());
    ///
    /// let update = Update::from(delete);
    /// assert_eq!(r#"DELETE foo ["a", "b", "c"]"#, update.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.DELETE
    /// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
    /// [`Update`]: crate::update::Update
    pub fn delete<T>(self, set: T) -> Delete
    where
        T: Into<value::Set>,
    {
        Delete::new(self, set)
    }

    /// Represents an DynamoDB [`ADD` statement][1] in an [update expression][2].
    ///
    /// The [DynamoDB documentation recommends][1] against using `ADD`:
    ///
    /// > In general, we recommend using `SET` rather than `ADD`.
    ///
    /// To increment or decrement a number value, use [`Path::math`].
    ///
    /// To append items to a list, use [`Path::list_append`].
    ///
    /// See also: [`AddValue`], [`Update`], [`Set`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path, update::Update};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let add = Path::new_name("foo").add(Num::from(1));
    /// assert_eq!("ADD foo 1", add.to_string());
    ///
    /// let update = Update::from(add);
    /// assert_eq!("ADD foo 1", update.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.ADD
    /// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html
    /// [`Update`]: crate::update::Update
    /// [`Set`]: crate::update::Set
    #[rustversion::attr(before(1.81), allow(clippy::should_implement_trait))]
    #[rustversion::attr(
        since(1.81),
        expect(
            clippy::should_implement_trait,
            reason = "This is for the DynamoDB `ADD` operation, not the Rust `+` operator."
        )
    )]
    pub fn add<T>(self, value: T) -> Add
    where
        T: Into<AddValue>,
    {
        Add::new(self, value)
    }

    /// Creates an update expression to [remove attributes from an item][1], or
    /// [elements from a list][2].
    ///
    /// See also: [`Remove`], [`Update`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::{Path, update::Update};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let remove = Path::new_name("foo").remove();
    /// assert_eq!(r#"REMOVE foo"#, remove.to_string());
    ///
    /// let update = Update::from(remove);
    /// assert_eq!(r#"REMOVE foo"#, update.to_string());
    ///
    /// let remove = Path::new_indexed_field("foo", [8]).remove();
    /// assert_eq!(r#"REMOVE foo[8]"#, remove.to_string());
    ///
    /// let update = Update::from(remove);
    /// assert_eq!(r#"REMOVE foo[8]"#, update.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.REMOVE
    /// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.REMOVE.RemovingListElements
    /// [`Update`]: crate::update::Update
    pub fn remove(self) -> Remove {
        self.into()
    }
}

impl Path {
    /// Turns this [`Path`] into a [`Key`], for building a [key condition expression][1].
    ///
    /// See also: [`Key`]
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use dynamodb_expression::{key::KeyCondition, Expression, Num, Path};
    ///
    /// let key_condition: KeyCondition = "id"
    ///     .parse::<Path>()?
    ///     .key()
    ///     .equal(Num::new(42))
    ///     .and("category".parse::<Path>()?.key().begins_with("hardware."));
    ///
    /// let expression = Expression::builder().with_key_condition(key_condition).build();
    /// # _ = expression;
    /// #
    /// # Ok(())
    /// # }
    /// ```
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

impl<T> ops::Add<T> for Path
where
    T: Into<Path>,
{
    type Output = Self;

    /// Allows for using the `+` operator to combine two [`Path`]s.
    ///
    /// Notice that each of these examples produces the same path:
    /// `foo[3][7].bar[2].baz`
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use dynamodb_expression::{path::{Element, Name}, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// # let expected: Path= "foo[3][7].bar[2].baz".parse()?;
    /// #
    /// let path: Path = "foo[3][7]".parse()?;
    /// let path = path + "bar[2].baz".parse::<Path>()?;
    /// assert_eq!("foo[3][7].bar[2].baz", path.to_string());
    /// # assert_eq!(expected, path);
    ///
    /// // You can `+` anything that is `Into<Path>` (or `Into<Element>`)
    ///
    /// let path = Path::new_indexed_field("foo", [3, 7]) +
    ///     Element::new_indexed_field("bar", 2) +
    ///     Element::new_name("baz");
    /// assert_eq!("foo[3][7].bar[2].baz", path.to_string());
    /// # assert_eq!(expected, path);
    ///
    /// let path = Path::from(("foo", [3, 7])) +
    ///     ("bar", 2) +
    ///     Name::from("baz");
    /// assert_eq!("foo[3][7].bar[2].baz", path.to_string());
    /// # assert_eq!(expected, path);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn add(mut self, rhs: T) -> Self::Output {
        self.append(rhs.into());
        self
    }
}

impl<T> ops::AddAssign<T> for Path
where
    T: Into<Path>,
{
    /// Allows for using the `+=` operator to combine two [`Path`]s.
    ///
    /// Notice that each of these examples produces the same path:
    /// `foo[3][7].bar[2].baz`
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use dynamodb_expression::{path::{Element, Name}, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// # let expected: Path= "foo[3][7].bar[2].baz".parse()?;
    /// #
    /// let mut path: Path = "foo[3][7]".parse()?;
    /// path += "bar[2].baz".parse::<Path>()?;
    /// assert_eq!("foo[3][7].bar[2].baz", path.to_string());
    /// # assert_eq!(expected, path);
    ///
    /// // You can `+=` anything that is `Into<Path>` (or `Into<Element>`)
    ///
    /// let mut path = Path::default();
    /// path += Path::new_indexed_field("foo", [3, 7]);
    /// path += Element::new_indexed_field("bar", 2);
    /// path += Element::new_name("baz");
    /// assert_eq!("foo[3][7].bar[2].baz", path.to_string());
    /// # assert_eq!(expected, path);
    ///
    /// let mut path = Path::default();
    /// path += Path::from(("foo", [3, 7]));
    /// path += ("bar", 2);
    /// path += Name::from("baz");
    /// assert_eq!("foo[3][7].bar[2].baz", path.to_string());
    /// # assert_eq!(expected, path);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn add_assign(&mut self, rhs: T) {
        self.append(rhs.into());
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
    T: Into<Path>,
{
    /// Comines multiple items into a single [`Path`], with each element
    /// separated by `.`. Items must be `Into<Path>`.
    ///
    /// Notice that each of these examples produces the same [`Path`]:
    /// `foo[3][7].bar[2].baz`
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use dynamodb_expression::{path::Element, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// # let expected = Path::from_iter([
    /// #     Element::new_indexed_field("foo", [3, 7]),
    /// #     Element::new_indexed_field("bar", 2),
    /// #     Element::new_name("baz"),
    /// # ]);
    ///
    /// // `Path` items
    /// let path: Path = [
    ///         "foo[3][7]".parse::<Path>()?,
    ///         "bar[2]".parse::<Path>()?,
    ///         "baz".parse::<Path>()?,
    ///     ]
    ///     .into_iter()
    ///     .collect();
    /// assert_eq!("foo[3][7].bar[2].baz", path.to_string());
    /// # assert_eq!(expected, path);
    ///
    /// // `Element` items
    /// let path: Path = [
    ///         Element::new_indexed_field("foo", [3, 7]),
    ///         Element::new_indexed_field("bar", 2),
    ///         Element::new_name("baz"),
    ///     ]
    ///     .into_iter()
    ///     .collect();
    /// assert_eq!("foo[3][7].bar[2].baz", path.to_string());
    /// # assert_eq!(expected, path);
    ///
    /// // `Into<Element>` items
    /// let path: Path = [
    ///         ("foo", vec![3, 7]),
    ///         ("bar", vec![2]),
    ///         ("baz", vec![]),
    ///     ]
    ///     .into_iter()
    ///     .collect();
    /// assert_eq!("foo[3][7].bar[2].baz", path.to_string());
    /// # assert_eq!(expected, path);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            elements: iter
                .into_iter()
                .map(Into::into)
                .flat_map(|path| path.elements)
                .collect(),
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

impl From<Path> for String {
    fn from(path: Path) -> Self {
        path.elements
            .into_iter()
            .map(String::from)
            .collect_vec()
            .join(".")
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
    /// # use dynamodb_expression::path::{Element, Name, Path};
    /// #
    /// let path: Path = "foo".parse().unwrap();
    /// let name = Name::try_from(path).unwrap();
    /// assert_eq!(Name::from("foo"), name);
    /// ```
    ///
    /// If the [`Path`] has indexes, or has sub-attributes, it cannot be
    /// converted, and the original [`Path`] is returned.
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
#[derive(Debug, PartialEq, Eq)]
pub struct PathParseError;

impl std::error::Error for PathParseError {}

impl fmt::Display for PathParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid document path")
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::Num;

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
            Path::from_iter([
                Element::new_indexed_field("foo", 42),
                Element::new_name("bar")
            ]),
            path
        );

        let path: Path = "foo.bar[37]".parse().unwrap();
        assert_eq!(
            Path::from_iter([
                Element::new_name("foo"),
                Element::new_indexed_field("bar", 37)
            ]),
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

    /// Demonstration/proof of how a [`Path`] can be expressed to prove usability.
    #[test]
    fn express_path() {
        let _: Element = ("foo", 0).into();
        let _: Path = ("foo", 0).into();
    }

    #[test]
    fn display_path() {
        let path: Path = ["foo", "bar"].into_iter().map(Name::from).collect();
        assert_eq!("foo.bar", path.to_string());

        let path = Path::from_iter([
            Element::new_name("foo"),
            Element::new_indexed_field("bar", 42),
        ]);
        assert_eq!("foo.bar[42]", path.to_string());

        let path = Path::from_iter([
            Element::new_indexed_field("foo", 42),
            Element::new_name("bar"),
        ]);
        assert_eq!("foo[42].bar", path.to_string());
    }

    #[test]
    fn size() {
        assert_eq!(
            "size(a) = 0",
            "a".parse::<Path>()
                .unwrap()
                .size()
                .equal(Num::new(0))
                .to_string()
        );
    }

    #[test]
    fn begins_with_string() {
        let begins_with = Path::new_indexed_field("foo", 3).begins_with("foo");
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());

        let begins_with = Path::new_indexed_field("foo", 3).begins_with(String::from("foo"));
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());

        #[allow(clippy::needless_borrows_for_generic_args)]
        let begins_with = Path::new_indexed_field("foo", 3).begins_with(&String::from("foo"));
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());

        #[allow(clippy::needless_borrows_for_generic_args)]
        let begins_with = Path::new_indexed_field("foo", 3).begins_with(&"foo");
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());
    }

    #[test]
    fn begins_with_value_ref() {
        use crate::{path::Path, value::Ref};

        let begins_with = Path::new_indexed_field("foo", 3).begins_with(Ref::new("prefix"));
        assert_eq!("begins_with(foo[3], :prefix)", begins_with.to_string());
    }

    #[test]
    fn in_() {
        use crate::Path;

        let condition = Path::new_name("name").in_(["Jack", "Jill"]);
        assert_eq!(r#"name IN ("Jack","Jill")"#, condition.to_string());
    }

    #[test]
    fn contains() {
        // String
        let condition = Path::new_name("foo").contains("Quinn");
        assert_eq!(r#"contains(foo, "Quinn")"#, condition.to_string());

        // Number
        let condition = Path::new_name("foo").contains(Num::new(42));
        assert_eq!(r#"contains(foo, 42)"#, condition.to_string());

        // Binary
        let condition = Path::new_name("foo").contains(b"fish".to_vec());
        assert_eq!(r#"contains(foo, "ZmlzaA==")"#, condition.to_string());
    }

    #[test]
    fn empty() {
        assert!(Path::default().is_empty());
        assert!(Path::from_iter(Vec::<Element>::new()).is_empty());
        assert!(Path::from_iter(Vec::<Path>::new()).is_empty());
    }

    #[test]
    fn from_iter() {
        let path = Path::from_iter(["foo", "bar"].map(Name::from));
        assert_eq!("foo.bar", path.to_string());
        assert_eq!(
            vec![Element::new_name("foo"), Element::new_name("bar")],
            path.elements
        );

        let path = Path::from_iter([("foo", 42), ("bar", 37)]);
        assert_eq!("foo[42].bar[37]", path.to_string());
        assert_eq!(
            vec![
                Element::new_indexed_field("foo", 42),
                Element::new_indexed_field("bar", 37),
            ],
            path.elements
        );

        let path = Path::from_iter([("foo", vec![42, 7]), ("bar", vec![37])]);
        assert_eq!("foo[42][7].bar[37]", path.to_string());
        assert_eq!(
            vec![
                Element::new_indexed_field("foo", [42, 7]),
                Element::new_indexed_field("bar", 37),
            ],
            path.elements
        );

        let path = Path::from_iter([("foo", [42, 7]), ("bar", [37, 9])]);
        assert_eq!("foo[42][7].bar[37][9]", path.to_string());
        assert_eq!(
            vec![
                Element::new_indexed_field("foo", [42, 7]),
                Element::new_indexed_field("bar", [37, 9]),
            ],
            path.elements
        );

        let path = Path::from_iter([
            Element::new_name("foo"),
            Element::new_indexed_field("bar", 42),
        ]);
        assert_eq!("foo.bar[42]", path.to_string());
        assert_eq!(
            vec![
                Element::new_name("foo"),
                Element::new_indexed_field("bar", 42),
            ],
            path.elements
        );

        let path = Path::from_iter([
            "foo.bar[42]".parse::<Path>().unwrap(),
            "baz.quux".parse::<Path>().unwrap(),
        ]);
        assert_eq!("foo.bar[42].baz.quux", path.to_string());
        assert_eq!(
            vec![
                Element::new_name("foo"),
                Element::new_indexed_field("bar", 42),
                Element::new_name("baz"),
                Element::new_name("quux"),
            ],
            path.elements
        );
    }

    #[test]
    fn add() -> Result<(), Box<dyn std::error::Error>> {
        let path = "foo".parse::<Path>()? + Name::from("bar");
        assert_eq!("foo.bar", path.to_string());
        assert_eq!(
            vec![Element::new_name("foo"), Element::new_name("bar")],
            path.elements
        );

        let path = "foo[42]".parse::<Path>()? + ("bar", 37);
        assert_eq!("foo[42].bar[37]", path.to_string());
        assert_eq!(
            vec![
                Element::new_indexed_field("foo", 42),
                Element::new_indexed_field("bar", 37),
            ],
            path.elements
        );

        let path = "foo[42][7]".parse::<Path>()? + ("bar", vec![37]);
        assert_eq!("foo[42][7].bar[37]", path.to_string());
        assert_eq!(
            vec![
                Element::new_indexed_field("foo", [42, 7]),
                Element::new_indexed_field("bar", 37),
            ],
            path.elements
        );

        let path = "foo[42][7]".parse::<Path>()? + ("bar", [37, 9]);
        assert_eq!("foo[42][7].bar[37][9]", path.to_string());
        assert_eq!(
            vec![
                Element::new_indexed_field("foo", [42, 7]),
                Element::new_indexed_field("bar", [37, 9]),
            ],
            path.elements
        );

        let path = "foo".parse::<Path>()? + Element::new_indexed_field("bar", 42);
        assert_eq!("foo.bar[42]", path.to_string());
        assert_eq!(
            vec![
                Element::new_name("foo"),
                Element::new_indexed_field("bar", 42),
            ],
            path.elements
        );

        let path = "foo.bar[42]".parse::<Path>()? + "baz.quux".parse::<Path>()?;
        assert_eq!("foo.bar[42].baz.quux", path.to_string());
        assert_eq!(
            vec![
                Element::new_name("foo"),
                Element::new_indexed_field("bar", 42),
                Element::new_name("baz"),
                Element::new_name("quux"),
            ],
            path.elements
        );

        Ok(())
    }

    #[test]
    fn add_assign() -> Result<(), Box<dyn std::error::Error>> {
        let mut path = "foo".parse::<Path>()?;
        path += Name::from("bar");
        assert_eq!("foo.bar", path.to_string());
        assert_eq!(
            vec![Element::new_name("foo"), Element::new_name("bar")],
            path.elements
        );

        let mut path = "foo[42]".parse::<Path>()?;
        path += ("bar", 37);
        assert_eq!("foo[42].bar[37]", path.to_string());
        assert_eq!(
            vec![
                Element::new_indexed_field("foo", 42),
                Element::new_indexed_field("bar", 37),
            ],
            path.elements
        );

        let mut path = "foo[42][7]".parse::<Path>()?;
        path += ("bar", vec![37]);
        assert_eq!("foo[42][7].bar[37]", path.to_string());
        assert_eq!(
            vec![
                Element::new_indexed_field("foo", [42, 7]),
                Element::new_indexed_field("bar", 37),
            ],
            path.elements
        );

        let mut path = "foo[42][7]".parse::<Path>()?;
        path += ("bar", [37, 9]);
        assert_eq!("foo[42][7].bar[37][9]", path.to_string());
        assert_eq!(
            vec![
                Element::new_indexed_field("foo", [42, 7]),
                Element::new_indexed_field("bar", [37, 9]),
            ],
            path.elements
        );

        let mut path = "foo".parse::<Path>()?;
        path += Element::new_indexed_field("bar", 42);
        assert_eq!("foo.bar[42]", path.to_string());
        assert_eq!(
            vec![
                Element::new_name("foo"),
                Element::new_indexed_field("bar", 42),
            ],
            path.elements
        );

        let mut path = "foo.bar[42]".parse::<Path>()?;
        path += "baz.quux".parse::<Path>()?;
        assert_eq!("foo.bar[42].baz.quux", path.to_string());
        assert_eq!(
            vec![
                Element::new_name("foo"),
                Element::new_indexed_field("bar", 42),
                Element::new_name("baz"),
                Element::new_name("quux"),
            ],
            path.elements
        );

        Ok(())
    }
}
