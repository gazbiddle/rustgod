// rustgod::scry
//  - Scry: a trait that can be implemented for different randomness modules


pub trait Scry {
    fn scry(&mut self) -> String;
    fn push(&mut self, s: String);
}
