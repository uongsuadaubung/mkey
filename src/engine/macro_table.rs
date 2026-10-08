use std::collections::HashMap;

/// Storage and resolution of Vietnamese shorthand / macros
#[derive(Debug, Clone, Default)]
pub struct MacroTable {
    entries: HashMap<String, String>,
}

impl MacroTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or update a macro entry
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.entries.insert(key.into().to_lowercase(), value.into());
    }

    /// Remove a macro entry
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.entries.remove(&key.to_lowercase())
    }

    /// Check if a macro shortcut key exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.entries.contains_key(&key.to_lowercase())
    }

    /// Clear all macro entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Number of macro entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if macro table is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Access all entries
    pub fn entries(&self) -> &HashMap<String, String> {
        &self.entries
    }

    /// Get all macro entries sorted by shortcut key
    pub fn get_sorted_entries(&self) -> Vec<(String, String)> {
        let mut list: Vec<(String, String)> = self
            .entries
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        list.sort_by(|a, b| a.0.cmp(&b.0));
        list
    }

    /// Load macros from a multiline text configuration (e.g. "key:expansion" or "key\texpansion")
    pub fn load_from_str(&mut self, text: &str) {
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((k, v)) = line.split_once(':') {
                self.insert(k.trim(), v.trim());
            } else if let Some((k, v)) = line.split_once('\t') {
                self.insert(k.trim(), v.trim());
            }
        }
    }

    /// Serialize all macros to configuration format
    pub fn save_to_str(&self) -> String {
        let mut s =
            String::from("# Bảng gõ tắt MKey\n# Cú pháp: <từ viết tắt>:<cụm từ thay thế>\n");
        for (k, v) in self.get_sorted_entries() {
            s.push_str(&format!("{}:{}\n", k, v));
        }
        s
    }

    /// Load macros from a file
    pub fn load_from_file(&mut self, path: &str) -> std::io::Result<()> {
        if let Ok(content) = std::fs::read_to_string(path) {
            self.load_from_str(&content);
        }
        Ok(())
    }

    /// Save macros to a file
    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.save_to_str())
    }

    /// Lookup macro expansion, matching the case convention of the input word
    pub fn lookup(&self, word: &str) -> Option<String> {
        if word.is_empty() {
            return None;
        }

        let key = word.to_lowercase();
        let expansion = self.entries.get(&key)?;

        // Adjust case based on input word
        Some(apply_case_style(word, expansion))
    }
}

/// Applies the casing style of `source` onto `target`
/// - "ko" -> "không"
/// - "Ko" -> "Không"
/// - "KO" -> "KHÔNG"
pub fn apply_case_style(source: &str, target: &str) -> String {
    if source.is_empty() || target.is_empty() {
        return target.to_string();
    }

    let is_all_upper = source
        .chars()
        .all(|c| !c.is_alphabetic() || c.is_uppercase());
    if is_all_upper {
        return target.to_uppercase();
    }

    if let (Some(first_source), Some(first_target)) =
        (source.chars().next(), target.chars().next())
        && first_source.is_uppercase()
    {
        let mut res = String::with_capacity(target.len());
        for u in first_target.to_uppercase() {
            res.push(u);
        }
        res.push_str(&target[first_target.len_utf8()..]);
        return res;
    }

    target.to_string()
}
