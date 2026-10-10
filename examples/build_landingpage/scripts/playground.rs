//! Preset selection logic and interactive live input simulator

pub fn render() -> &'static str {
    r####"  // ==========================================================================
  // Preset Selection Logic
  // ==========================================================================
  function selectPreset(wordKey) {
    const data = PRESETS[wordKey];
    if (!data) return;

    playSwitchSound();

    legacyDisplay.textContent = data.legacy;
    legacyCaption.innerHTML = data.legacyCaption;

    mkeyDisplay.innerHTML = data.mkey + '<span class="cursor"></span>';
    mkeyCaption.innerHTML = data.mkeyCaption;

    presetButtons.forEach(btn => {
      btn.classList.toggle('active', btn.dataset.word === wordKey);
    });

    liveInput.value = '';
  }

  presetButtons.forEach(btn => {
    btn.addEventListener('click', () => {
      selectPreset(btn.dataset.word);
    });
  });

  // ==========================================================================
  // Interactive Live Input Simulator
  // ==========================================================================
  liveInput.addEventListener('keydown', (e) => {
    playSwitchSound(e.key === ' ');
  });

  liveInput.addEventListener('input', (e) => {
    const val = liveInput.value;
    if (!val) {
      selectPreset('error');
      return;
    }

    // Deselect preset buttons
    presetButtons.forEach(btn => btn.classList.remove('active'));

    // Check if input contains repeated consonants or known English words
    const lower = val.toLowerCase();
    let legacySimulation = val;

    // Simulate legacy IME vs MKey behavior
    let mkeySimulation = val;
    let mkeyExplain = `Bảo toàn từ bạn gõ: <strong>${escapeHtml(val)}</strong>`;

    // Smart Macro Simulation: ko -> không, Ko -> Không, KO -> KHÔNG
    if (val === 'ko') {
      mkeySimulation = 'không';
      mkeyExplain = `Gõ tắt thông minh: <strong>ko</strong> &rarr; <strong>không</strong>`;
      legacySimulation = 'ko';
    } else if (val === 'Ko') {
      mkeySimulation = 'Không';
      mkeyExplain = `Tự động viết hoa: <strong>Ko</strong> &rarr; <strong>Không</strong>`;
      legacySimulation = 'Ko';
    } else if (val === 'KO') {
      mkeySimulation = 'KHÔNG';
      mkeyExplain = `Tự động viết hoa toàn bộ: <strong>KO</strong> &rarr; <strong>KHÔNG</strong>`;
      legacySimulation = 'KO';
    } else if (lower === 'hieuer') {
      mkeySimulation = 'hiểu';
      mkeyExplain = `Tự động ghép đúng dấu: <strong>hiểu</strong>`;
    } else if (lower.includes('hieuer')) {
      mkeySimulation = val.replace(/hieuer/gi, 'hiểu');
      mkeyExplain = `Tự động ghép đúng dấu: <strong>${escapeHtml(mkeySimulation)}</strong>`;
    }

    // Simulate legacy IME swallowing double consonants or breaking words
    legacySimulation = legacySimulation
      .replace(/rr/gi, 'r')
      .replace(/ss/gi, 's')
      .replace(/ff/gi, 'f')
      .replace(/server/gi, 'sevơ')
      .replace(/port/gi, 'pọt')
      .replace(/part/gi, 'pạt')
      .replace(/after/gi, 'àter')
      .replace(/text/gi, 'tẽt');

    legacyDisplay.textContent = legacySimulation;
    legacyCaption.innerHTML = legacySimulation !== val
      ? `Bị nuốt chữ hoặc gõ sai: <strong>${escapeHtml(legacySimulation)}</strong>`
      : `Bộ gõ truyền thống`;

    mkeyDisplay.innerHTML = escapeHtml(mkeySimulation) + '<span class="cursor"></span>';
    mkeyCaption.innerHTML = mkeyExplain;
  });

  clearBtn.addEventListener('click', () => {
    liveInput.value = '';
    selectPreset('error');
    liveInput.focus();
  });

  function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }"####
}
