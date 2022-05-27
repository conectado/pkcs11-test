use cryptoki::session::SessionFlags;

fn main() {
    let context = cryptoki::context::Pkcs11::new("/usr/local/lib/softhsm/libsofthsm2.so").unwrap();
    context
        .initialize(cryptoki::context::CInitializeArgs::OsThreads)
        .unwrap();
    println!("{:?}", context.get_slots_with_token());
    println!(
        "{:?}",
        context
            .get_slots_with_initialized_token()
            .unwrap()
            .first()
            .unwrap()
    );
    let init_token = context
        .get_slots_with_initialized_token()
        .unwrap()
        .first()
        .unwrap()
        .clone();

    let mut flags = SessionFlags::new();
    flags.set_serial_session(true).set_rw_session(true);
    let session = context.open_session_no_callback(init_token, flags).unwrap();
    let mechanism = cryptoki::mechanism::Mechanism::Ecdsa;
    let public_key_template =
        cryptoki::types::session.generate_key_pair(&mechanism, pub_key_template, priv_key_template);
    println!("Hello, world!");
}
