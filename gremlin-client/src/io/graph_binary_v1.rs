use std::{
    collections::HashMap,
    convert::TryInto,
};

use uuid::Uuid;

use crate::{process::traversal::Instruction, GKey, GValue, GremlinError, GremlinResult};

use super::IoProtocol;

pub(crate) struct RequestMessage<'a, 'b> {
    pub(crate) request_id: Uuid,
    pub(crate) op: &'a str,
    pub(crate) processor: &'b str,
    pub(crate) args: HashMap<String, GValue>,
}

impl<'a, 'b> GraphBinaryV1Ser for RequestMessage<'a, 'b> {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        //Need to write header first, its length is a Byte not a Int
        let header = IoProtocol::GraphBinaryV1.content_type();
        let header_length: u8 = header
            .len()
            .try_into()
            .expect("Header length should fit in u8");
        buf.push(header_length);
        buf.extend_from_slice(header.as_bytes());

        //Version byte is 0x81 for this version
        buf.push(0x81);

        //Request Id
        self.request_id.to_be_bytes(buf)?;

        //Op
        self.op.to_be_bytes(buf)?;

        //Processor
        self.processor.to_be_bytes(buf)?;

        //Args
        let args_length: i32 = self
            .args
            .len()
            .try_into()
            .map_err(|_| GremlinError::Cast(format!("Args exceeds i32 length limit")))?;
        GraphBinaryV1Ser::to_be_bytes(args_length, buf)?;
        for (k, v) in self.args.into_iter() {
            //Both keys and values need to be fully qualified here, so turn
            //the keys into a GValue
            GValue::from(k).to_be_bytes(buf)?;
            v.to_be_bytes(buf)?;
        }
        Ok(())
    }
}

//https://tinkerpop.apache.org/docs/3.7.2/dev/io/#_data_type_codes
//Each type has a "fully qualified" serialized form usually:  {type_code}{type_info}{value_flag}{value}
//{type_code} is a single unsigned byte representing the type number.
//{type_info} is an optional sequence of bytes providing additional information of the type represented. This is specially useful for representing complex and custom types.
//{value_flag} is a single byte providing information about the value. Flags have the following meaning:
// 0x01 The value is null. When this flag is set, no bytes for {value} will be provided.
//{value} is a sequence of bytes which content is determined by the type.
//All encodings are big-endian.

//However there are occassion when just "the value" is written without the fully qualified form, for example the 4 bytes of a integer without the type_code
//this is usually done in scenarios when the type in unambiguous by schema.

//Generally this is written such that serializing a value wrapped by GValue is taken to mean to write the fully qualified representation
//and serializing just "the value" is done directly upon the underlying value type

fn write_usize_as_i32_be_bytes(val: usize, buf: &mut Vec<u8>) -> GremlinResult<()> {
    let val_i32 = TryInto::<i32>::try_into(val)
        .map_err(|_| GremlinError::Cast(format!("Invalid usize bytes exceed i32")))?;
    GraphBinaryV1Ser::to_be_bytes(val_i32, buf)
}

impl GraphBinaryV1Ser for &GValue {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        match self {
            GValue::Null => {
                //Type code of 0xfe: Unspecified null object
                buf.push(0xfe);
                //Then the null {value_flag} set and no sequence of bytes.
                buf.push(0x01);
            }
            // GValue::Vertex(vertex) => todo!(),
            // GValue::Edge(edge) => todo!(),
            // GValue::VertexProperty(vertex_property) => todo!(),
            // GValue::Property(property) => todo!(),
            // GValue::Uuid(uuid) => todo!(),
            GValue::Int32(value) => {
                //Type code of 0x01
                buf.push(0x01);
                //Empty value flag
                buf.push(0x00);
                //then value bytes
                GraphBinaryV1Ser::to_be_bytes(*value, buf)?;
            }
            // GValue::Int64(_) => todo!(),
            // GValue::Float(_) => todo!(),
            // GValue::Double(_) => todo!(),
            // GValue::Date(date_time) => todo!(),
            // GValue::List(list) => todo!(),
            // GValue::Set(set) => todo!(),
            GValue::Map(map) => {
                //Type code of 0x0a: Map
                buf.push(0x0a);
                // //Empty value flag
                buf.push(0x00);

                //{length} is an Int describing the length of the map.
                write_usize_as_i32_be_bytes(map.len(), buf)?;

                //{item_0}…​{item_n} are the items of the map. {item_i} is sequence of 2 fully qualified typed values one representing the key
                //  and the following representing the value, each composed of {type_code}{type_info}{value_flag}{value}.
                for (k, v) in map.iter() {
                    k.to_be_bytes(buf)?;
                    v.to_be_bytes(buf)?;
                }
            }
            // GValue::Token(token) => todo!(),
            GValue::String(value) => {
                //Type code of 0x03: String
                buf.push(0x03);
                //Empty value flag
                buf.push(0x00);
                //Format: {length}{text_value}
                // {length} is an Int describing the byte length of the text. Length is a positive number or zero to represent the empty string.
                // {text_value} is a sequence of bytes representing the string value in UTF8 encoding.
                GraphBinaryV1Ser::to_be_bytes(value.as_str(), buf)?;
            }
            // GValue::Path(path) => todo!(),
            // GValue::TraversalMetrics(traversal_metrics) => todo!(),
            // GValue::Metric(metric) => todo!(),
            // GValue::TraversalExplanation(traversal_explanation) => todo!(),
            // GValue::IntermediateRepr(intermediate_repr) => todo!(),
            // GValue::P(p) => todo!(),
            // GValue::T(t) => todo!(),
            GValue::Bytecode(code) => {
                //Type code of 0x15: Bytecode
                buf.push(0x15);
                //Empty value flag
                buf.push(0x00);
                //then value bytes
                // {steps_length}{step_0}…​{step_n}{sources_length}{source_0}…​{source_n}
                //{steps_length} is an Int value describing the amount of steps.
                //{step_i} is composed of {name}{values_length}{value_0}…​{value_n}, where:
                //  {name} is a String.
                //  {values_length} is an Int describing the amount values.
                //  {value_i} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value} describing the step argument.

                fn write_instructions(
                    instructions: &Vec<Instruction>,
                    buf: &mut Vec<u8>,
                ) -> GremlinResult<()> {
                    write_usize_as_i32_be_bytes(instructions.len(), buf)?;
                    for instruction in instructions {
                        GraphBinaryV1Ser::to_be_bytes(instruction.operator().as_str(), buf)?;
                        write_usize_as_i32_be_bytes(instruction.args().len(), buf)?;
                        instruction
                            .args()
                            .iter()
                            .try_for_each(|arg| arg.to_be_bytes(buf))?;
                    }
                    Ok(())
                }
                write_instructions(code.steps(), buf)?;
                write_instructions(code.sources(), buf)?;
            }
            // GValue::Traverser(traverser) => todo!(),
            GValue::Scope(scope) => {
                //Type code of 0x1f: Scope
                buf.push(0x1f);
                //Empty value flag
                buf.push(0x00);

                //Format: a fully qualified single String representing the enum value.
                match scope {
                    crate::process::traversal::Scope::Global => {
                        (&GValue::from(String::from("global"))).to_be_bytes(buf)?
                    }
                    crate::process::traversal::Scope::Local => {
                        (&GValue::from(String::from("local"))).to_be_bytes(buf)?
                    }
                }
            }
            // GValue::Order(order) => todo!(),
            // GValue::Bool(_) => todo!(),
            // GValue::TextP(text_p) => todo!(),
            // GValue::Pop(pop) => todo!(),

            // GValue::Cardinality(cardinality) => todo!(),
            // GValue::Merge(merge) => todo!(),
            // GValue::Direction(direction) => todo!(),
            // GValue::Column(column) => todo!(),
            other => unimplemented!("TODO {other:?}"),
        }
        Ok(())
    }
}

impl GraphBinaryV1Ser for &GKey {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        match self {
            GKey::T(t) => todo!(),
            GKey::String(str) => (&GValue::from(str.clone())).to_be_bytes(buf),
            GKey::Token(token) => todo!(),
            GKey::Vertex(vertex) => todo!(),
            GKey::Edge(edge) => todo!(),
            GKey::Direction(direction) => todo!(),
        }
    }
}
pub trait GraphBinaryV1Ser: Sized {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()>;
}

pub trait GraphBinaryV1Deser: Sized {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self>;
}

impl GraphBinaryV1Deser for GValue {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        let data_code = bytes
            .next()
            .ok_or_else(|| GremlinError::Cast(format!("Invalid bytes no data code byte")))?;
        match data_code {
            // GValue::Null => {
            //     buf.reserve_exact(2);
            //     //Type code of 0xfe: Unspecified null object
            //     buf.push(0xfe);
            //     //Then the null {value_flag} set and no sequence of bytes.
            //     buf.push(0x01);
            // }
            // GValue::Vertex(vertex) => todo!(),
            // GValue::Edge(edge) => todo!(),
            // GValue::VertexProperty(vertex_property) => todo!(),
            // GValue::Property(property) => todo!(),
            // GValue::Uuid(uuid) => todo!(),
            0x01 => {
                //Type code of 0x01: Integer
                //Check null flag
                match bytes.next() {
                    Some(0x00) => {
                        //We've got a value to parse
                        GraphBinaryV1Deser::from_be_bytes(bytes).map(GValue::Int32)
                    }
                    Some(0x01) => Ok(GValue::Null),
                    _ => Err(GremlinError::Cast(format!("Invalid bytes into i32"))),
                }
            }
            // GValue::Int64(_) => todo!(),
            // GValue::Float(_) => todo!(),
            // GValue::Double(_) => todo!(),
            // GValue::Date(date_time) => todo!(),
            // GValue::List(list) => todo!(),
            // GValue::Set(set) => todo!(),
            // GValue::Map(map) => todo!(),
            // GValue::Token(token) => todo!(),
            0x03 => {
                //Type code of 0x03: String
                match bytes.next() {
                    Some(0x00) => {
                        //We've got a value to parse
                        GraphBinaryV1Deser::from_be_bytes(bytes).map(GValue::String)
                    }
                    Some(0x01) => Ok(GValue::Null),
                    _ => Err(GremlinError::Cast(format!("Invalid bytes into String"))),
                }

                // GValue::String(value) => {
                //
                //     //Empty value flag
                //     //Format: {length}{text_value}
                //     // {length} is an Int describing the byte length of the text. Length is a positive number or zero to represent the empty string.
                //     // {text_value} is a sequence of bytes representing the string value in UTF8 encoding.
            }
            // GValue::Path(path) => todo!(),
            // GValue::TraversalMetrics(traversal_metrics) => todo!(),
            // GValue::Metric(metric) => todo!(),
            // GValue::TraversalExplanation(traversal_explanation) => todo!(),
            // GValue::IntermediateRepr(intermediate_repr) => todo!(),
            // GValue::P(p) => todo!(),
            // GValue::T(t) => todo!(),
            // GValue::Bytecode(code) => {
            //     //Type code of 0x15: Bytecode
            //     buf.push(0x15);
            //     //Empty value flag
            //     buf.push(0x00);
            //     //then value bytes
            //     // {steps_length}{step_0}…​{step_n}{sources_length}{source_0}…​{source_n}
            //     //{steps_length} is an Int value describing the amount of steps.
            //     let steps_length: i32 = code.steps().len().try_into().expect("Number of steps should fit in i32");
            //     buf.extend_from_slice(serialize(&GValue::Int32(steps_length)));

            //     //{step_i} is composed of {name}{values_length}{value_0}…​{value_n}, where:
            //     //  {name} is a String.
            //     //  {values_length} is an Int describing the amount values.
            //     //  {value_i} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value} describing the step argument.

            //     let steps: GremlinResult<Vec<Value>> = code
            //             .steps()
            //             .iter()
            //             .map(|m| {
            //                 let mut instruction = vec![];
            //                 instruction.push(Value::String(m.operator().clone()));

            //                 let arguments: GremlinResult<Vec<Value>> =
            //                     m.args().iter().map(|a| self.write(a)).collect();

            //                 instruction.extend(arguments?);
            //                 Ok(Value::Array(instruction))
            //             })
            //             .collect();

            //         let sources: GremlinResult<Vec<Value>> = code
            //             .sources()
            //             .iter()
            //             .map(|m| {
            //                 let mut instruction = vec![];
            //                 instruction.push(Value::String(m.operator().clone()));

            //                 let arguments: GremlinResult<Vec<Value>> =
            //                     m.args().iter().map(|a| self.write(a)).collect();

            //                 instruction.extend(arguments?);
            //                 Ok(Value::Array(instruction))
            //             })
            //             .collect();
            // }
            // GValue::Traverser(traverser) => todo!(),
            // GValue::Scope(scope) => todo!(),
            // GValue::Order(order) => todo!(),
            // GValue::Bool(_) => todo!(),
            // GValue::TextP(text_p) => todo!(),
            // GValue::Pop(pop) => todo!(),
            // GValue::Cardinality(cardinality) => todo!(),
            // GValue::Merge(merge) => todo!(),
            // GValue::Direction(direction) => todo!(),
            // GValue::Column(column) => todo!(),
            _ => unimplemented!("TODO"),
        }
    }
}

impl GraphBinaryV1Ser for &str {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        let length: i32 = self
            .len()
            .try_into()
            .map_err(|_| GremlinError::Cast(format!("String length exceeds i32")))?;
        GraphBinaryV1Ser::to_be_bytes(length, buf)?;
        buf.extend_from_slice(self.as_bytes());
        Ok(())
    }
}

impl GraphBinaryV1Deser for String {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        let string_bytes_length: i32 = GraphBinaryV1Deser::from_be_bytes(bytes)
            .map_err(|_| GremlinError::Cast(format!("Invalid bytes for String length")))?;
        let string_bytes_length = string_bytes_length
            .try_into()
            .map_err(|_| GremlinError::Cast(format!("String length did not fit into usize")))?;
        let string_value_bytes: Vec<u8> = bytes.take(string_bytes_length).cloned().collect();
        if string_value_bytes.len() < string_bytes_length {
            return Err(GremlinError::Cast(format!(
                "Missing bytes for String value"
            )));
        }
        String::from_utf8(string_value_bytes)
            .map_err(|_| GremlinError::Cast(format!("Invalid bytes for String value")))
    }
}

impl GraphBinaryV1Ser for i32 {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        buf.extend_from_slice(&self.to_be_bytes());
        Ok(())
    }
}

impl GraphBinaryV1Deser for i32 {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        bytes
            .take(4)
            .cloned()
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| GremlinError::Cast(format!("Invalid bytes into i32")))
            .map(i32::from_be_bytes)
    }
}

impl GraphBinaryV1Ser for Uuid {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        buf.extend_from_slice(self.as_bytes().as_slice());
        Ok(())
    }
}

impl GraphBinaryV1Deser for Uuid {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        bytes
            .take(16)
            .cloned()
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| GremlinError::Cast(format!("Invalid bytes into Uuid")))
            .map(Uuid::from_bytes)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    //Non-Null i32 Integer (01 00)
    #[case::int_1(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x01], GValue::Int32(1))]
    #[case::int_256(&[0x01, 0x00, 0x00, 0x00, 0x01, 0x00], GValue::Int32(256))]
    #[case::int_257(&[0x01, 0x00, 0x00, 0x00, 0x01, 0x01], GValue::Int32(257))]
    #[case::int_neg_1(&[0x01, 0x00, 0xFF, 0xFF, 0xFF, 0xFF], GValue::Int32(-1))]
    #[case::int_neg_2(&[0x01, 0x00, 0xFF, 0xFF, 0xFF, 0xFE], GValue::Int32(-2))]
    //Non-Null Strings (03 00)
    #[case::str_abc(&[0x03, 0x00, 0x00, 0x00, 0x00, 0x03, 0x61, 0x62, 0x63], GValue::String("abc".into()))]
    #[case::str_abcd(&[0x03, 0x00, 0x00, 0x00, 0x00, 0x04, 0x61, 0x62, 0x63, 0x64], GValue::String("abcd".into()))]
    #[case::empty_str(&[0x03, 0x00, 0x00, 0x00, 0x00, 0x00], GValue::String("".into()))]
    fn serde(#[case] expected_serialized: &[u8], #[case] expected: GValue) {
        let mut serialized = Vec::new();
        (&expected)
            .to_be_bytes(&mut serialized)
            .expect("Shouldn't fail parsing");
        assert_eq!(serialized, expected_serialized);
        let deserialized: GValue = GraphBinaryV1Deser::from_be_bytes(&mut serialized.iter())
            .expect("Shouldn't fail parsing");
        assert_eq!(deserialized, expected);
    }

    #[rstest]
    #[case::too_few_bytes( &[0x01, 0x00, 0x00, 0x00, 0x00])]
    fn serde_int32_invalid_bytes(#[case] bytes: &[u8]) {
        <GValue as GraphBinaryV1Deser>::from_be_bytes(&mut bytes.iter())
            .expect_err("Should have failed due invalid bytes");
    }
}
