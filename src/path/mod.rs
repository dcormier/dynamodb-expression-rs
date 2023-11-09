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
    value::{self, StringOrRef, Value},
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
/// ## Parsing
///
/// The safest way to construct a [`Path`] is to [parse] it.
/// ```
/// use dynamodb_expression::Path;
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
/// also a valid character in an attribute name][3]. See
/// [below](#attribute-names-with--in-them) for examples of how to construct a
/// [`Path`] when an attribute name contains a `.`.
///
/// ## There are many ways to crate a `Path`
///
/// Each of these are ways to create a [`Path`] instance for `foo[3][7].bar[2].baz`.
/// ```
/// use dynamodb_expression::{path::{Element, Path}};
/// # use pretty_assertions::assert_eq;
/// #
/// # let expected = Path::from_iter([
/// #     Element::new_indexed_field("foo", [3, 7]),
/// #     Element::new_indexed_field("bar", 2),
/// #     Element::new_name("baz"),
/// # ]);
///
/// // A `Path` can be parsed from a string
/// let path: Path = "foo[3][7].bar[2].baz".parse().unwrap();
/// # assert_eq!(expected, path);
///
/// // `Path` implements `FromIterator` for items that are `Element`s.
/// let path = Path::from_iter([
///     Element::new_indexed_field("foo", [3, 7]),
///     Element::new_indexed_field("bar", 2),
///     Element::new_name("baz"),
/// ]);
/// # assert_eq!(expected, path);
///
/// // Of course, that means you can `.collect()` into a `Path`.
/// let path: Path = [
///     Element::new_indexed_field("foo", [3, 7]),
///     Element::new_indexed_field("bar", 2),
///     Element::new_name("baz"),
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
/// // `Path` implements `FromIterator` for items that are `Into<Element>`.
/// // So, the above example can be simplified.
/// let path = Path::from_iter([
///     ("foo", vec![3, 7]),
///     ("bar", vec![2]),
///     ("baz", vec![]),
/// ]);
/// # assert_eq!(expected, path);
///
/// // You can append one [`Path`] to another.
/// let mut path = Path::new_indexed_field("foo", [3, 7]);
/// path.append(Path::new_indexed_field("bar", 2));
/// path.append(Path::new_name("baz"));
/// # assert_eq!(expected, path);
/// ```
///
/// A [`Name`] can be converted into a [`Path`].
/// ```
/// use dynamodb_expression::path::{Element, Name, Path};
/// # use pretty_assertions::assert_eq;
///
/// let name = Name::from("foo");
/// let path = Path::from(name);
/// assert_eq!(Path::from(Element::new_name("foo")), path);
/// ```
///
/// ## Attribute names with `.` in them
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
/// let path = Path::new_name("attr.name");
/// # assert_eq!(Path::from_iter([Element::new_name("attr.name")]), path);
///
/// // If the top-level attribute, `foo`, has a sub-attribute named `attr.name`:
/// let path = Path::from_iter([
///     Element::new_name("foo"),
///     Element::new_name("attr.name"),
/// ]);
///
/// // If top-level attribute `foo`, item 3 (i.e., `foo[3]`) has a sub-attribute
/// // named `attr.name`:
/// let path = Path::from_iter([
///     Element::new_indexed_field("foo", 3),
///     Element::new_name("attr.name"),
/// ]);
///
/// // If top-level attribute `foo`, item 3, sub-item 7 (i.e., `foo[3][7]`) has
/// // an attribute named `attr.name`:
/// let path = Path::from_iter([
///     Element::new_indexed_field("foo", [3, 7]),
///     Element::new_name("attr.name"),
/// ]);
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
    /// use [`Path::new_indexed_field`].
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
    /// no indexes, you can pass an empty collection, or use [`Path::new_name`].
    ///
    /// `indexes` here can be an array, slice, `Vec` of, or single `usize`.
    /// ```
    /// # use dynamodb_expression::Path;
    /// # use pretty_assertions::assert_eq;
    /// #
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

    /// Appends another [`Path`] to the end of this one.
    ///
    /// ```
    /// use dynamodb_expression::Path;
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

    /// The [DynamoDB `BETWEEN` operator][1]. True if `self` is greater than or
    /// equal to `lower`, and less than or equal to `upper`.
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let condition = Path::new_name("age").between(Num::new(10), Num::new(90));
    /// assert_eq!(r#"age BETWEEN 10 AND 90"#, condition.to_string());
    /// ```
    ///
    /// See also: [`Key::between`]
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

    /// A [DynamoDB `IN` operation][1]. True if the value at this [`Path`] is equal
    /// to any value in the list.
    ///
    /// The list can contain up to 100 values. It must have at least 1.
    ///
    /// ```
    /// use dynamodb_expression::Path;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let condition = Path::new_name("name").in_(["Jack", "Jill"]);
    /// assert_eq!(r#"name IN ("Jack","Jill")"#, condition.to_string());
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
    /// See also: [`Ref`]
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
    /// ```
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// // String
    /// let condition = Path::new_name("foo").contains("Quinn");
    /// assert_eq!(r#"contains(foo, "Quinn")"#, condition.to_string());
    ///
    /// // Number
    /// let condition = Path::new_name("foo").contains(Num::new(42));
    /// assert_eq!(r#"contains(foo, 42)"#, condition.to_string());
    ///
    /// // Binary
    /// let condition = Path::new_name("foo").contains(Vec::<u8>::from("fish"));
    /// assert_eq!(r#"contains(foo, "ZmlzaA==")"#, condition.to_string());
    /// ```
    ///
    /// See also: [`Contains`]
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

/// Methods relating to building update expressions.
///
/// See also: [`Update`]
///
/// [`Update`]: crate::update::Update
impl Path {
    /// Represents assigning a value of a [attribute][1], [list][2], or [map][3].
    ///
    /// See also: [`Update`]
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path, update::Update};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let assign = Path::new_name("name").assign("Jill");
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
    pub fn assign<T>(self, value: T) -> Assign
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
    /// let list_append = Path::new_name("foo").list_append().list([7, 8, 9].map(Num::new));
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
    /// let list_append = Path::new_name("foo").list_append().before().list([1, 2, 3].map(Num::new));
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
    /// let if_not_exists = Path::new_name("foo").if_not_exists().value(Num::new(7));
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
    /// See also: [`AddValue`], [`Update`], [`Set`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path, update::{Add, Update}};
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
    #[allow(clippy::should_implement_trait)]
    pub fn add<T>(self, value: T) -> Add
    where
        T: Into<AddValue>,
    {
        Add::new(self, value)
    }

    /// See [`Remove`]
    pub fn remove(self) -> Remove {
        self.into()
    }
}

impl Path {
    /// Turns this [`Path`] into a [`Key`], for building a [key condition expression][1].
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let key_condition = Path::new_name("id").key().equal(Num::new(42))
    ///     .and(Path::new_name("category").key().begins_with("hardware."));
    /// assert_eq!(r#"id = 42 AND begins_with(category, "hardware.")"#, key_condition.to_string());
    /// ```
    ///
    /// See methods on [`Key`] for more docs and examples.
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
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("invalid document path")]
pub struct PathParseError;

#[cfg(test)]
mod test {
    use pretty_assertions::{assert_eq, assert_str_eq};

    #[test]
    fn parse_path() {
        use crate::path::{Element, Name, Path, PathParseError};

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
        use crate::path::{Element, Path};

        let _: Element = ("foo", 0).into();
        let _: Path = ("foo", 0).into();
    }

    #[test]
    fn display_name() {
        use crate::path::Element;

        let path = Element::new_name("foo");
        assert_str_eq!("foo", path.to_string());
    }

    #[test]
    fn display_indexed() {
        // Also tests that `Element::new_indexed_field()` can accept a few different types of input.

        use crate::path::Element;

        // From a usize
        let path = Element::new_indexed_field("foo", 42);
        assert_str_eq!("foo[42]", path.to_string());

        // From an array of usize
        let path = Element::new_indexed_field("foo", [42]);
        assert_str_eq!("foo[42]", path.to_string());

        // From a slice of usize
        let path = Element::new_indexed_field("foo", &([42, 37, 9])[..]);
        assert_str_eq!("foo[42][37][9]", path.to_string());
    }

    #[test]
    fn display_path() {
        use crate::path::{Element, Name, Path};

        let path: Path = ["foo", "bar"].into_iter().map(Name::from).collect();
        assert_str_eq!("foo.bar", path.to_string());

        let path = Path::from_iter([
            Element::new_name("foo"),
            Element::new_indexed_field("bar", 42),
        ]);
        assert_str_eq!("foo.bar[42]", path.to_string());

        // TODO: I'm not sure this is a legal path based on these examples:
        //       https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.NestedElements.DocumentPathExamples
        //       Test whether it's valid and remove this comment or handle it appropriately.
        let path = Path::from_iter([
            Element::new_indexed_field("foo", 42),
            Element::new_name("bar"),
        ]);
        assert_str_eq!("foo[42].bar", path.to_string());
    }

    #[test]
    fn size() {
        use crate::{path::Path, Num};

        assert_str_eq!(
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
        use crate::path::Path;

        let begins_with = Path::new_indexed_field("foo", 3).begins_with("foo");
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());

        let begins_with = Path::new_indexed_field("foo", 3).begins_with(String::from("foo"));
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());

        #[allow(clippy::needless_borrow)]
        let begins_with = Path::new_indexed_field("foo", 3).begins_with(&String::from("foo"));
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());

        #[allow(clippy::needless_borrow)]
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
        use crate::{Num, Path};

        // String
        let condition = Path::new_name("foo").contains("Quinn");
        assert_eq!(r#"contains(foo, "Quinn")"#, condition.to_string());

        // Number
        let condition = Path::new_name("foo").contains(Num::new(42));
        assert_eq!(r#"contains(foo, 42)"#, condition.to_string());

        // Binary
        let condition = Path::new_name("foo").contains(Vec::<u8>::from("fish"));
        assert_eq!(r#"contains(foo, "ZmlzaA==")"#, condition.to_string());
    }
}
