use random_word::Lang;

pub fn random_words_seq(num: u32, sep: &str) -> String {
    (0..num)
        .map(|_| random_word::gen(Lang::Fr))
        .collect::<Vec<&str>>()
        .join(sep)
}
