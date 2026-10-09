/// Vietnamese Tones (Dấu thanh)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tone {
    #[default]
    None,
    Acute,     // Sắc  (á, é, ó...)
    Grave,     // Huyền (à, è, ò...)
    HookAbove, // Hỏi  (ả, ẻ, ỏ...)
    Tilde,     // Ngã  (ã, ẽ, õ...)
    DotBelow,  // Nặng (ạ, ẹ, ọ...)
}

/// Vietnamese Diacritics (Dấu phụ / mũ / móc)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Diacritic {
    #[default]
    None,
    Circumflex, // Mũ (â, ê, ô)
    Breve,      // Trăng (ă)
    Horn,       // Móc (ơ, ư)
    DStroke,    // Gạch ngang (đ)
}

/// Base vowel letter without diacritics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseVowel {
    A,
    E,
    I,
    O,
    U,
    Y,
}

impl TryFrom<char> for BaseVowel {
    type Error = ();

    #[inline]
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_lowercase() {
            'a' => Ok(Self::A),
            'e' => Ok(Self::E),
            'i' => Ok(Self::I),
            'o' => Ok(Self::O),
            'u' => Ok(Self::U),
            'y' => Ok(Self::Y),
            _ => Err(()),
        }
    }
}

impl BaseVowel {
    pub fn from_char(c: char) -> Option<Self> {
        Self::try_from(c).ok()
    }
}

/// Builds a precomposed UTF-8 Unicode character from base vowel, diacritic, tone, and uppercase flag.
pub fn compose_vowel(base: BaseVowel, diacritic: Diacritic, tone: Tone, uppercase: bool) -> char {
    let lower_char = match (base, diacritic, tone) {
        // --- A ---
        (BaseVowel::A, Diacritic::None, Tone::None) => 'a',
        (BaseVowel::A, Diacritic::None, Tone::Acute) => 'á',
        (BaseVowel::A, Diacritic::None, Tone::Grave) => 'à',
        (BaseVowel::A, Diacritic::None, Tone::HookAbove) => 'ả',
        (BaseVowel::A, Diacritic::None, Tone::Tilde) => 'ã',
        (BaseVowel::A, Diacritic::None, Tone::DotBelow) => 'ạ',

        (BaseVowel::A, Diacritic::Circumflex, Tone::None) => 'â',
        (BaseVowel::A, Diacritic::Circumflex, Tone::Acute) => 'ấ',
        (BaseVowel::A, Diacritic::Circumflex, Tone::Grave) => 'ầ',
        (BaseVowel::A, Diacritic::Circumflex, Tone::HookAbove) => 'ẩ',
        (BaseVowel::A, Diacritic::Circumflex, Tone::Tilde) => 'ẫ',
        (BaseVowel::A, Diacritic::Circumflex, Tone::DotBelow) => 'ậ',

        (BaseVowel::A, Diacritic::Breve, Tone::None) => 'ă',
        (BaseVowel::A, Diacritic::Breve, Tone::Acute) => 'ắ',
        (BaseVowel::A, Diacritic::Breve, Tone::Grave) => 'ằ',
        (BaseVowel::A, Diacritic::Breve, Tone::HookAbove) => 'ẳ',
        (BaseVowel::A, Diacritic::Breve, Tone::Tilde) => 'ẵ',
        (BaseVowel::A, Diacritic::Breve, Tone::DotBelow) => 'ặ',

        // --- E ---
        (BaseVowel::E, Diacritic::None, Tone::None) => 'e',
        (BaseVowel::E, Diacritic::None, Tone::Acute) => 'é',
        (BaseVowel::E, Diacritic::None, Tone::Grave) => 'è',
        (BaseVowel::E, Diacritic::None, Tone::HookAbove) => 'ẻ',
        (BaseVowel::E, Diacritic::None, Tone::Tilde) => 'ẽ',
        (BaseVowel::E, Diacritic::None, Tone::DotBelow) => 'ẹ',

        (BaseVowel::E, Diacritic::Circumflex, Tone::None) => 'ê',
        (BaseVowel::E, Diacritic::Circumflex, Tone::Acute) => 'ế',
        (BaseVowel::E, Diacritic::Circumflex, Tone::Grave) => 'ề',
        (BaseVowel::E, Diacritic::Circumflex, Tone::HookAbove) => 'ể',
        (BaseVowel::E, Diacritic::Circumflex, Tone::Tilde) => 'ễ',
        (BaseVowel::E, Diacritic::Circumflex, Tone::DotBelow) => 'ệ',

        // --- I ---
        (BaseVowel::I, _, Tone::None) => 'i',
        (BaseVowel::I, _, Tone::Acute) => 'í',
        (BaseVowel::I, _, Tone::Grave) => 'ì',
        (BaseVowel::I, _, Tone::HookAbove) => 'ỉ',
        (BaseVowel::I, _, Tone::Tilde) => 'ĩ',
        (BaseVowel::I, _, Tone::DotBelow) => 'ị',

        // --- O ---
        (BaseVowel::O, Diacritic::None, Tone::None) => 'o',
        (BaseVowel::O, Diacritic::None, Tone::Acute) => 'ó',
        (BaseVowel::O, Diacritic::None, Tone::Grave) => 'ò',
        (BaseVowel::O, Diacritic::None, Tone::HookAbove) => 'ỏ',
        (BaseVowel::O, Diacritic::None, Tone::Tilde) => 'õ',
        (BaseVowel::O, Diacritic::None, Tone::DotBelow) => 'ọ',

        (BaseVowel::O, Diacritic::Circumflex, Tone::None) => 'ô',
        (BaseVowel::O, Diacritic::Circumflex, Tone::Acute) => 'ố',
        (BaseVowel::O, Diacritic::Circumflex, Tone::Grave) => 'ồ',
        (BaseVowel::O, Diacritic::Circumflex, Tone::HookAbove) => 'ổ',
        (BaseVowel::O, Diacritic::Circumflex, Tone::Tilde) => 'ỗ',
        (BaseVowel::O, Diacritic::Circumflex, Tone::DotBelow) => 'ộ',

        (BaseVowel::O, Diacritic::Horn, Tone::None) => 'ơ',
        (BaseVowel::O, Diacritic::Horn, Tone::Acute) => 'ớ',
        (BaseVowel::O, Diacritic::Horn, Tone::Grave) => 'ờ',
        (BaseVowel::O, Diacritic::Horn, Tone::HookAbove) => 'ở',
        (BaseVowel::O, Diacritic::Horn, Tone::Tilde) => 'ỡ',
        (BaseVowel::O, Diacritic::Horn, Tone::DotBelow) => 'ợ',

        // --- U ---
        (BaseVowel::U, Diacritic::None, Tone::None) => 'u',
        (BaseVowel::U, Diacritic::None, Tone::Acute) => 'ú',
        (BaseVowel::U, Diacritic::None, Tone::Grave) => 'ù',
        (BaseVowel::U, Diacritic::None, Tone::HookAbove) => 'ủ',
        (BaseVowel::U, Diacritic::None, Tone::Tilde) => 'ũ',
        (BaseVowel::U, Diacritic::None, Tone::DotBelow) => 'ụ',

        (BaseVowel::U, Diacritic::Horn, Tone::None) => 'ư',
        (BaseVowel::U, Diacritic::Horn, Tone::Acute) => 'ứ',
        (BaseVowel::U, Diacritic::Horn, Tone::Grave) => 'ừ',
        (BaseVowel::U, Diacritic::Horn, Tone::HookAbove) => 'ử',
        (BaseVowel::U, Diacritic::Horn, Tone::Tilde) => 'ữ',
        (BaseVowel::U, Diacritic::Horn, Tone::DotBelow) => 'ự',

        // --- Y ---
        (BaseVowel::Y, _, Tone::None) => 'y',
        (BaseVowel::Y, _, Tone::Acute) => 'ý',
        (BaseVowel::Y, _, Tone::Grave) => 'ỳ',
        (BaseVowel::Y, _, Tone::HookAbove) => 'ỷ',
        (BaseVowel::Y, _, Tone::Tilde) => 'ỹ',
        (BaseVowel::Y, _, Tone::DotBelow) => 'ỵ',

        // Fallbacks for invalid combinations (e.g. horn on A)
        _ => match base {
            BaseVowel::A => 'a',
            BaseVowel::E => 'e',
            BaseVowel::I => 'i',
            BaseVowel::O => 'o',
            BaseVowel::U => 'u',
            BaseVowel::Y => 'y',
        },
    };

    if uppercase {
        lower_char.to_uppercase().next().unwrap_or(lower_char)
    } else {
        lower_char
    }
}

/// Compose the letter 'd' / 'đ'
pub fn compose_d(has_stroke: bool, uppercase: bool) -> char {
    match (has_stroke, uppercase) {
        (true, false) => 'đ',
        (true, true) => 'Đ',
        (false, false) => 'd',
        (false, true) => 'D',
    }
}

/// Decomposes a character into (BaseVowel, Diacritic, Tone) if it is a Vietnamese vowel
pub fn decompose_vowel(c: char) -> Option<(BaseVowel, Diacritic, Tone)> {
    let lower = c.to_lowercase().next().unwrap_or(c);
    match lower {
        'a' => Some((BaseVowel::A, Diacritic::None, Tone::None)),
        'á' => Some((BaseVowel::A, Diacritic::None, Tone::Acute)),
        'à' => Some((BaseVowel::A, Diacritic::None, Tone::Grave)),
        'ả' => Some((BaseVowel::A, Diacritic::None, Tone::HookAbove)),
        'ã' => Some((BaseVowel::A, Diacritic::None, Tone::Tilde)),
        'ạ' => Some((BaseVowel::A, Diacritic::None, Tone::DotBelow)),

        'â' => Some((BaseVowel::A, Diacritic::Circumflex, Tone::None)),
        'ấ' => Some((BaseVowel::A, Diacritic::Circumflex, Tone::Acute)),
        'ầ' => Some((BaseVowel::A, Diacritic::Circumflex, Tone::Grave)),
        'ẩ' => Some((BaseVowel::A, Diacritic::Circumflex, Tone::HookAbove)),
        'ẫ' => Some((BaseVowel::A, Diacritic::Circumflex, Tone::Tilde)),
        'ậ' => Some((BaseVowel::A, Diacritic::Circumflex, Tone::DotBelow)),

        'ă' => Some((BaseVowel::A, Diacritic::Breve, Tone::None)),
        'ắ' => Some((BaseVowel::A, Diacritic::Breve, Tone::Acute)),
        'ằ' => Some((BaseVowel::A, Diacritic::Breve, Tone::Grave)),
        'ẳ' => Some((BaseVowel::A, Diacritic::Breve, Tone::HookAbove)),
        'ẵ' => Some((BaseVowel::A, Diacritic::Breve, Tone::Tilde)),
        'ặ' => Some((BaseVowel::A, Diacritic::Breve, Tone::DotBelow)),

        'e' => Some((BaseVowel::E, Diacritic::None, Tone::None)),
        'é' => Some((BaseVowel::E, Diacritic::None, Tone::Acute)),
        'è' => Some((BaseVowel::E, Diacritic::None, Tone::Grave)),
        'ẻ' => Some((BaseVowel::E, Diacritic::None, Tone::HookAbove)),
        'ẽ' => Some((BaseVowel::E, Diacritic::None, Tone::Tilde)),
        'ẹ' => Some((BaseVowel::E, Diacritic::None, Tone::DotBelow)),

        'ê' => Some((BaseVowel::E, Diacritic::Circumflex, Tone::None)),
        'ế' => Some((BaseVowel::E, Diacritic::Circumflex, Tone::Acute)),
        'ề' => Some((BaseVowel::E, Diacritic::Circumflex, Tone::Grave)),
        'ể' => Some((BaseVowel::E, Diacritic::Circumflex, Tone::HookAbove)),
        'ễ' => Some((BaseVowel::E, Diacritic::Circumflex, Tone::Tilde)),
        'ệ' => Some((BaseVowel::E, Diacritic::Circumflex, Tone::DotBelow)),

        'i' => Some((BaseVowel::I, Diacritic::None, Tone::None)),
        'í' => Some((BaseVowel::I, Diacritic::None, Tone::Acute)),
        'ì' => Some((BaseVowel::I, Diacritic::None, Tone::Grave)),
        'ỉ' => Some((BaseVowel::I, Diacritic::None, Tone::HookAbove)),
        'ĩ' => Some((BaseVowel::I, Diacritic::None, Tone::Tilde)),
        'ị' => Some((BaseVowel::I, Diacritic::None, Tone::DotBelow)),

        'o' => Some((BaseVowel::O, Diacritic::None, Tone::None)),
        'ó' => Some((BaseVowel::O, Diacritic::None, Tone::Acute)),
        'ò' => Some((BaseVowel::O, Diacritic::None, Tone::Grave)),
        'ỏ' => Some((BaseVowel::O, Diacritic::None, Tone::HookAbove)),
        'õ' => Some((BaseVowel::O, Diacritic::None, Tone::Tilde)),
        'ọ' => Some((BaseVowel::O, Diacritic::None, Tone::DotBelow)),

        'ô' => Some((BaseVowel::O, Diacritic::Circumflex, Tone::None)),
        'ố' => Some((BaseVowel::O, Diacritic::Circumflex, Tone::Acute)),
        'ồ' => Some((BaseVowel::O, Diacritic::Circumflex, Tone::Grave)),
        'ổ' => Some((BaseVowel::O, Diacritic::Circumflex, Tone::HookAbove)),
        'ỗ' => Some((BaseVowel::O, Diacritic::Circumflex, Tone::Tilde)),
        'ộ' => Some((BaseVowel::O, Diacritic::Circumflex, Tone::DotBelow)),

        'ơ' => Some((BaseVowel::O, Diacritic::Horn, Tone::None)),
        'ớ' => Some((BaseVowel::O, Diacritic::Horn, Tone::Acute)),
        'ờ' => Some((BaseVowel::O, Diacritic::Horn, Tone::Grave)),
        'ở' => Some((BaseVowel::O, Diacritic::Horn, Tone::HookAbove)),
        'ỡ' => Some((BaseVowel::O, Diacritic::Horn, Tone::Tilde)),
        'ợ' => Some((BaseVowel::O, Diacritic::Horn, Tone::DotBelow)),

        'u' => Some((BaseVowel::U, Diacritic::None, Tone::None)),
        'ú' => Some((BaseVowel::U, Diacritic::None, Tone::Acute)),
        'ù' => Some((BaseVowel::U, Diacritic::None, Tone::Grave)),
        'ủ' => Some((BaseVowel::U, Diacritic::None, Tone::HookAbove)),
        'ũ' => Some((BaseVowel::U, Diacritic::None, Tone::Tilde)),
        'ụ' => Some((BaseVowel::U, Diacritic::None, Tone::DotBelow)),

        'ư' => Some((BaseVowel::U, Diacritic::Horn, Tone::None)),
        'ứ' => Some((BaseVowel::U, Diacritic::Horn, Tone::Acute)),
        'ừ' => Some((BaseVowel::U, Diacritic::Horn, Tone::Grave)),
        'ử' => Some((BaseVowel::U, Diacritic::Horn, Tone::HookAbove)),
        'ữ' => Some((BaseVowel::U, Diacritic::Horn, Tone::Tilde)),
        'ự' => Some((BaseVowel::U, Diacritic::Horn, Tone::DotBelow)),

        'y' => Some((BaseVowel::Y, Diacritic::None, Tone::None)),
        'ý' => Some((BaseVowel::Y, Diacritic::None, Tone::Acute)),
        'ỳ' => Some((BaseVowel::Y, Diacritic::None, Tone::Grave)),
        'ỷ' => Some((BaseVowel::Y, Diacritic::None, Tone::HookAbove)),
        'ỹ' => Some((BaseVowel::Y, Diacritic::None, Tone::Tilde)),
        'ỵ' => Some((BaseVowel::Y, Diacritic::None, Tone::DotBelow)),

        _ => None,
    }
}

/// Checks if character is 'đ' or 'Đ'
pub fn is_d_stroke(c: char) -> bool {
    matches!(c, 'đ' | 'Đ')
}
