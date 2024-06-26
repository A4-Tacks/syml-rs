#![doc = include_str!("../README.md")]

#[cfg(feature = "cli-utils")]
pub mod cli_utils;

pub mod parser;
mod serialize;

pub use serialize::SYMLSerialize;

use linked_hash_map::LinkedHashMap;
use core::{char, num, str};

pub type Table = LinkedHashMap<String, Value>;

#[cfg_attr(feature = "serde_impl", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    String(String),
    Array(Vec<Self>),
    Table(Table),
}
impl Value {
    pub fn is_empty(&self) -> bool {
        match self {
            Value::String(s) => s.is_empty(),
            Value::Array(a) => a.is_empty(),
            Value::Table(t) => t.is_empty(),
        }
    }

    /// Returns `true` if the value is [`String`].
    ///
    /// [`String`]: Value::String
    #[must_use]
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(..))
    }

    /// Returns `true` if the value is [`Array`].
    ///
    /// [`Array`]: Value::Array
    #[must_use]
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(..))
    }

    /// Returns `true` if the value is [`Table`].
    ///
    /// [`Table`]: Value::Table
    #[must_use]
    pub fn is_table(&self) -> bool {
        matches!(self, Self::Table(..))
    }

    pub fn as_string(&self) -> Option<&String> {
        if let Self::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        if let Self::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Self>> {
        if let Self::Array(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Self>> {
        if let Self::Array(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_table(&self) -> Option<&Table> {
        if let Self::Table(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        if let Self::Table(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        Some(&**self.as_string()?)
    }

    /// use [`as_array`]
    ///
    /// [`as_array`]: Self::as_array
    pub fn as_slice(&self) -> Option<&[Self]> {
        Some(&**self.as_array()?)
    }

    /// use [`as_array_mut`]
    ///
    /// [`as_array_mut`]: Self::as_array_mut
    pub fn as_slice_mut(&mut self) -> Option<&mut [Self]> {
        Some(&mut **self.as_array_mut()?)
    }

}
impl From<&'_ str> for Value {
    fn from(value: &'_ str) -> Self {
        value.to_owned().into()
    }
}
impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<Vec<Self>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Self::Array(value)
    }
}
impl From<Vec<&'_ str>> for Value {
    fn from(value: Vec<&'_ str>) -> Self {
        value.into_iter().collect()
    }
}
impl<T: Into<Self>, const N: usize> From<[T; N]> for Value {
    fn from(value: [T; N]) -> Self {
        Self::Array(value.into_iter()
            .map(Into::into)
            .collect())
    }
}
impl<K, V, const N: usize> From<[(K, V); N]> for Value
where K: Into<String>, V: Into<Self>,
{
    fn from(value: [(K, V); N]) -> Self {
        Self::Table(value.into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect())
    }
}
impl From<Vec<(String, Self)>> for Value {
    fn from(value: Vec<(String, Self)>) -> Self {
        Self::from_iter(value)
    }
}
impl From<Table> for Value {
    fn from(value: Table) -> Self {
        Self::Table(value)
    }
}
impl FromIterator<(String, Self)> for Value {
    fn from_iter<T: IntoIterator<Item = (String, Self)>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut map
            = Table::with_capacity(iter.size_hint().0);
        iter.for_each(|(k, v)| {
            map.entry(k).or_insert(v);
        });
        map.into()
    }
}
impl<T1: Into<Self>> FromIterator<T1> for Value {
    fn from_iter<T: IntoIterator<Item = T1>>(iter: T) -> Self {
        Vec::from_iter(iter.into_iter()
            .map(Into::into)).into()
    }
}
impl AsRef<Self> for Value {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl TryFrom<Value> for String {
    type Error = Value;

    fn try_from(this: Value) -> Result<String, Self::Error> {
        match this {
            Value::String(s) => Ok(s),
            oth => Err(oth),
        }
    }
}
impl TryFrom<Value> for Vec<Value> {
    type Error = Value;

    fn try_from(this: Value) -> Result<Vec<Value>, Value> {
        match this {
            Value::Array(arr) => Ok(arr),
            oth => Err(oth),
        }
    }
}
impl TryFrom<Value> for Table {
    type Error = Value;

    fn try_from(this: Value) -> Result<Table, Value> {
        match this {
            Value::Table(map) => Ok(map),
            oth => Err(oth),
        }
    }
}
impl<'a> TryFrom<&'a Value> for &'a str {
    type Error = ();

    fn try_from(this: &'a Value) -> Result<&'a str, Self::Error> {
        this.as_str().ok_or(())
    }
}
impl<'a> TryFrom<&'a Value> for &'a String {
    type Error = ();

    fn try_from(this: &'a Value) -> Result<&'a String, Self::Error> {
        this.as_string().ok_or(())
    }
}
impl<'a> TryFrom<&'a Value> for &'a [Value] {
    type Error = ();

    fn try_from(this: &'a Value) -> Result<&'a [Value], Self::Error> {
        this.as_slice().ok_or(())
    }
}
impl<'a> TryFrom<&'a Value> for &'a Vec<Value> {
    type Error = ();

    fn try_from(this: &'a Value) -> Result<&'a Vec<Value>, Self::Error> {
        this.as_array().ok_or(())
    }
}
impl<'a> TryFrom<&'a Value> for &'a Table {
    type Error = ();

    fn try_from(this: &'a Value) -> Result<&'a Table, Self::Error> {
        this.as_table().ok_or(())
    }
}
impl<'a> TryFrom<&'a mut Value> for &'a mut Table {
    type Error = &'a mut Value;

    fn try_from(this: &'a mut Value) -> Result<&'a mut Table, Self::Error> {
        if let Value::Table(table) = this {
            Ok(table)
        } else {
            Err(this)
        }
    }
}
impl<'a> TryFrom<&'a mut Value> for &'a mut Vec<Value> {
    type Error = &'a mut Value;

    fn try_from(this: &'a mut Value) -> Result<&'a mut Vec<Value>, Self::Error> {
        if let Value::Array(arr) = this {
            Ok(arr)
        } else {
            Err(this)
        }
    }
}
impl<'a> TryFrom<&'a mut Value> for &'a mut [Value] {
    type Error = &'a mut Value;

    fn try_from(this: &'a mut Value) -> Result<&'a mut [Value], Self::Error> {
        if let Value::Array(arr) = this {
            Ok(&mut arr[..])
        } else {
            Err(this)
        }
    }
}
impl<'a> TryFrom<&'a mut Value> for &'a mut String {
    type Error = &'a mut Value;

    fn try_from(this: &'a mut Value) -> Result<&'a mut String, Self::Error> {
        if let Value::String(str) = this {
            Ok(str)
        } else {
            Err(this)
        }
    }
}
macro_rules! impl_try_from_parse {
    (@impl $err:ty = $($num:ty),+) => {
        $(
            impl TryFrom<&'_ Value> for $num {
                type Error = Option<$err>;

                /// use [`FromStr`]
                ///
                /// - Return [`None`] indicates type mismatch,
                /// - Return [`Some`] indicates type matched and parse error
                ///
                /// [`FromStr`]: core::str::FromStr
                fn try_from(this: &'_ Value) -> Result<$num, Self::Error> {
                    this.as_str()
                        .ok_or(None)
                        .and_then(|s| s.parse()
                            .map_err(Some))
                }
            }
        )+
    };
    ($($err:ty = $($num:ty),+ $(,)?);* $(;)?) => {
        $(
            impl_try_from_parse!(@impl $err = $($num),+);
        )*
    };
}
impl_try_from_parse! {
    num::ParseIntError      = i8, i16, i32, i64, i128, isize;
    num::ParseIntError      = u8, u16, u32, u64, u128, usize;
    num::ParseFloatError    = f32, f64;
    str::ParseBoolError     = bool;
    char::ParseCharError    = char;
}
