use log::{error, info};
use rustls_native_certs::RootStoreBuilder;
use std::io::BufRead;
use std::io::Error;
use webpki::TrustAnchor;
use x509_parser::prelude::*;

fn main() {
    env_logger::init();

    let mut loader = RootCertStoreLoader::default();
    rustls_native_certs::build_native_certs(&mut loader)
        .expect("This can't fail because the loader always returns OK");

    let mut store = rustls::RootCertStore::empty();
    let (success, failure) = store.add_parsable_certificates(&loader.ders);

    info!(
        "Loaded {} valid and skipped {} invalid certificates",
        success, failure
    );

    loader.subjects.sort_unstable();

    for subject in loader.subjects {
        info!("Loaded: {}", subject);
    }
}

struct RootCertStoreLoader {
    ders: Vec<Vec<u8>>,
    subjects: Vec<String>,
}

impl Default for RootCertStoreLoader {
    fn default() -> Self {
        Self {
            ders: Default::default(),
            subjects: Default::default(),
        }
    }
}

impl RootStoreBuilder for RootCertStoreLoader {
    fn load_der(&mut self, der: Vec<u8>) -> Result<(), Error> {
        let x509 = X509Certificate::from_der(&der).unwrap().1.tbs_certificate;

        let subject = x509.subject.to_string();
        let subject = subject.split(", ").last().unwrap().to_owned();

        let parsed = TrustAnchor::try_from_cert_der(&der);

        match parsed {
            Ok(_) => {
                info!("Successfully parsed cert: {}", subject);
            }
            Err(e) => {
                error!("Failed to parse cert");
                error!("    Subject: {}", subject);
                error!("    Error: {}", e);
            }
        }

        self.ders.push(der);
        self.subjects.push(subject);

        Ok(())
    }

    fn load_pem_file(&mut self, _rd: &mut dyn BufRead) -> Result<(), Error> {
        // skip
        Ok(())
    }
}
