//! Test-only certificate-authority construction helpers.
//!
//! TODO Reviewed ??-??-2026.
//!
//! These helpers create test certificates and are not production CA tooling.
use std::str::FromStr;
use std::time::Duration;
use std::time::SystemTime;

use crypto_common::Generate;
use p384::ecdsa::VerifyingKey;
use p384::ecdsa::signature::Verifier;
use uuid::Uuid;
#[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
use wasm_bindgen_test::*;
use x509_cert::builder::Builder;
use x509_cert::builder::CertificateBuilder;
use x509_cert::builder::RequestBuilder;
use x509_cert::builder::profile::cabf::Root;
use x509_cert::builder::profile::cabf::tls::CertificateType;
use x509_cert::builder::profile::cabf::tls::Subordinate;
use x509_cert::builder::profile::cabf::tls::Subscriber;
use x509_cert::certificate::CertificateInner;
use x509_cert::der::Encode;
use x509_cert::der::asn1::Ia5String;
use x509_cert::der::flagset::FlagSet;
use x509_cert::ext::pkix::AuthorityKeyIdentifier;
use x509_cert::ext::pkix::ExtendedKeyUsage;
use x509_cert::ext::pkix::KeyUsage;
use x509_cert::ext::pkix::KeyUsages;
use x509_cert::ext::pkix::NameConstraints;
use x509_cert::ext::pkix::SubjectAltName;
use x509_cert::ext::pkix::SubjectKeyIdentifier;
use x509_cert::ext::pkix::constraints::BasicConstraints;
use x509_cert::ext::pkix::constraints::name::GeneralSubtree;
use x509_cert::ext::pkix::crl::CrlDistributionPoints;
use x509_cert::ext::pkix::crl::dp::DistributionPoint;
use x509_cert::ext::pkix::name::DistributionPointName;
use x509_cert::ext::pkix::name::GeneralName;
use x509_cert::name::Name;
use x509_cert::request::CertReq;
use x509_cert::serial_number::SerialNumber;
use x509_cert::spki::DecodePublicKey;
use x509_cert::spki::SubjectPublicKeyInfoOwned;
use x509_cert::time::Time;
use x509_cert::time::Validity;

use crate::ecdsa_p384::EcdsaP384DerSignature;
use crate::ecdsa_p384::EcdsaP384SigningKey;
use crate::x509::uuid_to_serial;
#[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
wasm_bindgen_test_configure!(run_in_browser);

/// [`SystemTime::now`][] equivalent that uses `Date.now()` on WASM.
#[cfg_attr(
    not(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none"))),
    inline
)]
pub fn now() -> SystemTime {
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    {
        use std::time::Duration;
        use std::time::UNIX_EPOCH;

        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern "C" {
            // NOTE: This signature works around https://bugzilla.mozilla.org/show_bug.cgi?id=1787770
            #[wasm_bindgen(js_namespace = Date, catch)]
            fn now() -> Result<f64, JsValue>;
        }

        let now = now().unwrap_throw() as u64;
        UNIX_EPOCH + Duration::from_millis(now)
    }

    #[cfg(not(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none"))))]
    SystemTime::now()
}

pub(crate) fn build_test_ca_root(
    not_before: Time,
    not_after: Time,
) -> (EcdsaP384SigningKey, CertificateInner) {
    let root_serial_uuid = Uuid::new_v4();
    let serial_number = uuid_to_serial(root_serial_uuid);

    let validity = Validity::new(not_before, not_after);

    let emits_ocsp_response = false;
    let root_subject = Name::from_str("CN=Oh no he is writing a CA,O=Pls Help,C=AU")
        .expect("static root CA subject name should be valid");
    let profile =
        Root::new(emits_ocsp_response, root_subject).expect("Unable to build root profile");

    let signing_key = EcdsaP384SigningKey::generate();
    let verifying_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`
    let pub_key = SubjectPublicKeyInfoOwned::from_key(&verifying_key).expect("get rsa pub key");

    let mut builder = CertificateBuilder::new(
        profile,
        serial_number,
        validity,
        // root_subject.clone(),
        pub_key.clone(),
        // &signing_key,
    )
    .expect("Create certificate");

    let dist_points = vec![DistributionPoint {
        distribution_point: Some(DistributionPointName::FullName(vec![
            GeneralName::UniformResourceIdentifier(
                "https://example.com/crl"
                    .to_string()
                    .try_into()
                    .expect("static root CRL URI should be valid"),
            ),
        ])),
        reasons: None,
        crl_issuer: None,
    }];

    let crl_extension = CrlDistributionPoints(dist_points);

    builder
        .add_extension(&crl_extension)
        .expect("Unable to add extension");

    let cert = builder
        .build::<EcdsaP384SigningKey, EcdsaP384DerSignature>(&signing_key)
        .expect("failed to build root CA certificate");

    // let cert_der = cert.to_der().expect("failed to encode root CA certificate as DER");
    println!("{cert:?}");

    let cert_bytes = cert
        .tbs_certificate()
        .to_der()
        .expect("failed to encode root CA TBS certificate as DER");

    let byte_sig: &[u8] = cert
        .signature()
        .as_bytes()
        .expect("root CA signature should be byte-aligned");
    let cert_sig =
        EcdsaP384DerSignature::try_from(byte_sig).expect("failed to parse root CA DER signature");
    assert!(verifying_key.verify(&cert_bytes, &cert_sig).is_ok());

    // For a root cert we must validate
    //
    //   Basic Constraints: critical
    //     CA: True
    let (critical, basic_constraints) = cert
        .tbs_certificate()
        .get_extension::<BasicConstraints>()
        .expect("failed to get extensions")
        .expect("basic constraints not present");

    assert!(critical);
    eprintln!("{basic_constraints:?}");

    assert!(basic_constraints.ca);
    assert!(basic_constraints.path_len_constraint.is_none());

    //   Key Usage: critical
    //     Certificate Sign
    //     CRL Sign

    let (critical, key_usage) = cert
        .tbs_certificate()
        .get_extension::<KeyUsage>()
        .expect("failed to get extensions")
        .expect("key usage not present");

    assert!(critical);

    eprintln!("{key_usage:?}");
    let expected_key_usages = KeyUsages::KeyCertSign | KeyUsages::CRLSign;
    assert_eq!(key_usage, expected_key_usages.into());

    //   Subject Key ID
    //     (Should be sha1 of the public key?)
    let (_, ca_subject_key_id) = cert
        .tbs_certificate()
        .get_extension::<SubjectKeyIdentifier>()
        .expect("failed to get extensions")
        .expect("key usage not present");

    eprintln!("{ca_subject_key_id:?}");

    //   Validity
    assert_eq!(
        cert.tbs_certificate()
            .validity()
            .not_before
            .to_unix_duration(),
        validity.not_before.to_unix_duration()
    );
    assert_eq!(
        cert.tbs_certificate()
            .validity()
            .not_after
            .to_unix_duration(),
        validity.not_after.to_unix_duration()
    );

    //   Issuer == Subject

    assert_eq!(
        cert.tbs_certificate().issuer(),
        cert.tbs_certificate().subject(),
    );

    //   Serial Number - We have to drop the first byte.
    println!(
        "{:?}",
        &cert.tbs_certificate().serial_number().as_bytes()[1..]
    );
    println!("{:?}", root_serial_uuid.as_bytes());
    let verify_serial = Uuid::from_slice(&cert.tbs_certificate().serial_number().as_bytes()[1..])
        .expect("failed to parse root CA certificate serial number as UUID");

    assert_eq!(root_serial_uuid, verify_serial);

    // CRL
    //   It's there. Trust me.

    (signing_key, cert)
}

pub(crate) fn build_test_ca_int(
    not_before: Time,
    not_after: Time,
    root_signing_key: &EcdsaP384SigningKey,
    root_ca_cert: &CertificateInner,
) -> (EcdsaP384SigningKey, CertificateInner) {
    let root_verifying_key = VerifyingKey::from(root_signing_key); // Serialize with `::to_encoded_point()`

    let (_, root_subject_key_id) = root_ca_cert
        .tbs_certificate()
        .get_extension::<SubjectKeyIdentifier>()
        .expect("failed to get extensions")
        .expect("key usage not present");

    let int_serial_uuid = Uuid::new_v4();

    let mut serial_bytes: [u8; 17] = [0; 17];
    serial_bytes[0] = 0x01;
    {
        let update_bytes = &mut serial_bytes[1..];
        update_bytes.copy_from_slice(int_serial_uuid.as_bytes());
    }

    println!("{serial_bytes:?}");
    let serial_number =
        SerialNumber::new(&serial_bytes).expect("intermediate serial number should be valid");

    let validity = Validity::new(not_before, not_after);

    let int_subject = Name::from_str("CN=Oh no its an intermediate,C=AU")
        .expect("static intermediate CA subject name should be valid");

    let profile = Subordinate {
        issuer: root_ca_cert.tbs_certificate().subject().clone(),
        subject: int_subject.clone(),
        path_len_constraint: Some(0),
        emits_ocsp_response: false,
        client_auth: false,
    };

    let int_signing_key = EcdsaP384SigningKey::generate();
    let int_verifying_key = VerifyingKey::from(&int_signing_key); // Serialize with `::to_encoded_point()`
    let int_pub_key =
        SubjectPublicKeyInfoOwned::from_key(&int_verifying_key).expect("get rsa pub key");

    let mut builder = CertificateBuilder::new(
        profile,
        serial_number,
        validity,
        int_pub_key.clone(),
        // root_signing_key,
    )
    .expect("Create certificate");

    let dist_points = vec![DistributionPoint {
        distribution_point: Some(DistributionPointName::FullName(vec![
            GeneralName::UniformResourceIdentifier(
                "https://example.com/int/crl"
                    .to_string()
                    .try_into()
                    .expect("static intermediate CRL URI should be valid"),
            ),
        ])),
        reasons: None,
        crl_issuer: None,
    }];

    let crl_extension = CrlDistributionPoints(dist_points);

    builder
        .add_extension(&crl_extension)
        .expect("Unable to add extension");

    let name_constraint_extension = NameConstraints {
        permitted_subtrees: Some(vec![GeneralSubtree {
            base: GeneralName::DnsName(
                "example.com"
                    .to_string()
                    .try_into()
                    .expect("static intermediate DNS name constraint should be valid"),
            ),
            minimum: 0,
            maximum: None,
        }]),
        excluded_subtrees: None,
    };

    builder
        .add_extension(&name_constraint_extension)
        .expect("Unable to add extension");

    let int_cert = builder
        .build::<EcdsaP384SigningKey, EcdsaP384DerSignature>(root_signing_key)
        .expect("failed to build intermediate CA certificate");

    // let cert_der = int_cert
    //     .to_der()
    //     .expect("failed to encode intermediate CA certificate as DER");
    println!("{int_cert:?}");

    let cert_bytes = int_cert
        .tbs_certificate()
        .to_der()
        .expect("failed to encode intermediate CA TBS certificate as DER");

    let byte_sig: &[u8] = int_cert
        .signature()
        .as_bytes()
        .expect("intermediate CA signature should be byte-aligned");
    let cert_sig = EcdsaP384DerSignature::try_from(byte_sig)
        .expect("failed to parse intermediate CA DER signature");
    assert!(root_verifying_key.verify(&cert_bytes, &cert_sig).is_ok());

    // Intermediate:
    //   Basic Constraints: critical
    //     CA:TRUE
    //     pathlen:0  // indicates no subordinate CA's

    let (critical, basic_constraints) = int_cert
        .tbs_certificate()
        .get_extension::<BasicConstraints>()
        .expect("failed to get extensions")
        .expect("basic constraints not present");

    assert!(critical);

    eprintln!("{basic_constraints:?}");

    assert!(basic_constraints.ca);
    assert_eq!(basic_constraints.path_len_constraint, Some(0));

    //   Key Usage: critical
    //     Digital Signature
    //     Certificate Sign
    //     CRL Sign

    let (critical, key_usage) = int_cert
        .tbs_certificate()
        .get_extension::<KeyUsage>()
        .expect("failed to get extensions")
        .expect("key usage not present");

    assert!(critical);

    eprintln!("{key_usage:?}");
    let expected_key_usages = KeyUsages::KeyCertSign | KeyUsages::CRLSign;
    assert_eq!(key_usage, expected_key_usages.into());

    //   "Name Constraints": https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.10
    //      Only needed for "servers".

    let (critical, _name_constraints) = int_cert
        .tbs_certificate()
        .get_extension::<NameConstraints>()
        .expect("failed to get extensions")
        .expect("key usage not present");

    assert!(critical);
    // Otherwise, it's there, trust me bro.

    //   Authority Key ID
    //     (Should be sha1 of the signer public key)
    let (_, authority_key_id) = int_cert
        .tbs_certificate()
        .get_extension::<AuthorityKeyIdentifier>()
        .expect("failed to get extensions")
        .expect("key usage not present");

    eprintln!("{authority_key_id:?}");

    assert_eq!(
        authority_key_id
            .key_identifier
            .as_ref()
            .expect("intermediate CA should include authority key identifier"),
        root_subject_key_id.as_ref()
    );

    //   Subject Key ID
    let (_, int_subject_key_id) = int_cert
        .tbs_certificate()
        .get_extension::<SubjectKeyIdentifier>()
        .expect("failed to get extensions")
        .expect("key usage not present");

    eprintln!("{int_subject_key_id:?}");

    //   Validity
    assert_eq!(
        int_cert
            .tbs_certificate()
            .validity()
            .not_before
            .to_unix_duration(),
        validity.not_before.to_unix_duration()
    );
    assert_eq!(
        int_cert
            .tbs_certificate()
            .validity()
            .not_after
            .to_unix_duration(),
        validity.not_after.to_unix_duration()
    );

    //   Issuer == Subject of Authority Key
    assert_eq!(
        int_cert.tbs_certificate().issuer(),
        root_ca_cert.tbs_certificate().subject(),
    );

    assert_eq!(int_cert.tbs_certificate().subject(), &int_subject);

    //   Serial Number
    println!(
        "{:?}",
        &int_cert.tbs_certificate().serial_number().as_bytes()[1..]
    );
    println!("{:?}", int_serial_uuid.as_bytes());
    let verify_serial =
        Uuid::from_slice(&int_cert.tbs_certificate().serial_number().as_bytes()[1..])
            .expect("failed to parse intermediate CA certificate serial number as UUID");

    assert_eq!(int_serial_uuid, verify_serial);

    //   CRL
    //   It's there. Trust me.

    (int_signing_key, int_cert)
}

pub(crate) fn build_test_csr(subject: &Name) -> (EcdsaP384SigningKey, CertReq) {
    let client_signing_key = EcdsaP384SigningKey::generate();
    let client_verifying_key = VerifyingKey::from(&client_signing_key);

    // Serialize with `::to_encoded_point()`
    // let int_pub_key =
    // SubjectPublicKeyInfoOwned::from_key(int_verifying_key).expect("get rsa pub key");

    let builder = RequestBuilder::new(subject.clone()).expect("Create certificate request");

    let client_cert_req = builder
        .build::<EcdsaP384SigningKey, EcdsaP384DerSignature>(&client_signing_key)
        .expect("failed to build client certificate request");

    let client_cert_req_der = client_cert_req
        .to_der()
        .expect("failed to encode client certificate request as DER");
    println!("{client_cert_req_der:?}");

    // First, extract the public key from the cert and use it to self-verify

    // Need to check the algorithm ID in future.
    let spki = &client_cert_req.info.public_key;

    println!("--> {spki:?}");

    let extracted_public_key = VerifyingKey::from_public_key_der(
        // spki.subject_public_key
        //     .as_bytes()
        //     .expect("client CSR subject public key should be byte-aligned")
        spki.to_der()
            .expect("failed to encode client CSR public key as DER")
            .as_slice(),
    )
    .expect("Unable to parse key bytes");
    assert_eq!(extracted_public_key, client_verifying_key);

    let req_bytes = client_cert_req
        .info
        .to_der()
        .expect("failed to encode client CSR info as DER");

    let byte_sig: &[u8] = client_cert_req
        .signature
        .as_bytes()
        .expect("client CSR signature should be byte-aligned");
    let client_cert_req_sig = EcdsaP384DerSignature::try_from(byte_sig)
        .expect("failed to parse client CSR DER signature");
    assert!(
        extracted_public_key
            .verify(&req_bytes, &client_cert_req_sig)
            .is_ok()
    );
    assert!(
        client_verifying_key
            .verify(&req_bytes, &client_cert_req_sig)
            .is_ok()
    );

    (client_signing_key, client_cert_req)
}

pub(crate) fn test_ca_sign_client_csr(
    not_before: Time,
    not_after: Time,
    cert_req: &CertReq,
    ca_signing_key: &EcdsaP384SigningKey,
    ca_cert: &CertificateInner,
) -> CertificateInner {
    // The process of issuance at this point really is up to "what do we want to copy from the
    // csr and what don't we?".

    // ------------------------

    let client_serial_uuid = Uuid::new_v4();

    let mut serial_bytes: [u8; 17] = [0; 17];
    serial_bytes[0] = 0x01;
    {
        let update_bytes = &mut serial_bytes[1..];
        update_bytes.copy_from_slice(client_serial_uuid.as_bytes());
    }

    println!("{serial_bytes:?}");
    let serial_number =
        SerialNumber::new(&serial_bytes).expect("client certificate serial number should be valid");

    let validity = Validity::new(not_before, not_after);

    let client_cert_subject = cert_req.info.subject.clone();

    let alt_name = Name::from_str("ENTRYUUID=cb98d3d3-efcc-4675-ad40-435f6280d41b")
        .expect("static client certificate directoryName SAN should be valid");

    let names = vec![GeneralName::DirectoryName(alt_name)];

    let certificate_type =
        CertificateType::domain_validated(client_cert_subject.clone(), names.clone())
            .expect("Unable to build certificate type");

    let profile = Subscriber {
        certificate_type,
        issuer: ca_cert.tbs_certificate().subject().clone(),
        client_auth: true,
        tls12_options: Default::default(),
        enable_data_encipherment: Default::default(),
    };

    let spki = &cert_req.info.public_key;

    println!("--> {spki:?}");

    let mut builder = CertificateBuilder::new(
        profile,
        serial_number,
        validity,
        // client_cert_subject.clone(),
        spki.clone(),
        // ca_signing_key,
    )
    .expect("Create certificate");

    // let eku_extension = ExtendedKeyUsage(vec![const_oid::db::rfc5280::ID_KP_CLIENT_AUTH]);
    //
    // builder
    // .add_extension(&eku_extension)
    // .expect("Unable to add extension");

    let san = SubjectAltName(names);

    builder
        .add_extension(&san)
        .expect("Unable to add extension");

    let client_cert = builder
        .build::<EcdsaP384SigningKey, EcdsaP384DerSignature>(ca_signing_key)
        .expect("failed to build client certificate");

    // let client_cert_der = client_cert
    //     .to_der()
    //     .expect("failed to encode client certificate as DER");
    println!("{client_cert:?}");

    // Client Leaf Cert
    //   Basic Constraints: critical
    //     CA:FALSE

    let (critical, basic_constraints) = client_cert
        .tbs_certificate()
        .get_extension::<BasicConstraints>()
        .expect("failed to get extensions")
        .expect("basic constraints not present");

    assert!(critical);
    eprintln!("{basic_constraints:?}");

    assert!(!basic_constraints.ca);

    //   Key Usage: critical
    //     Digital Signature
    //     Non Repudiation
    //     Key Encipherment

    let (critical, key_usage) = client_cert
        .tbs_certificate()
        .get_extension::<KeyUsage>()
        .expect("failed to get extensions")
        .expect("key usage not present");

    assert!(critical);

    eprintln!("{key_usage:?}");
    // These are no longer required
    // KeyUsages::NonRepudiation | KeyUsages::KeyEncipherment;
    let expected_key_usages: FlagSet<_> = KeyUsages::DigitalSignature.into();
    assert_eq!(key_usage, expected_key_usages.into());

    //   Extended Key Usage
    //     TLS Web Client Authentication

    let (_, key_usage) = client_cert
        .tbs_certificate()
        .get_extension::<ExtendedKeyUsage>()
        .expect("failed to get extensions")
        .expect("extended key usage not present");

    assert_eq!(
        key_usage.0,
        vec![
            const_oid::db::rfc5280::ID_KP_SERVER_AUTH,
            const_oid::db::rfc5280::ID_KP_CLIENT_AUTH
        ]
    );

    //   Authority Key ID
    //     (Should be sha256 of the signer public key)

    let (_, authority_key_id) = client_cert
        .tbs_certificate()
        .get_extension::<AuthorityKeyIdentifier>()
        .expect("failed to get extensions")
        .expect("key usage not present");

    eprintln!("{authority_key_id:?}");

    //   Subject Key ID
    let (_, int_subject_key_id) = ca_cert
        .tbs_certificate()
        .get_extension::<SubjectKeyIdentifier>()
        .expect("failed to get extensions")
        .expect("key usage not present");

    assert_eq!(
        authority_key_id
            .key_identifier
            .as_ref()
            .expect("client certificate should include authority key identifier"),
        int_subject_key_id.as_ref()
    );

    //   Subject Key ID - now ABSENT
    // let (_, client_subject_key_id) = client_cert
    // .tbs_certificate()
    // .get_extension::<SubjectKeyIdentifier>()
    // .expect("failed to get extensions")
    // .expect("key usage not present");
    //
    // eprintln!("{client_subject_key_id:?}");

    //   Validity
    assert_eq!(
        client_cert
            .tbs_certificate()
            .validity()
            .not_before
            .to_unix_duration(),
        validity.not_before.to_unix_duration()
    );
    assert_eq!(
        client_cert
            .tbs_certificate()
            .validity()
            .not_after
            .to_unix_duration(),
        validity.not_after.to_unix_duration()
    );

    //   Issuer == Subject of Authority Key
    assert_eq!(
        client_cert.tbs_certificate().issuer(),
        ca_cert.tbs_certificate().subject()
    );

    //  Must have
    //   Subject Alternative Name

    //   Serial Number
    println!(
        "{:?}",
        &client_cert.tbs_certificate().serial_number().as_bytes()[1..]
    );
    println!("{:?}", client_serial_uuid.as_bytes());
    let verify_serial =
        Uuid::from_slice(&client_cert.tbs_certificate().serial_number().as_bytes()[1..])
            .expect("failed to parse client certificate serial number as UUID");

    assert_eq!(client_serial_uuid, verify_serial);
    //   Subject
    assert_eq!(
        &client_cert_subject,
        client_cert.tbs_certificate().subject()
    );

    client_cert
}

pub(crate) fn test_ca_sign_server_csr(
    not_before: Time,
    not_after: Time,
    cert_req: &CertReq,
    ca_signing_key: &EcdsaP384SigningKey,
    ca_cert: &CertificateInner,
) -> CertificateInner {
    // The process of issuance at this point really is up to "what do we want to copy from the
    // csr and what don't we?".

    // ------------------------

    let server_serial_uuid = Uuid::new_v4();

    let mut serial_bytes: [u8; 17] = [0; 17];
    serial_bytes[0] = 0x01;
    {
        let update_bytes = &mut serial_bytes[1..];
        update_bytes.copy_from_slice(server_serial_uuid.as_bytes());
    }

    println!("{serial_bytes:?}");
    let serial_number =
        SerialNumber::new(&serial_bytes).expect("server certificate serial number should be valid");

    let validity = Validity::new(not_before, not_after);

    let server_cert_subject = cert_req.info.subject.clone();

    let alt_name = Ia5String::new("localhost").expect("static server DNS SAN should be valid");

    let names = vec![GeneralName::DnsName(alt_name)];

    let certificate_type =
        CertificateType::domain_validated(server_cert_subject.clone(), names.clone())
            .expect("Unable to build certificate type");

    let profile = Subscriber {
        certificate_type,
        issuer: ca_cert.tbs_certificate().subject().clone(),
        client_auth: false,
        tls12_options: Default::default(),
        enable_data_encipherment: Default::default(),
    };

    let spki = &cert_req.info.public_key;

    println!("--> {spki:?}");

    let mut builder = CertificateBuilder::new(
        profile,
        serial_number,
        validity,
        // server_cert_subject.clone(),
        spki.clone(),
        // ca_signing_key,
    )
    .expect("Create certificate");

    // let eku_extension = ExtendedKeyUsage(vec![const_oid::db::rfc5280::ID_KP_SERVER_AUTH]);
    //
    // builder
    // .add_extension(&eku_extension)
    // .expect("Unable to add extension");

    let san = SubjectAltName(names);

    builder
        .add_extension(&san)
        .expect("Unable to add extension");

    let server_cert = builder
        .build::<EcdsaP384SigningKey, EcdsaP384DerSignature>(ca_signing_key)
        .expect("Failed to build server certificate");

    // let server_cert_der = server_cert
    //     .to_der()
    //     .expect("failed to encode server certificate as DER");
    println!("{server_cert:?}");

    // VALIDATION NOW

    // Server Leaf Cert
    //   Basic Constraints: critical
    //     CA:FALSE

    let (critical, basic_constraints) = server_cert
        .tbs_certificate()
        .get_extension::<BasicConstraints>()
        .expect("failed to get extensions")
        .expect("basic constraints not present");

    assert!(critical);
    eprintln!("{basic_constraints:?}");

    assert!(!basic_constraints.ca);

    //   Key Usage: critical
    //     Digital Signature,
    //     Non Repudiation,
    //     Key Encipherment,
    //     Key Agreement

    let (critical, key_usage) = server_cert
        .tbs_certificate()
        .get_extension::<KeyUsage>()
        .expect("failed to get extensions")
        .expect("key usage not present");

    assert!(critical);

    eprintln!("{key_usage:?}");
    let expected_key_usages: FlagSet<_> = KeyUsages::DigitalSignature.into();
    // | KeyUsages::NonRepudiation
    // | KeyUsages::KeyAgreement
    // | KeyUsages::KeyEncipherment;
    assert_eq!(key_usage, expected_key_usages.into());

    //   Extended Key Usage
    //     TLS Web Server Authentication

    let (_, key_usage) = server_cert
        .tbs_certificate()
        .get_extension::<ExtendedKeyUsage>()
        .expect("failed to get extensions")
        .expect("extended key usage not present");

    assert_eq!(key_usage.0, vec![const_oid::db::rfc5280::ID_KP_SERVER_AUTH]);

    //   Authority Key ID
    //     (Should be sha256 of the signer public key)

    let (_, authority_key_id) = server_cert
        .tbs_certificate()
        .get_extension::<AuthorityKeyIdentifier>()
        .expect("failed to get extensions")
        .expect("key usage not present");

    eprintln!("{authority_key_id:?}");

    //   Subject Key ID
    let (_, int_subject_key_id) = ca_cert
        .tbs_certificate()
        .get_extension::<SubjectKeyIdentifier>()
        .expect("failed to get extensions")
        .expect("key usage not present");

    assert_eq!(
        authority_key_id
            .key_identifier
            .as_ref()
            .expect("Failed to get authority key identifier"),
        int_subject_key_id.as_ref()
    );

    //   Subject Key ID - now ABSENT
    // let (_, server_subject_key_id) = server_cert
    // .tbs_certificate()
    // .get_extension::<SubjectKeyIdentifier>()
    // .expect("failed to get extensions")
    // .expect("key usage not present");
    //
    // eprintln!("{server_subject_key_id:?}");

    //   Validity
    assert_eq!(
        server_cert
            .tbs_certificate()
            .validity()
            .not_before
            .to_unix_duration(),
        validity.not_before.to_unix_duration()
    );
    assert_eq!(
        server_cert
            .tbs_certificate()
            .validity()
            .not_after
            .to_unix_duration(),
        validity.not_after.to_unix_duration()
    );

    //   Issuer == Subject of Authority Key
    assert_eq!(
        server_cert.tbs_certificate().issuer(),
        ca_cert.tbs_certificate().subject()
    );

    //  Must have
    //   Subject Alternative Name

    //   Serial Number
    println!(
        "{:?}",
        &server_cert.tbs_certificate().serial_number().as_bytes()[1..]
    );
    println!("{:?}", server_serial_uuid.as_bytes());
    let verify_serial =
        Uuid::from_slice(&server_cert.tbs_certificate().serial_number().as_bytes()[1..])
            .expect("Failed to parse server certificate serial number");

    assert_eq!(server_serial_uuid, verify_serial);
    //   Subject
    assert_eq!(
        &server_cert_subject,
        server_cert.tbs_certificate().subject()
    );

    server_cert
}

#[test]
#[cfg_attr(
    all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
    wasm_bindgen_test
)]
fn test_ca_build_process() {
    let now = now();
    let not_before = Time::try_from(now).expect("Failed to convert SystemTime to Time");
    let not_after =
        Time::try_from(now + Duration::new(3600, 0)).expect("Failed to convert SystemTime to Time");

    let (root_signing_key, root_ca_cert) = build_test_ca_root(not_before, not_after);

    // =========================================================================================

    let (int_signing_key, int_ca_cert) =
        build_test_ca_int(not_before, not_after, &root_signing_key, &root_ca_cert);

    // =========================================================================================

    let subject = Name::from_str("CN=multi pass").expect("Failed to parse subject name");

    let (_client_key, client_csr) = build_test_csr(&subject);

    let _client_cert = test_ca_sign_client_csr(
        not_before,
        not_after,
        &client_csr,
        &int_signing_key,
        &int_ca_cert,
    );

    // =========================================================================================

    let subject = Name::from_str("CN=localhost").expect("Failed to parse subject name");

    let (_server_key, server_csr) = build_test_csr(&subject);

    let _server_cert = test_ca_sign_server_csr(
        not_before,
        not_after,
        &server_csr,
        &int_signing_key,
        &int_ca_cert,
    );

    // Server Leaf Cert
    //   Basic Constraints: critical
    //     CA:FALSE
    //   Key Usage: critical
    //     Digital Signature,
    //     Non Repudiation,
    //     Key Encipherment,
    //     Key Agreement
    //   Extended Key Usage
    //     TLS Web Server Authentication
    //   Authority Key ID
    //     (Should be sha256 of the signer public key)
    //   Subject Key ID
    //   Validity
    //   Issuer == Subject of Authority Key
    //
    //  Must have
    //   Subject Alternative Name
    //     DNS: <hostname>
    //   Serial Number
    //   Subject
}
