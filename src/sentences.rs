use rand::Rng;

static SENTENCES: &[&str] = &[
    "Don't work too hard. The sun will expand and engulf this CPU anyway.",
    "Everything you do today will eventually be overwritten.",
    "Nothing matters. Build good software anyway.",
    "That is all.",
    "The loop continues.",
    "The universe has not noticed.",
    "Try not to take it too seriously.",
];

pub fn welcome_message() -> &'static str {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..SENTENCES.len());
    SENTENCES[idx]
}
