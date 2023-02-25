pub fn gen(value: &str) -> String {
    let hash = md5::compute(value);
    let val = base_62::encode(&hash.0);
    let short_val = val[0..6].to_string();

    return short_val;
}
