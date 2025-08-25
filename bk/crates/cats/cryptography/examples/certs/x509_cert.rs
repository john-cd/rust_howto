#![allow(dead_code)]
// // ANCHOR: example
// use std::fs::File;
// use std::io::BufReader;

// use chrono::Utc;
// use rustls::internal::pemfile::certs;
// use rustls::internal::pemfile::rsa_private_keys;
// use x509_cert::builder::CertificateBuilder;
// use x509_cert::builder::Profile;
// use x509_cert::name::Name;
// use x509_cert::serial_number::SerialNumber;
// use x509_cert::time::Duration;
// use x509_cert::time::Time;

// fn main() -> anyhow::Result<()> {
//     // Load existing certificate and private key (replace with your actual
//     // paths)
//     let cert_file = File::open("path/to/your/cert.pem")?;
//     let key_file = File::open("path/to/your/key.pem")?;
//     let cert_reader = BufReader::new(cert_file);
//     let key_reader = BufReader::new(key_file);

//     let certs = certs(&mut cert_reader).unwrap();
//     let keys = rsa_private_keys(&mut key_reader).unwrap();

//     // Create a new certificate builder
//     let now = Utc::now();
//     let not_before = Time::from(now);
//     // Valid for one year.
//     let not_after = Time::from(now + Duration::days(365));
//     let serial_number = SerialNumber::random();
//     let subject = Name::build_common_name("example.com").unwrap();
//     let issuer = subject.clone();

//     let mut builder = CertificateBuilder::new(
//         Profile::Server,
//         serial_number,
//         Validity {
//             not_before,
//             not_after,
//         },
//         issuer,
//         subject,
//     )?;

//     // Add extensions (optional).
//     // ...

//     // Sign the certificate:
//     let certificate = builder.sign(&keys[0])?;

//     // Print or save the certificate (DER encoding):
//     println!("Certificate: {:?}", certificate.to_der()?);

//     Ok(())
// }
// // ANCHOR_END: example

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/705)
