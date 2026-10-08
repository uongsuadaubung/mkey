use mkey::{EngineAction, EngineConfig, VietnameseEngine};

fn simulate(engine: &mut VietnameseEngine, text: &str) -> String {
    let mut screen = String::new();
    for ch in text.chars() {
        let is_upper = ch.is_uppercase();
        match engine.on_key(ch, is_upper, false) {
            EngineAction::Passthrough => screen.push(ch),
            EngineAction::Replace { backspaces, output } => {
                for _ in 0..backspaces {
                    screen.pop();
                }
                screen.push_str(&output);
            }
            EngineAction::Consume => {}
        }
    }
    screen
}

#[test]
fn test_group_1_english_words_preservation() {
    let mut engine = VietnameseEngine::new(EngineConfig::default());

    // Các từ tiếng Anh phổ biến trong lập trình & đời sống không bị nuốt phụ âm hay biến đổi sai
    assert_eq!(simulate(&mut engine, "browser "), "browser ");
    assert_eq!(simulate(&mut engine, "project "), "project ");
    assert_eq!(simulate(&mut engine, "interface "), "interface ");
    assert_eq!(simulate(&mut engine, "speed "), "speed ");
    assert_eq!(simulate(&mut engine, "flow "), "flow ");
    assert_eq!(simulate(&mut engine, "wwindow "), "window "); // Gõ 'ww' ra 'w'
    assert_eq!(simulate(&mut engine, "file "), "file ");
    assert_eq!(simulate(&mut engine, "json "), "json ");
    assert_eq!(simulate(&mut engine, "log "), "log ");
    assert_eq!(simulate(&mut engine, "tag "), "tag ");
    assert_eq!(simulate(&mut engine, "bug "), "bug ");
    assert_eq!(simulate(&mut engine, "flag "), "flag ");
    assert_eq!(simulate(&mut engine, "dog "), "dog ");
    assert_eq!(simulate(&mut engine, "passs "), "pass "); // Gõ 'passs' (3 phím 's') để ra 'pass'
    assert_eq!(simulate(&mut engine, "pass "), "pas "); // Gõ 2 phím 's' hủy sắc trả lại 1 chữ 's' -> 'pas'
}

#[test]
fn test_group_2_undo_toggle_and_cancel_key() {
    let mut engine = VietnameseEngine::new(EngineConfig::default());

    // 1. Gõ lặp phím dấu 's' -> hoàn tác sắc
    assert_eq!(simulate(&mut engine, "toanss "), "toans ");

    // 2. Gõ lặp phím sừng 'w' -> hoàn tác ư và khôi phục ký tự 'w'
    assert_eq!(simulate(&mut engine, "chuww "), "chuw ");
    assert_eq!(simulate(&mut engine, "chuwz "), "chu "); // Gõ 'z' xóa dấu sừng -> 'chu'

    // 3. Gõ lặp phím mũ 'e' (3 lần 'e') -> hoàn tác ê thành 2 chữ 'ee'
    assert_eq!(simulate(&mut engine, "teest "), "tết ");
    assert_eq!(simulate(&mut engine, "teeest "), "teest ");

    // 4. Gõ lặp phím đ-stroke 'd' -> hoàn tác đ
    assert_eq!(simulate(&mut engine, "dd "), "đ ");
    assert_eq!(simulate(&mut engine, "ddd "), "dd ");

    // 5. Phím 'z' xóa dấu thanh hoặc xóa dấu phụ (móc, mũ, đ-stroke)
    assert_eq!(simulate(&mut engine, "toansz "), "toan ");
    assert_eq!(simulate(&mut engine, "hoafz "), "hoa ");
    assert_eq!(simulate(&mut engine, "cowz "), "co "); // Gõ 'cow' ra 'cơ', gõ 'z' xóa móc sừng thành 'co'
    assert_eq!(simulate(&mut engine, "tooz "), "to "); // Gõ 'too' ra 'tô', gõ 'z' xóa mũ thành 'to'
    assert_eq!(simulate(&mut engine, "ddaz "), "da "); // Gõ 'dda' ra 'đa', gõ 'z' xóa nét đ thành 'da'

    // 6. Gõ lặp phím 'w' khôi phục chữ 'w' (e.g. coww -> cow)
    assert_eq!(simulate(&mut engine, "coww "), "cow ");
}

#[test]
fn test_group_3_free_mark_and_tone_placement() {
    let mut engine = VietnameseEngine::new(EngineConfig::default());

    // Gõ dấu ngay sau nguyên âm vs cuối từ -> cùng một kết quả chuẩn xác
    assert_eq!(simulate(&mut engine, "tieesng "), "tiếng ");
    assert_eq!(simulate(&mut engine, "tieengs "), "tiếng ");

    assert_eq!(simulate(&mut engine, "vieejt "), "việt ");
    assert_eq!(simulate(&mut engine, "vieetj "), "việt ");

    assert_eq!(simulate(&mut engine, "thuyeesn "), "thuyến ");
    assert_eq!(simulate(&mut engine, "thuyeens "), "thuyến ");

    // Đặt dấu kiểu mới chuẩn hiện đại: dấu rơi vào nguyên âm chính (thuỷ, thuý, hòa, hoàn)
    assert_eq!(simulate(&mut engine, "thuyr "), "thu\u{1EF7} ");
    assert_eq!(simulate(&mut engine, "thuys "), "thu\u{00FD} ");
    assert_eq!(simulate(&mut engine, "hoaf "), "ho\u{00E0} ");
    assert_eq!(simulate(&mut engine, "hoafn "), "ho\u{00E0}n ");

    // Gõ tự do nét đ và sừng cuối từ: dudowjc -> được
    assert_eq!(simulate(&mut engine, "dudowjc "), "được ");
}

#[test]
fn test_group_4_casing_preservation() {
    let mut engine = VietnameseEngine::new(EngineConfig::default());

    // Title Case
    assert_eq!(simulate(&mut engine, "Tieengs Vieetj "), "Tiếng Việt ");
    assert_eq!(simulate(&mut engine, "Haf Nooji "), "Hà Nội ");
    assert_eq!(simulate(&mut engine, "Ddaf Nawngx "), "Đà Nẵng ");

    // ALL CAPS: Không bao giờ bị nhảy về chữ thường
    assert_eq!(
        simulate(&mut engine, "COONGJ HOAF XAX HOOIJ CHUR NGHIAX VIEETJ NAM "),
        "CỘNG HOÀ XÃ HỘI CHỦ NGHĨA VIỆT NAM "
    );
    // Gõ 'RUSST' (với 2 phím 'S') để giữ chữ tiếng Anh 'RUST' thay vì biến thành 'RÚT'
    assert_eq!(
        simulate(&mut engine, "TOOI GOX TIEESNG VIEETJ TREEN RUSST "),
        "TÔI GÕ TIẾNG VIỆT TRÊN RUST "
    );

    // CamelCase trong lập trình: Giữ nguyên từng case qua ranh giới từ
    assert_eq!(simulate(&mut engine, "handleClick "), "handleClick ");
    assert_eq!(simulate(&mut engine, "itemCount "), "itemCount ");
    assert_eq!(simulate(&mut engine, "getOption "), "getOption ");
    assert_eq!(simulate(&mut engine, "checkFlag "), "checkFlag ");
    // 'List' chứa phím 's' tạo thành từ tiếng Việt 'lít', dùng 'ss' để giữ 'List'
    assert_eq!(simulate(&mut engine, "dataLisst "), "dataList ");
}

#[test]
fn test_group_5_backspace_across_space() {
    let mut engine = VietnameseEngine::new(EngineConfig::default());
    let mut screen = String::new();

    // 1. Gõ "chao " (đã có dấu cách)
    for ch in "chao ".chars() {
        match engine.on_key(ch, false, false) {
            EngineAction::Passthrough => screen.push(ch),
            EngineAction::Replace { backspaces, output } => {
                for _ in 0..backspaces {
                    screen.pop();
                }
                screen.push_str(&output);
            }
            EngineAction::Consume => {}
        }
    }
    assert_eq!(screen, "chao ");

    // 2. Backspace một lần xóa dấu cách
    screen.pop();
    engine.on_backspace();

    // 3. Gõ tiếp 's' -> phục hồi từ và thêm dấu sắc -> "cháo"
    match engine.on_key('s', false, false) {
        EngineAction::Passthrough => screen.push('s'),
        EngineAction::Replace { backspaces, output } => {
            for _ in 0..backspaces {
                screen.pop();
            }
            screen.push_str(&output);
        }
        EngineAction::Consume => {}
    }
    assert_eq!(screen, "cháo");
}

#[test]
fn test_group_6_bracket_shortcuts() {
    let mut engine = VietnameseEngine::new(EngineConfig::default());

    assert_eq!(simulate(&mut engine, "[ "), "ư ");
    assert_eq!(simulate(&mut engine, "] "), "ơ ");
    assert_eq!(simulate(&mut engine, "m[ "), "mư ");
    assert_eq!(simulate(&mut engine, "t] "), "tơ ");
    assert_eq!(simulate(&mut engine, "[[ "), "[ ");
}

#[test]
fn test_group_7_macro_expansion_with_casing() {
    let mut engine = VietnameseEngine::new(EngineConfig::default());
    engine.config_mut().use_macro = true;
    engine.macro_table.insert("ko", "không");
    engine.macro_table.insert("dc", "được");
    engine.macro_table.insert("vn", "Việt Nam");

    // Thường
    assert_eq!(simulate(&mut engine, "ko "), "không ");
    assert_eq!(simulate(&mut engine, "dc "), "được ");
    assert_eq!(simulate(&mut engine, "vn "), "Việt Nam ");

    // Viết hoa chữ đầu (Title case)
    assert_eq!(simulate(&mut engine, "Ko "), "Không ");
    assert_eq!(simulate(&mut engine, "Dc "), "Được ");

    // Viết hoa toàn bộ (ALL CAPS)
    assert_eq!(simulate(&mut engine, "KO "), "KHÔNG ");
    assert_eq!(simulate(&mut engine, "DC "), "ĐƯỢC ");
}
