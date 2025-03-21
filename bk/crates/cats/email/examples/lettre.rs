// ANCHOR: example
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;

fn main() {
    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .reply_to("Someone <someone@domain.tld>".parse().unwrap())
        .to("Buddy <buddy@domain.tld>".parse().unwrap())
        .subject("Subject here")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Body text here"))
        .unwrap();

    let creds = Credentials::new(
        "smtp_username".to_owned(),
        "smtp_password".to_owned(),
    );

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
// ANCHOR_END: example
// Requires valid SMTP credentials to run
// TODO WIP review NOW
