// rustgod::oracle
//  - IronOracle: a container for a `fortune` style set of quote

extern crate rand;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;

use crate::scry::Scry;

//--------------------------------------------------------------------------------
pub struct IronOracle {
    pub prophecies: Vec<String>,
    pub fate: ThreadRng,
}

impl Scry for IronOracle {
    // scry() -> String
    //  return a string from the Scry struct
    fn scry(&mut self) -> String {
        let i: usize = self.fate.gen_range(0,self.prophecies.len());
        format!("{}",self.prophecies[i])
    }

    // push() - just because it's a good convention
    fn push(&mut self, p: String) {
        self.prophecies.push(p);
    }

}

impl IronOracle {
    // new()
    pub fn new() -> IronOracle{
        IronOracle{ prophecies: Vec::new(), fate: thread_rng(), }
    }

    // len() - just because it's a good convention
    pub fn len(&self) -> usize { self.prophecies.len() }

}

//--------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::oracle::IronOracle;
    use crate::scry::Scry;

    #[test]
    fn forge_oracle() {
        let mut o = IronOracle::new();
        o.push(String::from("Phrase 1"));
        o.push(String::from("Phrase 2"));

        assert_eq!(o.len(), 2);
    }

    #[test]
    fn scry_oracle() {
        let mut o = IronOracle::new();
        o.push(String::from("a"));
//        o.push(String::from("b"));
//        o.push(String::from("c"));
//        o.push(String::from("d"));
//        o.push(String::from("e"));


        assert_eq!(o.scry(),String::from("a"));
    }
}
