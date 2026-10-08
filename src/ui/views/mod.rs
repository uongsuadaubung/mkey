//! Views for MKey UI

pub mod control_panel;
pub mod tray_handler;

pub use control_panel::{
    ControlPanelControls, IDC_BTN_ADD_MACRO, IDC_BTN_CANCEL_MACRO, IDC_BTN_CLOSE, IDC_BTN_DEFAULTS, IDC_BTN_DEL_MACRO,
    IDC_BTN_EDIT_MACRO, IDC_BTN_EXIT, IDC_BTN_OPEN_LOG, IDC_EDIT_MACRO_KEY, IDC_EDIT_MACRO_VALUE,
    IDC_LIST_MACRO, IDC_TAB_MAIN,
};
pub use tray_handler::{
    TrayHandler, IDM_CONTROL_PANEL, IDM_EXIT,
    IDM_SIMPLE_TELEX, IDM_TELEX, IDM_TOGGLE_VIET, IDM_VNI, WM_TRAY_MESSAGE,
};

