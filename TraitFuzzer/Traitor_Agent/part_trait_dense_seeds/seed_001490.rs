#![feature(type_alias_impl_trait)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};
use std::option;

#[derive(Clone, Debug)]
enum Json {
    I64(i64),
    U64(u64),
    F64(f64),
    String(String),
    Boolean(bool),
    Array(Array),
    Object(Object),
    Null,
}

type Array = Vec<Json>;
type Object = BTreeMap<String, Json>;

enum object {
    bool_value(bool),
    int_value(i64),
}

trait Lookup {
    fn lookup(&self, key: String, default: String) -> String;
}

impl Lookup for Object {
    fn lookup(&self, key: String, default: String) -> String {
        match self.get(&key) {
            option::Option::Some(&Json::String(ref s)) => {
                s.to_string()
            }
            option::Option::Some(value) => {
                println!("{} was expected to be a string but is a {:?}", key, value);
                default
            }
            option::Option::None => {
                default
            }
        }
    }
}

trait AddInterface {
    type ResultType;
    fn add_interface(&self, managed_ip: String, data: Json) -> Self::ResultType;
}

impl AddInterface for isize {
    type ResultType = (String, object);
    fn add_interface(&self, managed_ip: String, data: Json) -> Self::ResultType {
        match &data {
            &Json::Object(ref interface) => {
                let name = interface.lookup("ifDescr".to_string(), "".to_string());
                let label = format!("{}-{}", managed_ip, name);

                (label, object::bool_value(false))
            }
            _ => {
                println!("Expected dict for {} interfaces, found {:?}", managed_ip, data);
                ("gnos:missing-interface".to_string(), object::bool_value(true))
            }
        }
    }
}

trait AddInterfaces {
    type ResultType;
    fn add_interfaces(&self, managed_ip: String, device: HashMap<String, Json>) -> Self::ResultType;
}

impl AddInterfaces for isize {
    type ResultType = Vec<(String, object)>;
    fn add_interfaces(&self, managed_ip: String, device: HashMap<String, Json>) -> Self::ResultType {
        match device["interfaces"] {
            Json::Array(ref interfaces) => {
                interfaces.iter().map(|interface| {
                    self.add_interface(managed_ip.clone(), (*interface).clone())
                }).collect()
            }
            _ => {
                println!("Expected list for {} interfaces, found {:?}", managed_ip,
                         device["interfaces"]);
                Vec::new()
            }
        }
    }
}

trait ProcessDevice {
    type ResultType;
    fn process_device(&self, managed_ip: String, device: HashMap<String, Json>) -> Self::ResultType;
}

impl ProcessDevice for isize {
    type ResultType = Vec<(String, object)>;
    fn process_device(&self, managed_ip: String, device: HashMap<String, Json>) -> Self::ResultType {
        self.add_interfaces(managed_ip, device)
    }
}

pub fn main() {}