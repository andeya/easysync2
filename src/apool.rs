use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub struct AttribPair {
    pub id: u32,
    pub kv: KeyValue,
}

impl AttribPair {
    pub fn new(id: u32, key: &dyn ToString, value: &dyn ToString) -> AttribPair {
        AttribPair { id, kv: KeyValue { key: key.to_string(), value: value.to_string() } }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl Display for KeyValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.key, self.value)
    }
}

pub trait Apool {
    fn set(&mut self, attrib_pair: AttribPair) -> Option<AttribPair>;
    fn get_attrib(&self, id: u32) -> Option<&AttribPair>;
    fn get_id(&mut self, attrib_kv: &dyn ToString) -> Option<u32>;
}

pub(crate) struct Mem {
    id_to_attrib: BTreeMap<u32, AttribPair>,
    attrib_to_id: BTreeMap<String, u32>,
}

impl Mem {
    pub(crate) fn new(_doc_id: u32) -> Box<dyn Apool> {
        Box::new(Mem { id_to_attrib: Default::default(), attrib_to_id: Default::default() })
    }
}

impl Apool for Mem {
    fn set(&mut self, attrib_pair: AttribPair) -> Option<AttribPair> {
        self.attrib_to_id.insert(attrib_pair.kv.to_string(), attrib_pair.id);
        return self.id_to_attrib.insert(attrib_pair.id, attrib_pair);
    }
    fn get_attrib(&self, id: u32) -> Option<&AttribPair> {
        self.id_to_attrib.get(&id)
    }
    fn get_id(&mut self, attrib_kv: &dyn ToString) -> Option<u32> {
        self.attrib_to_id.get(&attrib_kv.to_string()).map(|id| id.clone())
    }
}

#[test]
fn test_apool() {
    let mut apool = Mem::new(1);
    let attrib_pair = AttribPair::new(1, "color", "red");
    let old = apool.set(attrib_pair.clone());
    assert_eq!(old, None);
    assert_eq!(apool.get_attrib(1), Some(&attrib_pair))
}
