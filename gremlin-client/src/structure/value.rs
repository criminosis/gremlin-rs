use crate::conversion::{BorrowFromGValue, FromGValue};
use crate::process::traversal::{Bytecode, Order, Scope};
use crate::structure::traverser::Traverser;
use crate::structure::{
    label::LabelType, Cardinality, Edge, GKey, IntermediateRepr, List, Map, Metric, Path, Property,
    Set, Token, TraversalExplanation, TraversalMetrics, Vertex, VertexProperty,
};
use crate::structure::{Pop, TextP, P, T};
use crate::{GremlinError, GremlinResult};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
pub type Date = chrono::DateTime<chrono::offset::Utc>;
use std::borrow::Borrow;
use std::convert::TryInto;
use std::hash::Hash;
/// Represent possible values coming from the [Gremlin Server](http://tinkerpop.apache.org/docs/3.4.0/dev/io/)
#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Clone)]
pub enum GValue {
    Null,
    Vertex(Vertex),
    Edge(Edge),
    VertexProperty(VertexProperty),
    Property(Property),
    Uuid(uuid::Uuid),
    Int32(i32),
    Int64(i64),
    Float(f32),
    Double(f64),
    Date(Date),
    List(List),
    Set(Set),
    Map(Map),
    Token(Token),
    String(String),
    Path(Path),
    TraversalMetrics(TraversalMetrics),
    Metric(Metric),
    TraversalExplanation(TraversalExplanation),
    IntermediateRepr(IntermediateRepr),
    P(P),
    T(T),
    Bytecode(Bytecode),
    Traverser(Traverser),
    Scope(Scope),
    Order(Order),
    Bool(bool),
    TextP(TextP),
    Pop(Pop),
    Cardinality(Cardinality),
}

impl GValue {
    pub fn take<T>(self) -> GremlinResult<T>
    where
        T: FromGValue,
    {
        T::from_gvalue(self)
    }

    pub fn get<'a, T>(&'a self) -> GremlinResult<&'a T>
    where
        T: BorrowFromGValue,
    {
        T::from_gvalue(self)
    }
}

impl From<Date> for GValue {
    fn from(val: Date) -> Self {
        GValue::Date(val)
    }
}

impl From<String> for GValue {
    fn from(val: String) -> Self {
        GValue::String(val)
    }
}

impl From<&String> for GValue {
    fn from(val: &String) -> Self {
        GValue::String(val.clone())
    }
}

impl From<i32> for GValue {
    fn from(val: i32) -> Self {
        GValue::Int32(val)
    }
}

impl From<i64> for GValue {
    fn from(val: i64) -> Self {
        GValue::Int64(val)
    }
}

impl From<f32> for GValue {
    fn from(val: f32) -> Self {
        GValue::Float(val)
    }
}
impl From<f64> for GValue {
    fn from(val: f64) -> Self {
        GValue::Double(val)
    }
}

impl<'a> From<&'a str> for GValue {
    fn from(val: &'a str) -> Self {
        GValue::String(String::from(val))
    }
}

impl From<Vertex> for GValue {
    fn from(val: Vertex) -> Self {
        GValue::Vertex(val)
    }
}

impl From<&Vertex> for GValue {
    fn from(val: &Vertex) -> Self {
        GValue::Vertex(val.clone())
    }
}

impl From<Path> for GValue {
    fn from(val: Path) -> Self {
        GValue::Path(val)
    }
}
impl From<Edge> for GValue {
    fn from(val: Edge) -> Self {
        GValue::Edge(val)
    }
}

impl From<VertexProperty> for GValue {
    fn from(val: VertexProperty) -> Self {
        GValue::VertexProperty(val)
    }
}

impl From<Traverser> for GValue {
    fn from(val: Traverser) -> Self {
        GValue::Traverser(val)
    }
}
impl From<TraversalMetrics> for GValue {
    fn from(val: TraversalMetrics) -> Self {
        GValue::TraversalMetrics(val)
    }
}

impl From<TraversalExplanation> for GValue {
    fn from(val: TraversalExplanation) -> Self {
        GValue::TraversalExplanation(val)
    }
}

impl From<Metric> for GValue {
    fn from(val: Metric) -> Self {
        GValue::Metric(val)
    }
}

impl From<Property> for GValue {
    fn from(val: Property) -> Self {
        GValue::Property(val)
    }
}

impl From<Scope> for GValue {
    fn from(val: Scope) -> Self {
        GValue::Scope(val)
    }
}

impl From<Order> for GValue {
    fn from(val: Order) -> Self {
        GValue::Order(val)
    }
}
impl From<Token> for GValue {
    fn from(val: Token) -> Self {
        GValue::Token(val)
    }
}

impl From<HashMap<String, GValue>> for GValue {
    fn from(val: HashMap<String, GValue>) -> Self {
        GValue::Map(Map::from(val))
    }
}

impl From<HashMap<GKey, GValue>> for GValue {
    fn from(val: HashMap<GKey, GValue>) -> Self {
        GValue::Map(Map::from(val))
    }
}

impl From<BTreeMap<String, GValue>> for GValue {
    fn from(val: BTreeMap<String, GValue>) -> Self {
        GValue::Map(Map::from(val))
    }
}

impl From<Vec<GValue>> for GValue {
    fn from(val: Vec<GValue>) -> Self {
        GValue::List(List::new(val))
    }
}

impl From<GValue> for Vec<GValue> {
    fn from(val: GValue) -> Self {
        vec![val]
    }
}

impl From<GValue> for VecDeque<GValue> {
    fn from(val: GValue) -> Self {
        match val {
            GValue::List(l) => VecDeque::from(l.take()),
            GValue::Set(l) => VecDeque::from(l.take()),
            _ => VecDeque::from(vec![val]),
        }
    }
}

impl From<GKey> for GValue {
    fn from(val: GKey) -> Self {
        match val {
            GKey::String(s) => GValue::String(s),
            GKey::Token(s) => GValue::String(s.value().clone()),
            GKey::Vertex(v) => GValue::Vertex(v),
            GKey::Edge(v) => GValue::Edge(v),
        }
    }
}

impl From<P> for GValue {
    fn from(val: P) -> GValue {
        GValue::P(val)
    }
}

impl From<TextP> for GValue {
    fn from(val: TextP) -> GValue {
        GValue::TextP(val)
    }
}

impl From<T> for GValue {
    fn from(val: T) -> GValue {
        GValue::T(val)
    }
}

impl From<Bytecode> for GValue {
    fn from(val: Bytecode) -> GValue {
        GValue::Bytecode(val)
    }
}

impl From<bool> for GValue {
    fn from(val: bool) -> GValue {
        GValue::Bool(val)
    }
}

impl From<LabelType> for GValue {
    fn from(val: LabelType) -> GValue {
        match val {
            LabelType::Str(val) => val.into(),
            LabelType::Bool(val) => val.into(),
            LabelType::T(val) => val.into(),
        }
    }
}

impl From<Cardinality> for GValue {
    fn from(val: Cardinality) -> GValue {
        GValue::Cardinality(val)
    }
}

impl From<uuid::Uuid> for GValue {
    fn from(val: uuid::Uuid) -> GValue {
        GValue::Uuid(val)
    }
}

impl std::convert::TryFrom<GValue> for String {
    type Error = crate::GremlinError;

    fn try_from(value: GValue) -> GremlinResult<Self> {
        match value {
            GValue::String(s) => Ok(s),
            GValue::List(s) => from_list(s),
            _ => Err(GremlinError::Cast(format!(
                "Cannot cast {:?} to String",
                value
            ))),
        }
    }
}

impl std::convert::TryFrom<GValue> for i32 {
    type Error = crate::GremlinError;

    fn try_from(value: GValue) -> GremlinResult<Self> {
        match value {
            GValue::Int32(s) => Ok(s),
            GValue::List(s) => from_list(s),
            _ => Err(GremlinError::Cast(format!(
                "Cannot cast {:?} to i32",
                value
            ))),
        }
    }
}

impl std::convert::TryFrom<GValue> for i64 {
    type Error = crate::GremlinError;

    fn try_from(value: GValue) -> GremlinResult<Self> {
        match value {
            GValue::Int64(s) => Ok(s),
            GValue::List(s) => from_list(s),
            _ => Err(GremlinError::Cast(format!(
                "Cannot cast {:?} to i32",
                value
            ))),
        }
    }
}

impl std::convert::TryFrom<GValue> for f32 {
    type Error = crate::GremlinError;

    fn try_from(value: GValue) -> GremlinResult<Self> {
        match value {
            GValue::Float(s) => Ok(s),
            GValue::List(s) => from_list(s),
            _ => Err(GremlinError::Cast(format!(
                "Cannot cast {:?} to f32",
                value
            ))),
        }
    }
}

impl std::convert::TryFrom<GValue> for f64 {
    type Error = crate::GremlinError;

    fn try_from(value: GValue) -> GremlinResult<Self> {
        match value {
            GValue::Double(s) => Ok(s),
            GValue::List(s) => from_list(s),
            _ => Err(GremlinError::Cast(format!(
                "Cannot cast {:?} to f64",
                value
            ))),
        }
    }
}

impl std::convert::TryFrom<GValue> for uuid::Uuid {
    type Error = crate::GremlinError;

    fn try_from(value: GValue) -> GremlinResult<Self> {
        match value {
            GValue::Uuid(uid) => Ok(uid),
            _ => Err(GremlinError::Cast(format!(
                "Cannot cast {:?} to Uuid",
                value
            ))),
        }
    }
}

impl std::convert::TryFrom<GValue> for Date {
    type Error = crate::GremlinError;

    fn try_from(value: GValue) -> GremlinResult<Self> {
        match value {
            GValue::Date(date) => Ok(date),
            _ => Err(GremlinError::Cast(format!(
                "Cannot cast {:?} to DateTime<Utc>",
                value
            ))),
        }
    }
}

impl std::convert::TryFrom<GValue> for bool {
    type Error = crate::GremlinError;

    fn try_from(value: GValue) -> GremlinResult<Self> {
        match value {
            GValue::Bool(val) => Ok(val),
            _ => Err(GremlinError::Cast(format!(
                "Cannot cast {:?} to bool",
                value
            ))),
        }
    }
}

fn from_list<T>(glist: List) -> GremlinResult<T>
where
    T: std::convert::TryFrom<GValue, Error = GremlinError>,
{
    let mut vec = glist.take();

    match vec.len() {
        1 => vec.pop().unwrap().try_into(),
        _ => Err(GremlinError::Cast(String::from(
            "Cannot cast a List to String",
        ))),
    }
}

// Optional

macro_rules! impl_try_from_option {
    ($t:ty) => {
        impl std::convert::TryFrom<GValue> for Option<$t> {
            type Error = crate::GremlinError;

            fn try_from(value: GValue) -> GremlinResult<Self> {
                if let GValue::Null = value {
                    return Ok(None);
                }
                let res = value.try_into()?;
                Ok(res)
            }
        }
    };
}

impl_try_from_option!(String);

fn for_list<T>(glist: &List) -> GremlinResult<Vec<T>>
where
    T: std::convert::TryFrom<GValue, Error = GremlinError>,
{
    glist
        .iter()
        .map(|x| x.clone().try_into())
        .collect::<GremlinResult<Vec<T>>>()
}

fn for_list_to_set<T>(glist: &List) -> GremlinResult<HashSet<T>>
where
    T: std::convert::TryFrom<GValue, Error = GremlinError> + Hash + Eq,
{
    glist
        .iter()
        .map(|x| x.clone().try_into())
        .collect::<GremlinResult<HashSet<T>>>()
}

fn for_set<T>(gset: &Set) -> GremlinResult<HashSet<T>>
where
    T: std::convert::TryFrom<GValue, Error = GremlinError> + Hash + Eq,
{
    gset.iter()
        .map(|x| x.clone().try_into())
        .collect::<GremlinResult<HashSet<T>>>()
}

macro_rules! impl_try_from_set {
    ($t:ty) => {
        //Ideally this would be handled in conversion.rs but because the GValue::Set holds a Vec
        //we handle converting it here
        impl FromGValue for HashSet<$t> {
            fn from_gvalue(value: GValue) -> GremlinResult<Self> {
                match value {
                    GValue::List(s) => for_list_to_set(&s),
                    GValue::Set(s) => for_set(&s),
                    _ => Err(GremlinError::Cast(format!(
                        "Cannot cast {:?} to HashSet",
                        value
                    ))),
                }
            }
        }

        impl std::convert::TryFrom<GValue> for HashSet<$t> {
            type Error = crate::GremlinError;

            fn try_from(value: GValue) -> GremlinResult<Self> {
                match value {
                    GValue::List(s) => for_list_to_set(&s),
                    GValue::Set(s) => for_set(&s),
                    _ => Err(GremlinError::Cast(format!(
                        "Cannot cast {:?} to HashSet",
                        value
                    ))),
                }
            }
        }

        impl std::convert::TryFrom<&GValue> for HashSet<$t> {
            type Error = crate::GremlinError;

            fn try_from(value: &GValue) -> GremlinResult<Self> {
                match value {
                    GValue::List(s) => for_list_to_set(s),
                    GValue::Set(s) => for_set(s),
                    _ => Err(GremlinError::Cast(format!(
                        "Cannot cast {:?} to HashSet",
                        value
                    ))),
                }
            }
        }
    };
}

impl_try_from_set!(String);
impl_try_from_set!(i32);
impl_try_from_set!(i64);
impl_try_from_set!(Date);
impl_try_from_set!(uuid::Uuid);
impl_try_from_set!(bool);
//floats do not conform to the Eq or Hash traits
// impl_try_from_set!(f32);
// impl_try_from_set!(f64);

macro_rules! impl_try_from_list {
    ($t:ty) => {
        impl std::convert::TryFrom<GValue> for Vec<$t> {
            type Error = crate::GremlinError;

            fn try_from(value: GValue) -> GremlinResult<Self> {
                match value {
                    GValue::List(s) => for_list(&s),
                    _ => Err(GremlinError::Cast(format!(
                        "Cannot cast {:?} to Vec",
                        value
                    ))),
                }
            }
        }

        impl std::convert::TryFrom<&GValue> for Vec<$t> {
            type Error = crate::GremlinError;

            fn try_from(value: &GValue) -> GremlinResult<Self> {
                match value {
                    GValue::List(s) => for_list(s),
                    _ => Err(GremlinError::Cast(format!(
                        "Cannot cast {:?} to Vec",
                        value
                    ))),
                }
            }
        }
    };
}

impl_try_from_list!(String);
impl_try_from_list!(i32);
impl_try_from_list!(i64);
impl_try_from_list!(f32);
impl_try_from_list!(f64);
impl_try_from_list!(Date);
impl_try_from_list!(uuid::Uuid);
impl_try_from_list!(bool);
