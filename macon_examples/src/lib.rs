#[derive(macon::Builder, Default)]
pub struct Person {
    pub given_name: String,
    pub family_name: String,
    pub address: Address,
}

#[derive(macon::Builder, Default)]
pub struct Address {
    pub lines: Vec<String>,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn person_default() {
        let person = Person::builder().build();
        assert_eq!("", person.given_name);
        assert_eq!("", person.family_name);
        assert_eq!(Vec::<String>::new(), person.address.lines);
    }
}
