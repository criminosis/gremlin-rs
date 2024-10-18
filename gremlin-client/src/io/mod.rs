#[macro_use]
mod macros;
mod graph_binary_v1;
mod serializer_v2;
mod serializer_v3;

use crate::conversion::ToGValue;
use crate::message::{Response, RequestIdV2};
use crate::process::traversal::{Bytecode, Order, Scope};
use crate::structure::{Cardinality, Direction, GValue, Merge, T};
use serde_json::{json, Map, Value};
use uuid::Uuid;
use std::collections::HashMap;
use std::convert::TryInto;
use std::f64::consts::E;
use std::string::ToString;

use crate::{GKey, GremlinError, GremlinResult, Message, io::graph_binary_v1::GraphBinaryV1Serde};

#[derive(Debug, Clone)]
pub enum IoProtocol {
    GraphSONV2,
    GraphSONV3,
    GraphBinaryV1,
}

impl IoProtocol {
    //TODO maybe we could remove pub from read/write?
    pub fn read(&self, value: &Value) -> GremlinResult<Option<GValue>> {
        if let Value::Null = value {
            return Ok(None);
        }
        match self {
            IoProtocol::GraphSONV2 => serializer_v2::deserializer_v2(value).map(Some),
            IoProtocol::GraphSONV3 => serializer_v3::deserializer_v3(value).map(Some),
            IoProtocol::GraphBinaryV1 => todo!(),
        }
    }

    pub fn write(&self, value: &GValue) -> GremlinResult<Value> {
        match self {
            IoProtocol::GraphSONV2 | IoProtocol::GraphSONV3 => self.write_graphson(value),
            IoProtocol::GraphBinaryV1 => todo!(),
        }
    }

    pub fn read_response(&self, response: &[u8]) -> GremlinResult<Response>{
        match self {
            IoProtocol::GraphSONV2 | IoProtocol::GraphSONV3 => serde_json::from_slice(&response).map_err(GremlinError::from),
            IoProtocol::GraphBinaryV1 => todo!()
        }
    }

    pub fn build_eval_message(&self, args: HashMap<String, GValue>) -> GremlinResult<Vec<u8>>{
        let op = String::from("eval");
        let processor = String::default();
        let content_type = self.content_type();

        match self {
            IoProtocol::GraphSONV2 | IoProtocol::GraphSONV3 => {
                let args = self.write(&GValue::from(args))?;
                let message = match self {
                    IoProtocol::GraphSONV2 => Message::V2 {
                    request_id: RequestIdV2 {
                        id_type: "g:UUID".to_string(),
                        value: Uuid::new_v4(),
                    },
                    op,
                    processor,
                    args,
                }, 
                IoProtocol::GraphSONV3 => {
                    Message::V3 { request_id: Uuid::new_v4(), op, processor, args}
                }
                _ => panic!("Invalid branch")
            };

                let msg = serde_json::to_string(&message).map_err(GremlinError::from)?;
                let payload = String::from("") + content_type + &msg;
                let mut binary = payload.into_bytes();
                binary.insert(0, content_type.len() as u8);
                Ok(binary)
            }
            IoProtocol::GraphBinaryV1 => {
                let mut message_bytes: Vec<u8> = Vec::new();
                //Need to write header first, its length is a Byte not a Int
                let header = String::from(content_type);
                let header_length: u8 = header.len().try_into().expect("Header length should fit in u8");
                message_bytes.push(header_length);
                message_bytes.extend_from_slice(header.as_bytes());

                //Version byte
                message_bytes.push(0x81);

                //Request Id
                Uuid::new_v4().to_be_bytes(&mut message_bytes)?;

                //Op
                op.to_be_bytes(&mut message_bytes)?;

                //Processor
                processor.to_be_bytes(&mut message_bytes)?;

                //Args
                (&GValue::from(args)).to_be_bytes(&mut message_bytes)?;
                Ok(message_bytes)
            }
        }
    }

    pub fn build_traversal_message(&self, aliases: HashMap<String, GValue>, bytecode: &Bytecode) -> GremlinResult<Vec<u8>> {
        let mut args = HashMap::new();
        args.insert(String::from("gremlin"), GValue::Bytecode(bytecode.clone()));
        args.insert(String::from("aliases"), GValue::from(aliases));
        let content_type = self.content_type();
    
        match self {
            IoProtocol::GraphSONV2 | IoProtocol::GraphSONV3 => {
                let args = GValue::from(args);
                //TODO this should be calling something more congruent with the graphbinary side
                let args = self.write(&args)?;
                let message =serde_json::to_string(&Message::V3 {
                    request_id: Uuid::new_v4(),
                    op: String::from("bytecode"),
                    processor: String::from("traversal"),
                    args,
                }).map_err(GremlinError::from)?;
                
                let payload = String::from("") + content_type + &message;
                let mut binary = payload.into_bytes();
                binary.insert(0, content_type.len() as u8);
                Ok(binary)
            }
            IoProtocol::GraphBinaryV1 => {
                let mut message_bytes: Vec<u8> = Vec::new();
                //Need to write header first, its length is a Byte not a Int
                let header = String::from(content_type);
                let header_length: u8 = header.len().try_into().expect("Header length should fit in u8");
                message_bytes.push(header_length);
                message_bytes.extend_from_slice(header.as_bytes());

                //Version byte
                message_bytes.push(0x81);

                //Request Id
                Uuid::new_v4().to_be_bytes(&mut message_bytes)?;

                //Op
                String::from("bytecode").to_be_bytes(&mut message_bytes)?;

                //Processor
                String::from("traversal").to_be_bytes(&mut message_bytes)?;

                //Args
                args.to_be_bytes(&mut message_bytes)?;
                Ok(message_bytes)
            }
        }
    }

    //TODO we can probably generalize this
    // pub fn generate_traversal_message(
    //     &self,
    //     aliases: HashMap<String, GValue>,
    //     bytecode: &Bytecode,
    // ) -> GremlinResult<Message<serde_json::Value>> {
    //     let mut args = HashMap::new();

    //     args.insert(String::from("gremlin"), GValue::Bytecode(bytecode.clone()));

    //     // let aliases = self
    //     //     .alias
    //     //     .clone()
    //     //     .or_else(|| Some(String::from("g")))
    //     //     .map(|s| {
    //     //         let mut map = HashMap::new();
    //     //         map.insert(String::from("g"), GValue::String(s));
    //     //         map
    //     //     })
    //     //     .unwrap_or_else(HashMap::new);

    //     args.insert(String::from("aliases"), GValue::from(aliases));

    //     let args = self.write(&GValue::from(args))?;

    //     Ok(message_with_args(
    //         String::from("bytecode"),
    //         String::from("traversal"),
    //         args,
    //     ))
    // }

    fn write_graphson(&self, value: &GValue) -> GremlinResult<Value> {
        match (self, value) {
            (_, GValue::Double(d)) => Ok(json!({
                "@type" : "g:Double",
                "@value" : d
            })),
            (_, GValue::Float(f)) => Ok(json!({
                "@type" : "g:Float",
                "@value" : f
            })),
            (_, GValue::Int32(i)) => Ok(json!({
                "@type" : "g:Int32",
                "@value" : i
            })),
            (_, GValue::Int64(i)) => Ok(json!({
                "@type" : "g:Int64",
                "@value" : i
            })),
            (_, GValue::String(s)) => Ok(Value::String(s.clone())),
            (_, GValue::Uuid(s)) => Ok(json!({
                "@type" : "g:UUID",
                "@value" : s.to_string()
            })),
            (_, GValue::Date(d)) => Ok(json!({
                "@type" : "g:Date",
                "@value" : d.timestamp_millis()
            })),
            (IoProtocol::GraphSONV2, GValue::List(d)) => {
                let elements: GremlinResult<Vec<Value>> = d.iter().map(|e| self.write(e)).collect();
                Ok(json!(elements?))
            }
            (IoProtocol::GraphSONV3, GValue::List(d)) => {
                let elements: GremlinResult<Vec<Value>> = d.iter().map(|e| self.write(e)).collect();
                Ok(json!({
                    "@type" : "g:List",
                    "@value" : elements?
                }))
            }
            (_, GValue::P(p)) => Ok(json!({
                "@type" : "g:P",
                "@value" : {
                    "predicate" : p.operator(),
                    "value" : self.write(p.value())?
                }
            })),
            (_, GValue::Bytecode(code)) => {
                let steps: GremlinResult<Vec<Value>> = code
                    .steps()
                    .iter()
                    .map(|m| {
                        let mut instruction = vec![];
                        instruction.push(Value::String(m.operator().clone()));

                        let arguments: GremlinResult<Vec<Value>> =
                            m.args().iter().map(|a| self.write(a)).collect();

                        instruction.extend(arguments?);
                        Ok(Value::Array(instruction))
                    })
                    .collect();

                let sources: GremlinResult<Vec<Value>> = code
                    .sources()
                    .iter()
                    .map(|m| {
                        let mut instruction = vec![];
                        instruction.push(Value::String(m.operator().clone()));

                        let arguments: GremlinResult<Vec<Value>> =
                            m.args().iter().map(|a| self.write(a)).collect();

                        instruction.extend(arguments?);
                        Ok(Value::Array(instruction))
                    })
                    .collect();
                Ok(json!({
                    "@type" : "g:Bytecode",
                    "@value" : {
                        "step" : steps?,
                        "source" : sources?
                    }
                }))
            }
            (_, GValue::Vertex(v)) => {
                let id = self.write(&v.id().to_gvalue())?;
                Ok(json!({
                    "@type" : "g:Vertex",
                    "@value" : {
                        "id" :  id,
                    }
                }))
            }
            (IoProtocol::GraphSONV2, GValue::Map(map)) => {
                let mut params = Map::new();

                for (k, v) in map.iter() {
                    params.insert(
                        self.write(&k.clone().into())?
                            .as_str()
                            .ok_or_else(|| {
                                GremlinError::Generic("Non-string key value.".to_string())
                            })?
                            .to_string(),
                        self.write(&v)?,
                    );
                }

                Ok(json!(params))
            }
            (IoProtocol::GraphSONV3, GValue::Map(map)) => {
                let mut params = vec![];

                for (k, v) in map.iter() {
                    params.push(self.write(&k.clone().into())?);
                    params.push(self.write(&v)?);
                }

                Ok(json!({
                    "@type" : "g:Map",
                    "@value" : params
                }))
            }
            (_, GValue::T(t)) => {
                let v = match t {
                    T::Id => "id",
                    T::Key => "key",
                    T::Label => "label",
                    T::Value => "value",
                };

                Ok(json!({
                    "@type" : "g:T",
                    "@value" : v
                }))
            }
            (_, GValue::Scope(s)) => {
                let v = match s {
                    Scope::Global => "global",
                    Scope::Local => "local",
                };

                Ok(json!({
                    "@type" : "g:Scope",
                    "@value" : v
                }))
            }

            (_, GValue::Order(s)) => {
                let v = match s {
                    Order::Asc => "asc",
                    Order::Desc => "desc",
                    Order::Shuffle => "shuffle",
                };

                Ok(json!({
                    "@type" : "g:Order",
                    "@value" : v
                }))
            }
            (_, GValue::Bool(b)) => {
                let json_string = match b {
                    true => "true",
                    false => "false",
                };
                Ok(serde_json::from_str(json_string).unwrap())
            }
            (_, GValue::TextP(text_p)) => Ok(json!({
                "@type" : "g:TextP",
                "@value" : {
                    "predicate" : text_p.operator(),
                    "value" : self.write(text_p.value())?
                }
            })),
            (_, GValue::Pop(pop)) => Ok(json!({
                "@type": "g:Pop",
                "@value": *pop.to_string(),
            })),
            (_, GValue::Cardinality(cardinality)) => {
                let v = match cardinality {
                    Cardinality::List => "list",
                    Cardinality::Single => "single",
                    Cardinality::Set => "set",
                };
                Ok(json!({
                    "@type" : "g:Cardinality",
                    "@value" : v
                }))
            }
            (_, GValue::Merge(merge)) => {
                let merge_option = match merge {
                    Merge::OnCreate => "onCreate",
                    Merge::OnMatch => "onMatch",
                    Merge::OutV => "outV",
                    Merge::InV => "inV",
                };
                Ok(json!({
                    "@type" : "g:Merge",
                    "@value" : merge_option
                }))
            }
            (_, GValue::Direction(direction)) => {
                let direction = match direction {
                    Direction::Out | Direction::From => "OUT",
                    Direction::In | Direction::To => "IN",
                };
                Ok(json!({
                    "@type" : "g:Direction",
                    "@value" : direction,
                }))
            }
            (_, GValue::Column(column)) => {
                let column = match column {
                    crate::structure::Column::Keys => "keys",
                    crate::structure::Column::Values => "values",
                };
                Ok(json!({
                    "@type" : "g:Column",
                    "@value" : column,
                }))
            }
            (_, _) => panic!("Type {:?} not supported.", value),
        }
    }

    pub fn content_type(&self) -> &str {
        match self {
            IoProtocol::GraphSONV2 => "application/vnd.gremlin-v2.0+json",
            IoProtocol::GraphSONV3 => "application/vnd.gremlin-v3.0+json",
            IoProtocol::GraphBinaryV1 => "application/vnd.graphbinary-v1.0",
        }
    }
}
