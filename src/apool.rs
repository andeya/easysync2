use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone)]
pub struct AttribPair {
    pub attrib_num: u32,
    pub attrib_str: String,
}

pub trait Apool {
    fn set(&mut self, attrib_pair: AttribPair) -> Option<AttribPair>;
    fn get(&self, num: u32) -> Option<&AttribPair>;
    fn get_num(&mut self, attrib_str: &dyn ToString) -> Option<u32>;
}

pub(crate) struct Mem {
    doc_id: u32,
    num_to_attrib: BTreeMap<u32, AttribPair>,
    attrib_to_num: BTreeMap<String, u32>,
}

impl Mem {
    pub(crate) fn new(doc_id: u32) -> Box<dyn Apool> {
        Box::new(Mem { doc_id, num_to_attrib: Default::default(), attrib_to_num: Default::default() })
    }
}

impl Apool for Mem {
    fn set(&mut self, attrib_pair: AttribPair) -> Option<AttribPair> {
        let attrib_num = attrib_pair.attrib_num;
        let attrib_str = attrib_pair.attrib_str.clone();
        self.attrib_to_num.insert(attrib_str, attrib_num);
        return self.num_to_attrib.insert(attrib_num, attrib_pair);
    }
    fn get(&self, num: u32) -> Option<&AttribPair> {
        self.num_to_attrib.get(&num)
    }
    fn get_num(&mut self, attrib_str: &dyn ToString) -> Option<u32> {
        let attrib_str = attrib_str.to_string();
        self.attrib_to_num.get(&attrib_str).map(|num| num.clone())
    }
}

#[test]
fn test_apool() {
    let mut apool = Mem::new(1);
    let old = apool.set(AttribPair { attrib_num: 1, attrib_str: "color:red".to_string() });
    assert_eq!(old, None);
    assert_eq!(apool.get(1), Some(&AttribPair { attrib_num: 1, attrib_str: "color:red".to_string() }))
}
