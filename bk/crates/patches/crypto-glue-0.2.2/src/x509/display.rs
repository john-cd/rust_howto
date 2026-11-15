//! Human-readable formatting for X.509 certificate structures.
//!
//! TODO Reviewed ??-??-2026.
//!
//! The output supports diagnostics and is not a stable serialization format.
use std::fmt;

use const_oid::AssociatedOid;
use x509_cert::der::Decode;
use x509_cert::der::referenced::OwnedToRef;
use x509_cert::ext::pkix::AuthorityKeyIdentifier;
use x509_cert::ext::pkix::BasicConstraints;
use x509_cert::ext::pkix::ExtendedKeyUsage;
use x509_cert::ext::pkix::KeyUsage;
use x509_cert::ext::pkix::SubjectAltName;
use x509_cert::ext::pkix::SubjectKeyIdentifier;
use x509_cert::ext::pkix::name::GeneralName;
use x509_cert::spki::SubjectPublicKeyInfoRef;

use crate::x509::Certificate;

const INDENT: usize = 2;
const INDENT_TWO: usize = INDENT * 2;
const INDENT_THREE: usize = INDENT * 3;
const INDENT_FOUR: usize = INDENT * 4;

pub struct X509Display<'a> {
    cert: &'a Certificate,
}

impl<'a> From<&'a Certificate> for X509Display<'a> {
    fn from(cert: &'a Certificate) -> Self {
        X509Display { cert }
    }
}

impl fmt::Display for X509Display<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "--")?;
        writeln!(f, "Certificate:")?;

        // cert.tbs_certificate()
        writeln!(f, "{:indent$}Data:", "", indent = INDENT)?;

        writeln!(
            f,
            "{:indent$}Version: {:?}",
            "",
            self.cert.tbs_certificate().version(),
            indent = INDENT_TWO,
        )?;

        writeln!(f, "{:indent$}Serial:", "", indent = INDENT_TWO)?;
        writeln!(
            f,
            "{}",
            BytesDisplay {
                bytes: self.cert.tbs_certificate().serial_number().as_bytes(),
                indent: INDENT_THREE
            }
        )?;

        writeln!(
            f,
            "{:indent$}Subject: {}",
            "",
            self.cert.tbs_certificate().subject(),
            indent = INDENT_TWO
        )?;

        writeln!(
            f,
            "{:indent$}Issuer: {}",
            "",
            self.cert.tbs_certificate().issuer(),
            indent = INDENT_TWO
        )?;

        if let Some(issuer_unique_id) = &self.cert.tbs_certificate().issuer_unique_id() {
            writeln!(f, "{:indent$}Issuer Unique ID:", "", indent = INDENT_TWO)?;
            if let Some(bytes) = issuer_unique_id.as_bytes() {
                writeln!(
                    f,
                    "{}",
                    BytesDisplay {
                        bytes,
                        indent: INDENT_THREE
                    }
                )?;
            } else {
                writeln!(f, "{:indent$}INVALID", "", indent = INDENT_THREE)?;
            }
        }

        if let Some(subject_unique_id) = &self.cert.tbs_certificate().subject_unique_id() {
            writeln!(f, "{:indent$}Subject Unique ID:", "", indent = INDENT_TWO)?;
            if let Some(bytes) = subject_unique_id.as_bytes() {
                writeln!(
                    f,
                    "{}",
                    BytesDisplay {
                        bytes,
                        indent: INDENT_THREE
                    }
                )?;
            } else {
                writeln!(f, "{:indent$}INVALID", "", indent = INDENT_THREE)?;
            }
        }

        writeln!(f, "{:indent$}Validity", "", indent = INDENT_TWO)?;
        writeln!(
            f,
            "{:indent$}Not Before: {}",
            "",
            self.cert.tbs_certificate().validity().not_before,
            indent = INDENT_THREE
        )?;
        writeln!(
            f,
            "{:indent$}Not After: {}",
            "",
            self.cert.tbs_certificate().validity().not_after,
            indent = INDENT_THREE
        )?;

        write!(
            f,
            "{:indent$}Signature Algorithm: ",
            "",
            indent = INDENT_TWO
        )?;
        match &self.cert.tbs_certificate().signature().oid {
            &const_oid::db::rfc5912::ECDSA_WITH_SHA_256 => writeln!(f, "ecdsa-with-sha256")?,
            &const_oid::db::rfc5912::ECDSA_WITH_SHA_384 => writeln!(f, "ecdsa-with-sha384")?,
            &const_oid::db::rfc5912::ECDSA_WITH_SHA_512 => writeln!(f, "ecdsa-with-sha512")?,
            &const_oid::db::rfc5912::SHA_256_WITH_RSA_ENCRYPTION => {
                writeln!(f, "sha256-with-rsa-encryption")?
            }
            oid => writeln!(f, "OID= {oid:?}")?,
        }

        // SUBJECT PUBLIC KEY
        writeln!(
            f,
            "{}",
            SubjectPublicKeyDisplay {
                spki: self
                    .cert
                    .tbs_certificate()
                    .subject_public_key_info()
                    .owned_to_ref(),
                indent: INDENT_TWO
            }
        )?;

        for extension in self
            .cert
            .tbs_certificate()
            .extensions()
            .iter()
            .flat_map(|exts| exts.iter())
        {
            match extension.extn_id {
                BasicConstraints::OID => {
                    write!(f, "{:indent$}Basic Constraints:", "", indent = INDENT_TWO)?;
                    if let Ok(bc) = BasicConstraints::from_der(extension.extn_value.as_bytes()) {
                        if extension.critical {
                            write!(f, " critical")?;
                        }
                        writeln!(f)?;

                        writeln!(f, "{:indent$}CA: {}", "", bc.ca, indent = INDENT_THREE)?;
                        if let Some(path_len_constraint) = bc.path_len_constraint {
                            writeln!(
                                f,
                                "{:indent$}Path Length: {}",
                                "",
                                path_len_constraint,
                                indent = INDENT_THREE
                            )?;
                        }
                    } else {
                        writeln!(f, "INVALID")?
                    }
                }
                SubjectKeyIdentifier::OID => {
                    write!(
                        f,
                        "{:indent$}Subject Key Identifier:",
                        "",
                        indent = INDENT_TWO
                    )?;
                    if let Ok(ski) = SubjectKeyIdentifier::from_der(extension.extn_value.as_bytes())
                    {
                        if extension.critical {
                            write!(f, " critical")?;
                        }
                        writeln!(f)?;
                        writeln!(
                            f,
                            "{}",
                            BytesDisplay {
                                bytes: ski.as_ref().as_bytes(),
                                indent: INDENT_THREE
                            }
                        )?;
                    } else {
                        writeln!(f, "INVALID")?
                    }
                }
                AuthorityKeyIdentifier::OID => {
                    write!(
                        f,
                        "{:indent$}Authority Key Identifier:",
                        "",
                        indent = INDENT_TWO
                    )?;
                    if let Ok(aki) =
                        AuthorityKeyIdentifier::from_der(extension.extn_value.as_bytes())
                    {
                        if extension.critical {
                            write!(f, " critical")?;
                        }
                        writeln!(f)?;

                        if let Some(bytes) = &aki.key_identifier {
                            writeln!(
                                f,
                                "{}",
                                BytesDisplay {
                                    bytes: bytes.as_bytes(),
                                    indent: INDENT_THREE
                                }
                            )?;
                        }

                        if let Some(aki_cert_issuer) = &aki.authority_cert_issuer {
                            for name in aki_cert_issuer.iter() {
                                writeln!(
                                    f,
                                    "{}",
                                    GeneralNameDisplay {
                                        name,
                                        indent: INDENT_THREE
                                    }
                                )?;
                            }
                        }

                        if let Some(bytes) = &aki.authority_cert_serial_number {
                            writeln!(
                                f,
                                "{}",
                                BytesDisplay {
                                    bytes: bytes.as_bytes(),
                                    indent: INDENT_THREE
                                }
                            )?;
                        }
                    } else {
                        writeln!(f, "INVALID")?
                    }
                }
                KeyUsage::OID => {
                    write!(f, "{:indent$}Key Usage:", "", indent = INDENT_TWO)?;
                    let ku = if let Ok(ku) = KeyUsage::from_der(extension.extn_value.as_bytes()) {
                        if extension.critical {
                            write!(f, " critical")?;
                        }
                        writeln!(f)?;
                        ku
                    } else {
                        writeln!(f, "INVALID")?;
                        continue;
                    };

                    if ku.digital_signature() {
                        writeln!(f, "{:indent$}Digital Signature", "", indent = INDENT_THREE)?;
                    }

                    if ku.non_repudiation() {
                        writeln!(f, "{:indent$}Non Repudiation", "", indent = INDENT_THREE)?;
                    }

                    if ku.key_encipherment() {
                        writeln!(f, "{:indent$}Key Encipherment", "", indent = INDENT_THREE)?;
                    }

                    if ku.data_encipherment() {
                        writeln!(f, "{:indent$}Data Encipherment", "", indent = INDENT_THREE)?;
                    }

                    if ku.key_agreement() {
                        writeln!(f, "{:indent$}Key Agreement", "", indent = INDENT_THREE)?;
                    }

                    if ku.key_cert_sign() {
                        writeln!(f, "{:indent$}Key Cert Sign", "", indent = INDENT_THREE)?;
                    }

                    if ku.crl_sign() {
                        writeln!(f, "{:indent$}CRL Sign", "", indent = INDENT_THREE)?;
                    }

                    if ku.encipher_only() {
                        writeln!(f, "{:indent$}Encipher Only", "", indent = INDENT_THREE)?;
                    }

                    if ku.decipher_only() {
                        writeln!(f, "{:indent$}Decipher Only", "", indent = INDENT_THREE)?;
                    }
                }

                ExtendedKeyUsage::OID => {
                    write!(f, "{:indent$}Extended Key Usage:", "", indent = INDENT_TWO)?;
                    let eku = if let Ok(eku) =
                        ExtendedKeyUsage::from_der(extension.extn_value.as_bytes())
                    {
                        if extension.critical {
                            write!(f, " critical")?;
                        }
                        writeln!(f)?;
                        eku
                    } else {
                        writeln!(f, "INVALID")?;
                        continue;
                    };

                    for oid in eku.as_ref().iter() {
                        write!(f, "{:indent$}", "", indent = INDENT_THREE)?;
                        match oid {
                            &const_oid::db::rfc5280::ID_KP_SERVER_AUTH => {
                                writeln!(f, "Server Authentication")?
                            }
                            &const_oid::db::rfc5280::ID_KP_CLIENT_AUTH => {
                                writeln!(f, "Client Authentication")?
                            }
                            &const_oid::db::rfc5280::ID_KP_CODE_SIGNING => {
                                writeln!(f, "Code Signing")?
                            }
                            oid => writeln!(f, "OID= {oid:?}")?,
                        }
                    }
                }
                SubjectAltName::OID => {
                    write!(f, "{:indent$}Subject Alt Name:", "", indent = INDENT_TWO)?;
                    let san = if let Ok(san) =
                        SubjectAltName::from_der(extension.extn_value.as_bytes())
                    {
                        if extension.critical {
                            write!(f, " critical")?;
                        }
                        writeln!(f)?;
                        san
                    } else {
                        writeln!(f, "INVALID")?;
                        continue;
                    };

                    for name in san.0.iter() {
                        writeln!(
                            f,
                            "{}",
                            GeneralNameDisplay {
                                name,
                                indent: INDENT_THREE
                            }
                        )?;
                    }
                }

                oid => {
                    // Probably in RFC5280
                    writeln!(
                        f,
                        "{:indent$}Unknown Extension: OID= {oid:?}",
                        "",
                        indent = INDENT_TWO
                    )?
                }
            }
        }

        write!(f, "{:indent$}Signature Algorithm: ", "", indent = INDENT)?;
        match &self.cert.signature_algorithm().oid {
            &const_oid::db::rfc5912::ECDSA_WITH_SHA_256 => writeln!(f, "ecdsa-with-sha256")?,
            &const_oid::db::rfc5912::ECDSA_WITH_SHA_384 => writeln!(f, "ecdsa-with-sha384")?,
            &const_oid::db::rfc5912::ECDSA_WITH_SHA_512 => writeln!(f, "ecdsa-with-sha512")?,
            &const_oid::db::rfc5912::SHA_256_WITH_RSA_ENCRYPTION => {
                writeln!(f, "sha256-with-rsa-encryption")?
            }
            oid => writeln!(f, "OID= {oid:?}")?,
        }

        writeln!(f, "{:indent$}Signature:", "", indent = INDENT)?;
        if let Some(bytes) = self.cert.signature().as_bytes() {
            writeln!(
                f,
                "{}",
                BytesDisplay {
                    bytes,
                    indent: INDENT_TWO
                }
            )?;
        } else {
            writeln!(f, "{:indent$}INVALID", "", indent = INDENT_TWO)?;
        }

        Ok(())
    }
}

struct BytesDisplay<'a> {
    bytes: &'a [u8],
    indent: usize,
}

impl fmt::Display for BytesDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:indent$}", "", indent = self.indent)?;

        let mut iter = self.bytes.iter().peekable();

        let mut count = 0;
        while let Some(byte) = iter.next() {
            write!(f, "{byte:02x}")?;

            count += 1;

            if count % 18 == 0 {
                write!(f, "\n{:indent$}", "", indent = self.indent)?;
            } else if iter.peek().is_some() {
                write!(f, ":")?;
            }
        }

        Ok(())
    }
}

struct GeneralNameDisplay<'a> {
    name: &'a GeneralName,
    indent: usize,
}

impl fmt::Display for GeneralNameDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:indent$}", "", indent = self.indent)?;
        match self.name {
            GeneralName::OtherName(_othername) => {
                write!(f, "Other: TODO")
            }
            GeneralName::Rfc822Name(ia5str) => {
                write!(f, "Email: {ia5str}")
            }
            GeneralName::DnsName(ia5str) => {
                write!(f, "DNS: {ia5str}")
            }
            GeneralName::DirectoryName(name) => {
                write!(f, "DN: {name}")
            }
            GeneralName::EdiPartyName(_edi_party_name) => {
                write!(f, "EdiParty: TODO")
            }
            GeneralName::UniformResourceIdentifier(ia5str) => {
                write!(f, "URI: {ia5str}")
            }
            GeneralName::IpAddress(_octet_str) => {
                write!(f, "IpAddress: TODO")
            }
            GeneralName::RegisteredId(oid) => {
                write!(f, "OID: {oid}")
            }
        }
    }
}

pub struct SubjectPublicKeyDisplay<'a> {
    spki: SubjectPublicKeyInfoRef<'a>,
    indent: usize,
}

impl<'a> From<SubjectPublicKeyInfoRef<'a>> for SubjectPublicKeyDisplay<'a> {
    fn from(spki: SubjectPublicKeyInfoRef<'a>) -> Self {
        Self { spki, indent: 0 }
    }
}

impl fmt::Display for SubjectPublicKeyDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{:indent$}Subject Public Key Info:",
            "",
            indent = self.indent
        )?;

        let indent = self.indent + INDENT;

        write!(f, "{:indent$}Algorithm: ", "", indent = indent)?;

        let alg_oid = &self.spki.algorithm.oid;

        match alg_oid {
            &const_oid::db::rfc5912::ID_EC_PUBLIC_KEY => writeln!(f, "id-ec-public-key")?,
            oid => writeln!(f, "OID= {oid:?}")?,
        }

        if let Ok(param_oid) = self.spki.algorithm.parameters_oid() {
            write!(f, "{:indent$}Parameters: ", "", indent = indent)?;
            match param_oid {
                const_oid::db::rfc5912::SECP_384_R_1 => writeln!(f, "SEC P384 R1")?,
                const_oid::db::rfc5912::SECP_256_R_1 => writeln!(f, "SEC P256 R1")?,
                oid => writeln!(f, "OID= {oid:?}")?,
            }
        }

        writeln!(f, "{:indent$}Public Key:", "", indent = indent)?;
        let indent = self.indent + INDENT_TWO;

        if let Some(sk_bytes) = self.spki.subject_public_key.as_bytes() {
            write!(
                f,
                "{}",
                BytesDisplay {
                    bytes: sk_bytes,
                    indent,
                }
            )?;
        } else {
            write!(f, "{:indent$}INVALID", "", indent = indent)?;
        };

        Ok(())
    }
}
