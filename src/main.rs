use csv::{Reader, ReaderBuilder};
use serde::Deserialize;
use std::collections::hash_set::HashSet;
use std::env;
use std::error::Error;
use std::io;
use vcard::properties::*;
use vcard::values::email_value::EmailValue;
use vcard::values::text::Text;
use vcard::{Set, VCard};

// id;ipv4;name;last_name;birthday;room_number;email
#[derive(Debug, Deserialize)]
struct Record {
    id: String,
    ipv4: String,
    name: String,
    last_name: String,
    birthday: String,
    room_number: u32,
    email: String,
}

fn main() {
    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(env::args().nth(1).unwrap())
        .unwrap();
    reader
        .deserialize()
        .for_each(|result: Result<Record, _>| match result {
            Ok(record) => {
                let mut vcard = VCard::from_formatted_name(FormattedName::from_text(
                    Text::from_string(format!("{} {}", record.name, record.last_name)).unwrap(),
                ))
                .unwrap();
                let mut email = HashSet::new();
                email.insert(Email::from_email_value(
                    EmailValue::from_string(record.email).unwrap(),
                ));
                vcard.emails = Some(Set::from_hash_set(email).unwrap());

                let mut haus_category = HashSet::new();
                haus_category.insert(Text::from_str("Haus").unwrap());

                let mut categories = HashSet::new();
                categories.insert(Category::from_text_list(
                    Set::from_hash_set(haus_category).unwrap(),
                ));
                vcard.categories = Some(Set::from_hash_set(categories).unwrap());
                vcard.save_to_file(format!("{}.vcf", record.id)).unwrap();
            }
            Err(_) => (),
        });
}
