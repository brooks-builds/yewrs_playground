use rand_os::rand_core::RngCore;

pub fn log_string(value: &str) {
    gloo::console::log!(value);
}

pub fn create_random_string() -> String {
    let mut random_numbers = [0u8; 43];
    let charset = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_~.";
    let charset_length = charset.len();

    rand_os::rand_core::OsRng.fill_bytes(&mut random_numbers);

    random_numbers
        .into_iter()
        .map(|random_number: u8| {
            let (_index, random_char) = charset
                .char_indices()
                .nth(random_number as usize % charset_length)
                .unwrap();
            random_char.to_string()
        })
        .collect::<Vec<String>>()
        .join("")
}
