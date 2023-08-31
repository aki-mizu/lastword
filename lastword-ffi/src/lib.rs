mod lastword;
uniffi::include_scaffolding!("lastword");

fn get_last_word(seed_phrase: Vec<String>) -> Vec<String> {
    return lastword::generate_last_word(seed_phrase)
}


