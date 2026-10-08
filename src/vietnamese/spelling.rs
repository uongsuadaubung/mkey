use super::{
    VowelLetter,
    charset::{BaseVowel, Diacritic, Tone},
    syllable::Syllable,
};

/// Validates whether a sequence of vowels forms a legitimate Vietnamese vowel cluster
pub fn is_valid_vowel_combination(vowels: &[VowelLetter]) -> bool {
    match vowels.len() {
        0 | 1 => true,
        2 => {
            let v0 = (vowels[0].base, vowels[0].diacritic);
            let v1 = (vowels[1].base, vowels[1].diacritic);
            matches!(
                (v0, v1),
                // ai, ao, au, ay, ây, âu
                ((BaseVowel::A, Diacritic::None), (BaseVowel::I, Diacritic::None))
                | ((BaseVowel::A, Diacritic::None), (BaseVowel::O, Diacritic::None))
                | ((BaseVowel::A, Diacritic::None), (BaseVowel::U, Diacritic::None))
                | ((BaseVowel::A, Diacritic::None), (BaseVowel::Y, Diacritic::None))
                | ((BaseVowel::A, Diacritic::Circumflex), (BaseVowel::Y, Diacritic::None))
                | ((BaseVowel::A, Diacritic::Circumflex), (BaseVowel::U, Diacritic::None))
                // eo, êu
                | ((BaseVowel::E, Diacritic::None), (BaseVowel::O, Diacritic::None))
                | ((BaseVowel::E, Diacritic::Circumflex), (BaseVowel::U, Diacritic::None))
                // ia, iê, iu
                | ((BaseVowel::I, Diacritic::None), (BaseVowel::A, Diacritic::None))
                | ((BaseVowel::I, Diacritic::None), (BaseVowel::E, Diacritic::Circumflex))
                | ((BaseVowel::I, Diacritic::None), (BaseVowel::U, Diacritic::None))
                // oa, oă, oe, oi, ôi, ơi
                | ((BaseVowel::O, Diacritic::None), (BaseVowel::A, Diacritic::None))
                | ((BaseVowel::O, Diacritic::None), (BaseVowel::A, Diacritic::Breve))
                | ((BaseVowel::O, Diacritic::None), (BaseVowel::E, Diacritic::None))
                | ((BaseVowel::O, Diacritic::None), (BaseVowel::I, Diacritic::None))
                | ((BaseVowel::O, Diacritic::Circumflex), (BaseVowel::I, Diacritic::None))
                | ((BaseVowel::O, Diacritic::Horn), (BaseVowel::I, Diacritic::None))
                | ((BaseVowel::O, Diacritic::None), (BaseVowel::O, Diacritic::None)) // oo (xoong, boong)
                // ua, uâ, uô, uơ, uê, ui, uy
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::A, Diacritic::None))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::A, Diacritic::Circumflex))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::O, Diacritic::Circumflex))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::O, Diacritic::Horn))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::E, Diacritic::Circumflex))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::I, Diacritic::None))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::Y, Diacritic::None))
                // ưa, ươ, ưu, ưi
                | ((BaseVowel::U, Diacritic::Horn), (BaseVowel::A, Diacritic::None))
                | ((BaseVowel::U, Diacritic::Horn), (BaseVowel::O, Diacritic::Horn))
                | ((BaseVowel::U, Diacritic::Horn), (BaseVowel::U, Diacritic::None))
                | ((BaseVowel::U, Diacritic::Horn), (BaseVowel::I, Diacritic::None))
                // ya, yê
                | ((BaseVowel::Y, Diacritic::None), (BaseVowel::A, Diacritic::None))
                | ((BaseVowel::Y, Diacritic::None), (BaseVowel::E, Diacritic::Circumflex))
            )
        }
        3 => {
            let v0 = (vowels[0].base, vowels[0].diacritic);
            let v1 = (vowels[1].base, vowels[1].diacritic);
            let v2 = (vowels[2].base, vowels[2].diacritic);
            matches!(
                (v0, v1, v2),
                // iêu, yêu
                ((BaseVowel::I, Diacritic::None), (BaseVowel::E, Diacritic::Circumflex), (BaseVowel::U, Diacritic::None))
                | ((BaseVowel::Y, Diacritic::None), (BaseVowel::E, Diacritic::Circumflex), (BaseVowel::U, Diacritic::None))
                // oai, oay, oao, oeo
                | ((BaseVowel::O, Diacritic::None), (BaseVowel::A, Diacritic::None), (BaseVowel::I, Diacritic::None))
                | ((BaseVowel::O, Diacritic::None), (BaseVowel::A, Diacritic::None), (BaseVowel::Y, Diacritic::None))
                | ((BaseVowel::O, Diacritic::None), (BaseVowel::A, Diacritic::None), (BaseVowel::O, Diacritic::None))
                | ((BaseVowel::O, Diacritic::None), (BaseVowel::E, Diacritic::None), (BaseVowel::O, Diacritic::None))
                // uai, uay, uôi, uơi, ươi, ươu, uya, uyê, uyu
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::A, Diacritic::None), (BaseVowel::I, Diacritic::None))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::A, Diacritic::None), (BaseVowel::Y, Diacritic::None))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::O, Diacritic::Circumflex), (BaseVowel::I, Diacritic::None))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::O, Diacritic::Horn), (BaseVowel::I, Diacritic::None))
                | ((BaseVowel::U, Diacritic::Horn), (BaseVowel::O, Diacritic::Horn), (BaseVowel::I, Diacritic::None))
                | ((BaseVowel::U, Diacritic::Horn), (BaseVowel::O, Diacritic::Horn), (BaseVowel::U, Diacritic::None))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::Y, Diacritic::None), (BaseVowel::A, Diacritic::None))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::Y, Diacritic::None), (BaseVowel::E, Diacritic::Circumflex))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::Y, Diacritic::None), (BaseVowel::U, Diacritic::None))
            )
        }
        _ => false,
    }
}

/// Validates whether an onset consonant sequence is a legitimate Vietnamese onset.
pub fn is_valid_onset(onset: &[(char, bool)], is_d_stroke: bool) -> bool {
    if onset.is_empty() {
        return true;
    }
    if is_d_stroke {
        return onset.len() == 1 && onset[0].0.eq_ignore_ascii_case(&'d');
    }
    match onset.len() {
        1 => {
            let c = onset[0].0.to_ascii_lowercase();
            matches!(
                c,
                'b' | 'c'
                    | 'd'
                    | 'g'
                    | 'h'
                    | 'k'
                    | 'l'
                    | 'm'
                    | 'n'
                    | 'p'
                    | 'q'
                    | 'r'
                    | 's'
                    | 't'
                    | 'v'
                    | 'x'
            )
        }
        2 => {
            let c0 = onset[0].0.to_ascii_lowercase();
            let c1 = onset[1].0.to_ascii_lowercase();
            matches!(
                (c0, c1),
                ('c', 'h')
                    | ('g', 'h')
                    | ('g', 'i')
                    | ('k', 'h')
                    | ('n', 'h')
                    | ('n', 'g')
                    | ('p', 'h')
                    | ('q', 'u')
                    | ('t', 'h')
                    | ('t', 'r')
            )
        }
        3 => {
            let c0 = onset[0].0.to_ascii_lowercase();
            let c1 = onset[1].0.to_ascii_lowercase();
            let c2 = onset[2].0.to_ascii_lowercase();
            c0 == 'n' && c1 == 'g' && c2 == 'h'
        }
        _ => false,
    }
}

/// Validates whether a syllable satisfies Vietnamese spelling / phonotactic rules.
pub fn is_valid_vietnamese_syllable(syllable: &Syllable) -> bool {
    // 1. Validate onset
    if !is_valid_onset(&syllable.onset, syllable.d_stroke) {
        return false;
    }

    if syllable.vowels.is_empty() {
        return true;
    }

    // 2. Validate onset + vowel orthography
    if !syllable.onset.is_empty() && !syllable.d_stroke {
        let first_vowel_base = syllable.vowels[0].base;
        let onset_str: String = syllable
            .onset
            .iter()
            .map(|(c, _)| c.to_ascii_lowercase())
            .collect();

        match onset_str.as_str() {
            "q" => {
                if first_vowel_base != BaseVowel::U {
                    return false;
                }
            }
            "k" => {
                if !matches!(first_vowel_base, BaseVowel::I | BaseVowel::Y | BaseVowel::E) {
                    return false;
                }
            }
            "c" => {
                if matches!(first_vowel_base, BaseVowel::I | BaseVowel::Y | BaseVowel::E) {
                    return false;
                }
            }
            "gh" | "ngh" => {
                if !matches!(first_vowel_base, BaseVowel::I | BaseVowel::E) {
                    return false;
                }
            }
            "ng" => {
                if matches!(first_vowel_base, BaseVowel::I | BaseVowel::E) {
                    return false;
                }
            }
            "g"
                // 'g' + 'i' is the valid 'gi' glide (gió, giờ, giúp)
                // but 'g' + 'e' / 'ê' is invalid (must use 'gh')
                if first_vowel_base == BaseVowel::E => {
                    return false;
                }
            _ => {}
        }
    }

    // Special glides:
    // If onset is 'g' and first vowel is 'i' (e.g. "gió", "giờ", "giúp", "giường", "giếng"),
    // or onset is 'q' and first vowel is 'u' (e.g. "qua", "quê", "quốc", "quần"),
    // the 'i' or 'u' acts as the glide consonant extension, and the vowel cluster starts at index 1!
    let last_onset = syllable.onset.last().map(|(c, _)| c.to_ascii_lowercase());
    let vowels_to_check = if syllable.vowels.len() >= 2 {
        if (last_onset == Some('g') && syllable.vowels[0].base == BaseVowel::I)
            || (last_onset == Some('q') && syllable.vowels[0].base == BaseVowel::U)
        {
            &syllable.vowels[1..]
        } else {
            &syllable.vowels[..]
        }
    } else {
        &syllable.vowels[..]
    };

    // 3. Valid vowel cluster
    if !is_valid_vowel_combination(vowels_to_check) {
        return false;
    }

    // 4. Validate coda
    if !syllable.coda.is_empty() {
        if !can_vowels_accept_coda(vowels_to_check) {
            return false;
        }

        match syllable.coda.len() {
            1 => {
                let c = syllable.coda[0].0.to_ascii_lowercase();
                if !matches!(c, 'c' | 'm' | 'n' | 'p' | 't' | 'g' | 'h' | 'k') {
                    return false;
                }
            }
            2 => {
                if !is_valid_coda_pair(syllable.coda[0].0, syllable.coda[1].0) {
                    return false;
                }
            }
            _ => return false,
        }

        // Stop codas (c, ch, p, t) only accept Acute (Sắc) or DotBelow (Nặng).
        // Nasal coda 'nh' accepts all 6 tones.
        if is_stop_coda(&syllable.coda) {
            match syllable.tone {
                Tone::None | Tone::Acute | Tone::DotBelow => {}
                Tone::Grave | Tone::HookAbove | Tone::Tilde => return false,
            }
        }
    }

    true
}

/// Returns true if the coda is a stop coda ('c', 'ch', 'p', 't').
/// Stop codas in Vietnamese phonotactics only accept Acute (Sắc) or DotBelow (Nặng) tones,
/// and do not accept additional vowels when followed by tone.
#[inline]
pub fn is_stop_coda(coda: &[(char, bool)]) -> bool {
    match coda {
        [(c, _)] => matches!(c.to_ascii_lowercase(), 'c' | 'p' | 't'),
        [(c1, _), (c2, _)] => c1.eq_ignore_ascii_case(&'c') && c2.eq_ignore_ascii_case(&'h'),
        _ => false,
    }
}

/// Validates whether an onset consonant sequence can be extended with next_ch.
/// Vietnamese multi-letter onsets:
/// - 2 letters: ch, gh, gi, kh, nh, ng, ph, qu, th, tr
/// - 3 letters: ngh
pub fn is_valid_onset_extension(current_onset: &[(char, bool)], next_ch: char) -> bool {
    let next_lower = next_ch.to_ascii_lowercase();
    match current_onset.len() {
        1 => {
            let first = current_onset[0].0.to_ascii_lowercase();
            matches!(
                (first, next_lower),
                ('c', 'h')
                    | ('g', 'h')
                    | ('g', 'i')
                    | ('k', 'h')
                    | ('n', 'h')
                    | ('n', 'g')
                    | ('p', 'h')
                    | ('q', 'u')
                    | ('t', 'h')
                    | ('t', 'r')
            )
        }
        2 => {
            let first = current_onset[0].0.to_ascii_lowercase();
            let second = current_onset[1].0.to_ascii_lowercase();
            first == 'n' && second == 'g' && next_lower == 'h'
        }
        _ => false,
    }
}

/// Validates whether a consonant can legitimately begin a Vietnamese coda.
/// Standard coda start letters: c, m, n, p, t
/// Quick-end start letters (opt-in): g (->ng), h (->nh), k (->ch)
pub fn is_valid_coda_start(ch: char, allow_quick_end: bool) -> bool {
    let ch_lower = ch.to_ascii_lowercase();
    matches!(ch_lower, 'c' | 'm' | 'n' | 'p' | 't')
        || (allow_quick_end && matches!(ch_lower, 'g' | 'h' | 'k'))
}

/// Validates whether two consonants form a legitimate 2-letter Vietnamese coda.
/// In Vietnamese, only 'ng', 'nh', and 'ch' are valid 2-letter codas.
pub fn is_valid_coda_pair(first: char, second: char) -> bool {
    let c0 = first.to_ascii_lowercase();
    let c1 = second.to_ascii_lowercase();
    (c0 == 'n' && (c1 == 'g' || c1 == 'h')) || (c0 == 'c' && c1 == 'h')
}

/// Validates whether a nucleus vowel combination can legally accept a coda consonant in Vietnamese.
/// In Vietnamese phonotactics:
/// - Single vowels can accept a coda: an, em, in, on, un, etc.
/// - Centering diphthongs can accept a coda: iê/yê, uô, ươ, uâ, oă, oa, oe, uê, uy.
/// - Off-glide diphthongs CANNOT accept a coda: ai, oi, ôi, ơi, ui, ưi, ay, ây, ao, eo, au, âu, iu, ưu, ia, ya, ua, ưa.
/// - Triphthongs ending in semivowels CANNOT accept a coda: iêu, yêu, oai, oay, oao, oeo, uai, uay, uôi, uơi, ươi, ươu, uya, uyu.
/// - Only triphthong 'uyê' can accept a coda: uyên, uyết.
pub fn can_vowels_accept_coda(vowels: &[VowelLetter]) -> bool {
    match vowels.len() {
        0 => false,
        1 => true,
        2 => {
            let v0 = (vowels[0].base, vowels[0].diacritic);
            let v1 = (vowels[1].base, vowels[1].diacritic);
            matches!(
                (v0, v1),
                // iê, yê, and intermediate typing bases (ie, ye for liene -> liên, yens -> yến)
                ((BaseVowel::I, Diacritic::None), (BaseVowel::E, Diacritic::Circumflex))
                | ((BaseVowel::I, Diacritic::None), (BaseVowel::E, Diacritic::None))
                | ((BaseVowel::Y, Diacritic::None), (BaseVowel::E, Diacritic::Circumflex))
                | ((BaseVowel::Y, Diacritic::None), (BaseVowel::E, Diacritic::None))
                // uô, ươ, uo (uo is the typing base for uô/ươ when free_mark is used)
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::O, Diacritic::Circumflex))
                | ((BaseVowel::U, Diacritic::Horn), (BaseVowel::O, Diacritic::Horn))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::O, Diacritic::None))
                // uâ, ua (ua is the typing base for uâ in xuana -> xuân)
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::A, Diacritic::Circumflex))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::A, Diacritic::None))
                // oă
                | ((BaseVowel::O, Diacritic::None), (BaseVowel::A, Diacritic::Breve))
                // oa, oe, uê, uy, and ue (typing base for quene -> quên)
                | ((BaseVowel::O, Diacritic::None), (BaseVowel::A, Diacritic::None))
                | ((BaseVowel::O, Diacritic::None), (BaseVowel::E, Diacritic::None))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::E, Diacritic::Circumflex))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::E, Diacritic::None))
                | ((BaseVowel::U, Diacritic::None), (BaseVowel::Y, Diacritic::None))
                | ((BaseVowel::O, Diacritic::None), (BaseVowel::O, Diacritic::None)) // oo (xoong, boong)
            )
        }
        3 => {
            let v0 = (vowels[0].base, vowels[0].diacritic);
            let v1 = (vowels[1].base, vowels[1].diacritic);
            let v2 = (vowels[2].base, vowels[2].diacritic);
            matches!(
                (v0, v1, v2),
                // uyê (uyên, uyết) and intermediate typing base uye (uyene -> uyên, chuyens -> chuyến)
                (
                    (BaseVowel::U, Diacritic::None),
                    (BaseVowel::Y, Diacritic::None),
                    (BaseVowel::E, Diacritic::Circumflex)
                ) | (
                    (BaseVowel::U, Diacritic::None),
                    (BaseVowel::Y, Diacritic::None),
                    (BaseVowel::E, Diacritic::None)
                )
            )
        }
        _ => false,
    }
}
