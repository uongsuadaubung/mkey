use mkey::vietnamese::{
    charset::{BaseVowel, Diacritic, Tone},
    spelling::{
        is_valid_coda_pair, is_valid_coda_start, is_valid_onset, is_valid_onset_extension,
        is_valid_vietnamese_syllable,
    },
    syllable::Syllable,
    VowelLetter,
};

#[test]
fn test_onset_extensions() {
    assert!(is_valid_onset_extension(&[('c', false)], 'h'));
    assert!(is_valid_onset_extension(&[('t', false)], 'r'));
    assert!(is_valid_onset_extension(&[('n', false)], 'g'));
    assert!(is_valid_onset_extension(&[('n', false), ('g', false)], 'h'));
    assert!(!is_valid_onset_extension(&[('b', false)], 't'));
    assert!(!is_valid_onset_extension(&[('t', false), ('r', false)], 'h'));
}

#[test]
fn test_coda_checks() {
    assert!(is_valid_coda_start('n', false));
    assert!(is_valid_coda_start('c', false));
    assert!(!is_valid_coda_start('g', false));
    assert!(is_valid_coda_start('g', true)); // quick-end
    assert!(!is_valid_coda_start('x', true));

    assert!(is_valid_coda_pair('n', 'g'));
    assert!(is_valid_coda_pair('n', 'h'));
    assert!(is_valid_coda_pair('c', 'h'));
    assert!(!is_valid_coda_pair('n', 't'));
    assert!(!is_valid_coda_pair('p', 't'));
}

#[test]
fn test_syllable_with_glides() {
    // "giờ" -> onset: 'g', vowels: ['i', 'ơ']
    let gio = Syllable {
        onset: vec![('g', false)],
        d_stroke: false,
        vowels: vec![
            VowelLetter {
                base: BaseVowel::I,
                diacritic: Diacritic::None,
                is_upper: false,
            },
            VowelLetter {
                base: BaseVowel::O,
                diacritic: Diacritic::Horn,
                is_upper: false,
            },
        ],
        tone: Tone::Grave,
        coda: Vec::new(),
    };
    assert!(is_valid_vietnamese_syllable(&gio));

    // "bạc" with Grave tone is invalid (coda 'c' only accepts Acute or DotBelow)
    let bac_grave = Syllable {
        onset: vec![('b', false)],
        d_stroke: false,
        vowels: vec![VowelLetter {
            base: BaseVowel::A,
            diacritic: Diacritic::None,
            is_upper: false,
        }],
        tone: Tone::Grave,
        coda: vec![('c', false)],
    };
    assert!(!is_valid_vietnamese_syllable(&bac_grave));

    let bac_dot = Syllable {
        onset: vec![('b', false)],
        d_stroke: false,
        vowels: vec![VowelLetter {
            base: BaseVowel::A,
            diacritic: Diacritic::None,
            is_upper: false,
        }],
        tone: Tone::DotBelow,
        coda: vec![('c', false)],
    };
    assert!(is_valid_vietnamese_syllable(&bac_dot));
}

#[test]
fn test_onset_validity() {
    // Valid onsets
    assert!(is_valid_onset(&[], false)); // empty onset is valid
    assert!(is_valid_onset(&[('d', false)], true)); // 'đ'
    assert!(is_valid_onset(&[('b', false)], false));
    assert!(is_valid_onset(&[('t', false), ('r', false)], false));
    assert!(is_valid_onset(&[('n', false), ('g', false), ('h', false)], false));

    // Invalid non-native consonants in Vietnamese
    assert!(!is_valid_onset(&[('f', false)], false));
    assert!(!is_valid_onset(&[('j', false)], false));
    assert!(!is_valid_onset(&[('w', false)], false));
    assert!(!is_valid_onset(&[('z', false)], false));

    // Invalid onset clusters
    assert!(!is_valid_onset(&[('f', false), ('l', false)], false));
    assert!(!is_valid_onset(&[('s', false), ('t', false)], false));
}

#[test]
fn test_invalid_syllables() {
    // "fĩ" -> onset: 'f', vowel: 'i' with Tilde -> INVALID!
    let fi_tilde = Syllable {
        onset: vec![('f', false)],
        d_stroke: false,
        vowels: vec![VowelLetter {
            base: BaseVowel::I,
            diacritic: Diacritic::None,
            is_upper: false,
        }],
        tone: Tone::Tilde,
        coda: Vec::new(),
    };
    assert!(!is_valid_vietnamese_syllable(&fi_tilde));

    // "q" without "u" -> INVALID!
    let qa = Syllable {
        onset: vec![('q', false)],
        d_stroke: false,
        vowels: vec![VowelLetter {
            base: BaseVowel::A,
            diacritic: Diacritic::None,
            is_upper: false,
        }],
        tone: Tone::None,
        coda: Vec::new(),
    };
    assert!(!is_valid_vietnamese_syllable(&qa));
}


