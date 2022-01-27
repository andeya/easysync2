use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct AttribPair(String);

struct Apool {
    doc_id: u64,
    num_to_attrib: BTreeMap<u32, AttribPair>,
    attrib_to_num: BTreeMap<AttribPair, u32>,
    next_num: u32,
}

impl Apool {
    fn new(doc_id: u64) -> Apool {
        Apool { doc_id, num_to_attrib: Default::default(), attrib_to_num: Default::default(), next_num: 0 }
    }
    fn attrib_to_num(&mut self, attrib_pair: &AttribPair) -> u32 {
        self.attrib_to_num.get(attrib_pair).map(|num| num.clone()).unwrap_or_else(|| {
            let num = self.next_num;
            self.num_to_attrib.insert(num, attrib_pair.clone());
            self.next_num += 1;
            num
        })
    }
    fn num_to_attrib(&self, num: u32) -> Option<&AttribPair> {
        self.num_to_attrib.get(&num)
    }
}
