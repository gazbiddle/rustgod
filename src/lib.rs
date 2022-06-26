mod scry;       // Scry is a trait that can be implemented by randomness modules
mod oracle;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
