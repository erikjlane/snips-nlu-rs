use std::io::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

use errors::*;
use serde_json;

pub trait Gazetteer {
    fn contains(&self, value: &str) -> bool;
}

pub struct HashSetGazetteer {
    values: HashSet<String>,
}

impl HashSetGazetteer {
    pub fn new(r: &mut Read) -> Result<HashSetGazetteer> {
        let vec: Vec<String> = serde_json::from_reader(r)?;
        Ok(HashSetGazetteer { values: HashSet::from_iter(vec) })
    }
}

impl Gazetteer for HashSetGazetteer {
    fn contains(&self, value: &str) -> bool {
        self.values.contains(value)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use serde_json;
    use super::HashSetGazetteer;

    #[test]
    fn gazetteer_work() {
        //let data = r#"["abc", "xyz"]"#;
        //let json: Vec<String> = serde_json::from_str(&mut data).unwrap();

        //let mut c = Cursor::new(json);

        //assert!(HashSetGazetteer::new(&mut c).is_ok())
    }
}
