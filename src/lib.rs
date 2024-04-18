#![doc = include_str!("../README.md")]

#[cfg(feature = "cli-utils")]
pub mod cli_utils;

pub mod parser;
mod serialize;

pub use serialize::SYMLSerialize;

use linked_hash_map::LinkedHashMap;

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

    pub fn as_array(&self) -> Option<&Vec<Self>> {
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
impl TryInto<String> for Value {
    type Error = Value;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Value::String(s) => Ok(s),
            oth => Err(oth),
        }
    }
}
impl TryInto<Vec<Self>> for Value {
    type Error = Value;

    fn try_into(self) -> Result<Vec<Value>, Value> {
        match self {
            Value::Array(arr) => Ok(arr),
            oth => Err(oth),
        }
    }
}
impl TryInto<Table> for Value {
    type Error = Value;

    fn try_into(self) -> Result<Table, Value> {
        match self {
            Value::Table(map) => Ok(map),
            oth => Err(oth),
        }
    }
}
