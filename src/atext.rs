use std::io::Read;

use crate::apool::Apool;
use crate::changeset::Changeset;

struct AText<'a> {
    changeset_vec: Vec<Changeset<'a>>,
    apool: Box<dyn Apool>,
}

impl<'a> AText<'a> {
    fn new(apool: Box<dyn Apool>) -> Self {
        AText {
            changeset_vec: vec![],
            apool,
        }
    }
    fn new_changeset_from_reader(&'a self, reader: &mut dyn Read) -> anyhow::Result<Changeset<'a>> {
        Changeset::from_reader(&self.apool, reader)
    }
    fn push_changeset_from_reader(&'a mut self, reader: &mut dyn Read) -> anyhow::Result<()> {
        self.changeset_vec.push(Changeset::from_reader(&self.apool, reader)?);
        Ok(())
    }
}
