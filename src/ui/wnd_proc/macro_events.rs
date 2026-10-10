//! Macro events handler (Add, Edit, Delete, Cancel, ListView select)

use crate::engine::config_store;
use crate::platform::win32::ENGINE_INSTANCE;
use crate::ui::UiState;

pub fn handle_add_macro(ui: &UiState) {
    let mtype = match ui
        .controls
        .tab_macro
        .combo_macro_type
        .get_selected()
        .unwrap_or(0)
    {
        1 => crate::engine::macro_table::MacroType::StartConsonant,
        2 => crate::engine::macro_table::MacroType::EndConsonant,
        _ => crate::engine::macro_table::MacroType::Normal,
    };
    let key = ui
        .controls
        .tab_macro
        .edit_macro_key
        .get_text()
        .trim()
        .to_string();
    let val = ui
        .controls
        .tab_macro
        .edit_macro_value
        .get_text()
        .trim()
        .to_string();

    if key.is_empty() || val.is_empty() {
        return;
    }

    let macros = if let Ok(mut guard) = ENGINE_INSTANCE.lock() {
        if let Some(ref mut engine) = *guard {
            engine.macro_table.insert_typed(&key, &val, mtype);
            let _ = config_store::save_config_and_macros(engine.config(), &engine.macro_table);
            println!(
                "[MKey] Đã thêm gõ tắt: '{}' -> '{}' ({:?})",
                key, val, mtype
            );
            engine.macro_table.get_sorted_entries()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    reset_macro_inputs(ui, &macros);
}

pub fn handle_edit_macro(ui: &UiState) {
    let mtype = match ui
        .controls
        .tab_macro
        .combo_macro_type
        .get_selected()
        .unwrap_or(0)
    {
        1 => crate::engine::macro_table::MacroType::StartConsonant,
        2 => crate::engine::macro_table::MacroType::EndConsonant,
        _ => crate::engine::macro_table::MacroType::Normal,
    };
    let key = ui
        .controls
        .tab_macro
        .edit_macro_key
        .get_text()
        .trim()
        .to_string();
    let val = ui
        .controls
        .tab_macro
        .edit_macro_value
        .get_text()
        .trim()
        .to_string();

    if key.is_empty() || val.is_empty() {
        return;
    }

    let macros = if let Ok(mut guard) = ENGINE_INSTANCE.lock() {
        if let Some(ref mut engine) = *guard {
            engine.macro_table.insert_typed(&key, &val, mtype);
            let _ = config_store::save_config_and_macros(engine.config(), &engine.macro_table);
            println!(
                "[MKey] Đã cập nhật gõ tắt: '{}' -> '{}' ({:?})",
                key, val, mtype
            );
            engine.macro_table.get_sorted_entries()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    reset_macro_inputs(ui, &macros);
}

pub fn handle_del_macro(ui: &UiState) {
    let key_input = ui
        .controls
        .tab_macro
        .edit_macro_key
        .get_text()
        .trim()
        .to_string();
    let key_to_del = if !key_input.is_empty() {
        key_input
    } else if let Some(sel_idx) = ui.controls.tab_macro.list_macro.get_selected_index() {
        ui.controls.tab_macro.list_macro.get_item_text(sel_idx, 0)
    } else {
        String::new()
    };

    if key_to_del.is_empty() {
        return;
    }

    let macros = if let Ok(mut guard) = ENGINE_INSTANCE.lock() {
        if let Some(ref mut engine) = *guard {
            if engine.macro_table.remove(&key_to_del).is_some() {
                let _ = config_store::save_config_and_macros(engine.config(), &engine.macro_table);
                println!("[MKey] Đã xóa từ gõ tắt: '{}'", key_to_del);
            }
            engine.macro_table.get_sorted_entries()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    reset_macro_inputs(ui, &macros);
}

pub fn handle_cancel_macro(ui: &UiState) {
    ui.controls.tab_macro.edit_macro_key.clear();
    ui.controls.tab_macro.edit_macro_value.clear();
    ui.controls.tab_macro.combo_macro_type.set_selected(0);
    ui.controls.tab_macro.list_macro.clear_selection();
    ui.controls.set_macro_edit_mode(false);
}

pub fn handle_macro_list_notify(ui: &UiState) {
    if let Some(sel_idx) = ui.controls.tab_macro.list_macro.get_selected_index() {
        let k = ui.controls.tab_macro.list_macro.get_item_text(sel_idx, 0);
        let v = ui.controls.tab_macro.list_macro.get_item_text(sel_idx, 1);
        let t = ui.controls.tab_macro.list_macro.get_item_text(sel_idx, 2);
        if !k.is_empty() {
            ui.controls.tab_macro.edit_macro_key.set_text(&k);
            ui.controls.tab_macro.edit_macro_value.set_text(&v);
            let strings = crate::language::current();
            let type_idx = if t == strings.macro_type_start {
                1
            } else if t == strings.macro_type_end {
                2
            } else {
                0
            };
            ui.controls
                .tab_macro
                .combo_macro_type
                .set_selected(type_idx);
            ui.controls.set_macro_edit_mode(true);
        }
    }
}

fn reset_macro_inputs(ui: &UiState, macros: &[crate::engine::macro_table::MacroEntry]) {
    ui.controls.populate_macros(macros);
    ui.controls.tab_macro.edit_macro_key.clear();
    ui.controls.tab_macro.edit_macro_value.clear();
    ui.controls.tab_macro.combo_macro_type.set_selected(0);
    ui.controls.tab_macro.list_macro.clear_selection();
    ui.controls.set_macro_edit_mode(false);
}
