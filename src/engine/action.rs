/// Action returned by the engine to tell the platform layer how to handle a keystroke.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineAction {
    /// Let the OS handle the keystroke normally without interference.
    Passthrough,

    /// Intercept the keystroke, send `backspaces` count to delete previous chars,
    /// then emit `output` string into the active input field.
    Replace { backspaces: usize, output: String },

    /// Consume (drop) the keystroke entirely. Do not let OS process it.
    Consume,
}
