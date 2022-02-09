use std::io::Read;

use crate::apool::Apool;
use crate::changeset::Changeset;

pub struct AText<'a> {
    doc_id: u64,
    changeset_vec: Vec<Changeset<'a>>,
    apool: Box<dyn Apool>,
}

impl<'a> AText<'a> {
    pub fn new(doc_id: u64, new_apool: fn(doc_id: u64) -> Box<dyn Apool>) -> Self {
        AText {
            doc_id,
            changeset_vec: vec![],
            apool: new_apool(doc_id),
        }
    }
    pub fn doc_id(&self) -> u64 {
        self.doc_id
    }
    pub fn new_changeset_from_reader(&'a self, reader: &mut dyn Read) -> anyhow::Result<Changeset<'a>> {
        Changeset::from_reader(&self.apool, reader)
    }
    pub fn push_changeset_from_reader(&'a mut self, reader: &mut dyn Read) -> anyhow::Result<()> {
        self.changeset_vec.push(Changeset::from_reader(&self.apool, reader)?);
        Ok(())
    }
    pub fn into_snapshot(mut self) -> anyhow::Result<AText<'a>> {
        let res = self.changeset_vec.iter_mut().reduce(|res, item| {
            res.compose(item);
            res
        });
        if let Some(c) = res {
            self.changeset_vec = vec![c.clone()];
        }
        Ok(self)
    }
}
