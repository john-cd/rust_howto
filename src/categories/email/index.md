# Email

Crates to help with sending, receiving, formatting, and parsing email.

{{#include index.incl.md}}

[![lettre][c-lettre-badge]][c-lettre] [![cat-email][cat-email-badge]][cat-email]{{hi:Email}}


[![lettre-crates.io][c-lettre-crates.io-badge]][c-lettre-crates.io]
[![lettre-github][c-lettre-github-badge]][c-lettre-github]
[![lettre-lib.rs][c-lettre-lib.rs-badge]][c-lettre-lib.rs]

```rust
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn main() {
    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to("Hei <hei@domain.tld>".parse().unwrap())
        .subject("Happy new year")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Be happy!"))
        .unwrap();

    let creds = Credentials::new("smtp_username".to_owned(), "smtp_password".to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: write
</div>
