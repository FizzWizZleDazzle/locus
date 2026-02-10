// Locus Factory - Frontend Logic

const API = 'http://localhost:9090';
let reviewProblems = [];
let stats = { generated: 0, approved: 0, rejected: 0 };

// ============================================================================
// Utilities
// ============================================================================

function toast(msg, type = 'info') {
  const el = document.createElement('div');
  el.className = `toast ${type}`;
  el.textContent = msg;
  document.getElementById('toastContainer').appendChild(el);
  setTimeout(() => el.remove(), 3000);
}

async function api(path, opts = {}) {
  const res = await fetch(`${API}${path}`, {
    headers: { 'Content-Type': 'application/json' },
    ...opts,
    body: opts.body ? JSON.stringify(opts.body) : undefined,
  });
  if (!res.ok) {
    const err = await res.json().catch(() => ({ detail: res.statusText }));
    throw new Error(err.detail || 'Request failed');
  }
  return res.json();
}

// ============================================================================
// Navigation
// ============================================================================

function switchTab(tabName) {
  document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
  document.querySelectorAll('.tab-content').forEach(t => t.classList.remove('active'));

  document.querySelector(`.tab[data-tab="${tabName}"]`).classList.add('active');
  document.getElementById(`tab-${tabName}`).classList.add('active');
}

// Setup tab click handlers
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('.tab').forEach(tab => {
    tab.addEventListener('click', () => {
      const tabName = tab.getAttribute('data-tab');
      switchTab(tabName);
    });
  });
});

// ============================================================================
// Init
// ============================================================================

async function init() {
  setInterval(() => {
    const now = new Date().toLocaleTimeString('en-US', { hour12: false });
    document.title = `Factory ${now}`;
  }, 1000);

  await checkConn();
  await refreshScripts();
  await refreshStaging();
}

async function checkConn() {
  try {
    const d = await api('/');
    document.getElementById('statusDot').classList.remove('offline');
    document.getElementById('statusLabel').textContent = 'Connected';

    // Load config
    const cfg = await api('/config');
    const statusEl = document.getElementById('configStatus');

    if (cfg.llm.configured) {
      document.getElementById('llmEndpoint').value = cfg.llm.endpoint || '';
      document.getElementById('llmKey').placeholder = cfg.llm.api_key || 'Not set';
      document.getElementById('llmModel').value = cfg.llm.model || '';

      statusEl.className = 'status-box ok';
      statusEl.textContent = `✓ Configured — ${cfg.llm.endpoint} | ${cfg.llm.model} | ${cfg.llm.api_key}`;
    } else {
      statusEl.className = 'status-box warn';
      statusEl.textContent = '⚠ Not configured — Enter credentials below';
    }
  } catch {
    document.getElementById('statusDot').classList.add('offline');
    document.getElementById('statusLabel').textContent = 'Offline';
    document.getElementById('configStatus').className = 'status-box error';
    document.getElementById('configStatus').textContent = '✗ Backend offline';
  }
}

// ============================================================================
// Config
// ============================================================================

async function saveLLMConfig() {
  const endpoint = document.getElementById('llmEndpoint').value;
  const key = document.getElementById('llmKey').value;
  const model = document.getElementById('llmModel').value;

  if (!endpoint || !key || !model) {
    toast('All fields required', 'error');
    return;
  }

  try {
    await api('/config/llm', {
      method: 'POST',
      body: { endpoint, api_key: key, model }
    });
    toast('Config saved', 'success');
    await checkConn();
  } catch (e) {
    toast('Save failed: ' + e.message, 'error');
  }
}

// ============================================================================
// Script Generation
// ============================================================================

async function generateScript() {
  const topic = document.getElementById('genTopic').value;
  const subtopic = document.getElementById('genSubtopic').value;
  const difficulty = document.getElementById('genDifficulty').value;
  const mode = document.getElementById('genMode').value;

  if (!topic || !subtopic) {
    toast('Topic and subtopic required', 'error');
    return;
  }

  const btn = document.getElementById('genBtn');
  btn.classList.add('loading');
  btn.textContent = 'Generating...';

  try {
    const d = await api('/generate-script', {
      method: 'POST',
      body: { main_topic: topic, subtopic, difficulty_level: difficulty, grading_mode: mode }
    });

    document.getElementById('scriptEditor').value = d.script;
    toast('Script generated', 'success');
  } catch (e) {
    toast('Failed: ' + e.message, 'error');
  } finally {
    btn.classList.remove('loading');
    btn.textContent = 'Generate Script';
  }
}

async function testScript() {
  const script = document.getElementById('scriptEditor').value;
  if (!script) {
    toast('No script', 'error');
    return;
  }

  try {
    const d = await api('/test-script', { method: 'POST', body: { script } });
    const resultEl = document.getElementById('testResult');

    if (d.success) {
      resultEl.style.display = 'block';
      resultEl.innerHTML = `
        <div style="background:rgba(16,185,129,.1);border:1px solid rgba(16,185,129,.3);border-radius:4px;padding:10px">
          <div style="font-family:var(--mono);font-size:11px;color:var(--green);margin-bottom:6px;font-weight:600">✓ Test Passed</div>
          <div style="font-size:12px;margin-bottom:4px">${d.problem.question_latex}</div>
          <div style="font-family:var(--mono);font-size:10px;color:var(--text-dim)">Answer: <code>${d.problem.answer_key}</code></div>
        </div>
      `;
      toast('Test passed', 'success');
    } else {
      resultEl.style.display = 'block';
      resultEl.innerHTML = `
        <div style="background:rgba(239,68,68,.1);border:1px solid rgba(239,68,68,.3);border-radius:4px;padding:10px">
          <div style="font-family:var(--mono);font-size:11px;color:var(--red);margin-bottom:4px;font-weight:600">✗ Failed</div>
          <pre style="font-size:10px;color:var(--text-muted);overflow:auto">${d.error}</pre>
        </div>
      `;
      toast('Test failed', 'error');
    }
  } catch (e) {
    toast('Error: ' + e.message, 'error');
  }
}

function saveScriptDialog() {
  const script = document.getElementById('scriptEditor').value;
  if (!script) {
    toast('No script', 'error');
    return;
  }

  const name = prompt('Script name:');
  if (!name) return;

  const desc = prompt('Description (optional):', '');
  saveScript(name, script, desc);
}

async function saveScript(name, script, desc) {
  try {
    await api('/scripts/save', {
      method: 'POST',
      body: { name, script, description: desc || '' }
    });
    toast('Saved', 'success');
    await refreshScripts();
  } catch (e) {
    toast('Save failed: ' + e.message, 'error');
  }
}

function clearScript() {
  document.getElementById('scriptEditor').value = '';
  document.getElementById('testResult').style.display = 'none';
}

// ============================================================================
// Script Library
// ============================================================================

async function refreshScripts() {
  try {
    const d = await api('/scripts');
    const list = document.getElementById('scriptList');

    if (d.count === 0) {
      list.innerHTML = '<div class="empty" style="padding:40px 10px"><div class="empty-icon">📜</div><p style="font-size:11px">No scripts</p></div>';
      return;
    }

    list.innerHTML = '';
    d.scripts.forEach(s => {
      const item = document.createElement('div');
      item.className = 'list-item';
      item.onclick = () => loadScript(s.name);
      item.innerHTML = `
        <div class="list-item-title">${s.name}</div>
        <div class="list-item-desc">${s.description}</div>
        <div class="list-item-meta">
          <span>${new Date(s.created).toLocaleDateString()}</span>
          <button class="btn btn-sm" style="background:rgba(59,130,246,.12);color:var(--blue);border:none;padding:2px 7px" onclick="event.stopPropagation();runScriptDialog('${s.name}')">RUN</button>
        </div>
      `;
      list.appendChild(item);
    });
  } catch (e) {
    toast('Load failed: ' + e.message, 'error');
  }
}

async function loadScript(name) {
  try {
    const d = await api(`/scripts/${name}`);
    document.getElementById('scriptEditor').value = d.script;
    switchTab('generate');
    toast(`Loaded ${name}`, 'success');
  } catch (e) {
    toast('Load failed: ' + e.message, 'error');
  }
}

function runScriptDialog(name) {
  const count = prompt('How many problems? (1-50 for review, or type "mass" for 1000)', '10');
  if (!count) return;

  if (count.toLowerCase() === 'mass') {
    massGenerate(name);
  } else {
    runScript(name, parseInt(count));
  }
}

async function runScript(name, count) {
  try {
    toast(`Running ${name}...`, 'info');
    const d = await api('/run-script', {
      method: 'POST',
      body: { script_name: name, count }
    });

    if (d.success) {
      reviewProblems = d.problems;
      stats.generated += d.count;
      renderProblems();
      switchTab('review');
      toast(`Generated ${d.count} problems`, 'success');
    } else {
      toast('Run failed', 'error');
    }
  } catch (e) {
    toast('Error: ' + e.message, 'error');
  }
}

async function massGenerate(name) {
  const count = parseInt(prompt('Mass generate count:', '1000'));
  if (!count || count < 100) {
    toast('Enter a number >= 100', 'error');
    return;
  }

  try {
    toast(`Mass generating ${count} problems...`, 'info');
    const d = await api('/mass-generate', {
      method: 'POST',
      body: { script_name: name, count }
    });

    if (d.success) {
      toast(`Generated ${d.generated}, ${d.staged} staged`, 'success');
      await refreshStaging();
    } else {
      toast('Mass generation failed', 'error');
    }
  } catch (e) {
    toast('Error: ' + e.message, 'error');
  }
}

// ============================================================================
// Problem Review
// ============================================================================

function renderProblems() {
  const container = document.getElementById('problemsContainer');
  const actions = document.getElementById('reviewActions');
  const count = document.getElementById('reviewCount');

  if (reviewProblems.length === 0) {
    container.innerHTML = '<div class="empty"><div class="empty-icon">?</div><p>Run a script to generate problems</p></div>';
    actions.style.display = 'none';
    count.textContent = '0';
    return;
  }

  actions.style.display = 'flex';
  count.textContent = reviewProblems.length;
  container.innerHTML = '';

  reviewProblems.forEach((p, i) => {
    const card = document.createElement('div');
    card.className = 'card';
    card.id = `card-${i}`;

    const diffClass = p.difficulty < 1300 ? 'diff-easy' : p.difficulty < 1600 ? 'diff-med' : 'diff-hard';

    card.innerHTML = `
      <div class="card-header">
        <div class="tags">
          <span class="tag tag-topic">${p.main_topic}</span>
          <span class="tag tag-sub">${p.subtopic.replace(/_/g, ' ')}</span>
          <span class="tag tag-mode">${p.grading_mode}</span>
        </div>
        <span class="diff ${diffClass}">${p.difficulty}</span>
      </div>
      <div class="question" id="q-${i}">${p.question_latex}</div>
      <div class="answer">Answer: <code>${p.answer_key}</code></div>
      <div class="actions">
        <button class="btn btn-danger btn-sm" onclick="rejectProblem(${i})">Reject</button>
        <button class="btn btn-success btn-sm" onclick="approveProblem(${i})">Approve</button>
      </div>
    `;
    container.appendChild(card);
  });

  // Render LaTeX
  requestAnimationFrame(() => {
    reviewProblems.forEach((_, i) => {
      const el = document.getElementById(`q-${i}`);
      if (el && window.renderMathInElement) {
        renderMathInElement(el, {
          delimiters: [
            { left: '$$', right: '$$', display: true },
            { left: '$', right: '$', display: false }
          ],
          throwOnError: false
        });
      }
    });
  });
}

async function approveProblem(i) {
  const p = reviewProblems[i];
  if (!p) return;

  try {
    await api('/confirm-problem', {
      method: 'POST',
      body: { problem: p, approved: true }
    });

    document.getElementById(`card-${i}`).classList.add('approved');
    reviewProblems[i] = null;
    stats.approved++;
    await refreshStaging();

    setTimeout(() => {
      reviewProblems = reviewProblems.filter(x => x);
      renderProblems();
    }, 300);
  } catch (e) {
    toast('Approve failed: ' + e.message, 'error');
  }
}

function rejectProblem(i) {
  document.getElementById(`card-${i}`).classList.add('rejected');
  reviewProblems[i] = null;
  stats.rejected++;

  setTimeout(() => {
    reviewProblems = reviewProblems.filter(x => x);
    renderProblems();
  }, 300);
}

async function approveAll() {
  const active = reviewProblems.filter(x => x);
  for (const p of active) {
    try {
      await api('/confirm-problem', {
        method: 'POST',
        body: { problem: p, approved: true }
      });
      stats.approved++;
    } catch { }
  }

  reviewProblems = [];
  renderProblems();
  await refreshStaging();
  toast(`Approved ${active.length}`, 'success');
}

function clearReview() {
  stats.rejected += reviewProblems.filter(x => x).length;
  reviewProblems = [];
  renderProblems();
}

// ============================================================================
// Staging
// ============================================================================

async function refreshStaging() {
  try {
    const d = await api('/staged');
    document.getElementById('stagingCount').textContent = d.count;
    document.getElementById('stagedBadge').textContent = `${d.count} staged`;

    const list = document.getElementById('stagingList');
    if (d.count === 0) {
      list.innerHTML = '<div class="empty" style="padding:40px 10px"><p style="font-size:11px">No problems staged</p></div>';
    } else {
      list.innerHTML = '';
      d.problems.forEach(p => {
        const item = document.createElement('div');
        item.style.cssText = 'font-family:var(--mono);font-size:10px;padding:6px 8px;border-bottom:1px solid var(--border);color:var(--text-dim);display:flex;gap:8px;align-items:center';

        const preview = p.question_latex.replace(/\$/g, '').substring(0, 35);
        const diffClass = p.difficulty < 1300 ? 'diff-easy' : p.difficulty < 1600 ? 'diff-med' : 'diff-hard';

        item.innerHTML = `
          <span style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">${preview}</span>
          <span class="diff ${diffClass}" style="font-size:9px;padding:2px 5px">${p.difficulty}</span>
        `;
        list.appendChild(item);
      });
    }
  } catch { }
}

async function clearStaging() {
  try {
    await api('/staged', { method: 'DELETE' });
    await refreshStaging();
    toast('Cleared', 'success');
  } catch (e) {
    toast('Clear failed: ' + e.message, 'error');
  }
}

async function massGenerate(name) {
  const count = parseInt(prompt('Problems per script (e.g., 100):', '100'));
  if (!count || count < 1) return;

  if (!confirm(`This will run ALL saved scripts ${count} times each. Continue?`)) return;

  try {
    toast('Mass generating...', 'info');
    const d = await api('/mass-generate', {
      method: 'POST',
      body: { count_per_script: count }
    });

    if (d.success) {
      toast(`Generated ${d.total_generated} from ${d.scripts_run} scripts`, 'success');
      await refreshStaging();
    } else {
      toast('Mass generation failed', 'error');
    }
  } catch (e) {
    toast('Error: ' + e.message, 'error');
  }
}

// ============================================================================
// Export
// ============================================================================

async function exportSQL() {
  try {
    const d = await api('/export', { method: 'POST', body: { format: 'sql' } });
    toast(`Saved to ${d.filename}`, 'success');
    await refreshStaging();
  } catch (e) {
    toast('Export failed: ' + e.message, 'error');
  }
}

async function viewExports() {
  try {
    const d = await api('/exports');

    if (d.count === 0) {
      toast('No exports yet', 'info');
      return;
    }

    let html = '<div style="background:var(--bg-2);border:1px solid var(--border);border-radius:4px;padding:12px;max-width:500px">';
    html += '<div style="font-family:var(--mono);font-size:11px;font-weight:600;margin-bottom:10px">Exported Files</div>';
    html += '<div style="display:flex;flex-direction:column;gap:6px">';

    d.exports.forEach(ex => {
      html += `
        <div style="background:var(--bg);border:1px solid var(--border);border-radius:3px;padding:8px;display:flex;justify-content:space-between;align-items:center;gap:8px">
          <div>
            <div style="font-family:var(--mono);font-size:11px">${ex.filename}</div>
            <div style="font-size:9px;color:var(--text-muted)">${new Date(ex.created).toLocaleString()} · ${(ex.size / 1024).toFixed(1)}KB</div>
          </div>
          <div style="display:flex;gap:4px">
            <button class="btn btn-sm" style="padding:3px 7px;font-size:9px" onclick="downloadExport('${ex.filename}')">Download</button>
          </div>
        </div>
      `;
    });

    html += '</div></div>';

    // Show in a simple modal/overlay
    const overlay = document.createElement('div');
    overlay.style.cssText = 'position:fixed;inset:0;background:rgba(0,0,0,.7);display:flex;align-items:center;justify-content:center;z-index:10000';
    overlay.innerHTML = html;
    overlay.onclick = (e) => {
      if (e.target === overlay) overlay.remove();
    };
    document.body.appendChild(overlay);

  } catch (e) {
    toast('Failed to load exports: ' + e.message, 'error');
  }
}

async function downloadExport(filename) {
  try {
    const d = await api(`/exports/${filename}`);
    const blob = new Blob([d.content], { type: filename.endsWith('.sql') ? 'text/sql' : 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    a.click();
    URL.revokeObjectURL(url);
    toast('Downloaded', 'success');
  } catch (e) {
    toast('Download failed: ' + e.message, 'error');
  }
}

// ============================================================================
// Init on load
// ============================================================================

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}
