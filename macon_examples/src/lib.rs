pub type Flag = Option<bool>;
pub type Opt1<E> = Option<E>;
pub type Opt2<E> = Option<E>;

#[derive(macon::Builder)]
#[derive(Default)]
pub struct Person {
    pub given_name: String,
    pub family_name: String,
    pub nick_name: Opt1<String>,
    pub address: Address,
    pub email: Opt2<String>,
    pub married: Flag,
}

#[derive(macon::Builder)]
#[derive(Default)]
pub struct Address {
    pub lines: Vec<String>,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn assert_person(
        given_name: &'static str,
        family_name: &'static str,
        nick_name: Option<&'static str>,
        email: Option<&'static str>,
        married: Option<bool>,
        person: Person,
    ) {
        assert_eq!(given_name, person.given_name, "given_name");
        assert_eq!(family_name, person.family_name, "family_name");
        assert_eq!(nick_name.map(|s| s.to_string()), person.nick_name, "nick_name");
        assert_eq!(Vec::<String>::new(), person.address.lines, "address.lines");
        assert_eq!(email.map(|s| s.to_string()), person.email, "email");
        assert_eq!(married, person.married, "married");
    }

    #[test]
    pub fn person_default() {
        let person = Person::builder().build();
        assert_person(
            "",
            "",
            None,
            None,
            None,
            person
        );
    }

    #[test]
    pub fn person_married_true() {
        let person = Person::builder()
            .married(true)
            .build();
        assert_person(
            "",
            "",
            None,
            None,
            Some(true),
            person,
        );
    }

    #[test]
    pub fn person_married_none() {
        let person = Person::builder()
            .married_none()
            .build();
        assert_person(
            "",
            "",
            None,
            None,
            None,
            person,
        );
    }

    #[test]
    pub fn person_nickname_some() {
        let person = Person::builder()
            .nick_name("noname")
            .build();
        assert_person(
            "",
            "",
            Some("noname"),
            None,
            None,
            person,
        );
    }

    #[test]
    pub fn person_nickname_none() {
        let person = Person::builder()
            .nick_name_none()
            .build();
        assert_person(
            "",
            "",
            None,
            None,
            None,
            person,
        );
    }

    #[test]
    pub fn person_email_some() {
        let person = Person::builder()
            .email("noname@nowhere.com")
            .build();
        assert_person(
            "",
            "",
            None,
            Some("noname@nowhere.com"),
            None,
            person,
        );
    }

    #[test]
    pub fn person_email_none() {
        let person = Person::builder()
            .email_none()
            .build();
        assert_person(
            "",
            "",
            None,
            None,
            None,
            person,
        );
    }
}
