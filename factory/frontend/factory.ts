// Locus Factory - Frontend Logic (TypeScript)

// ============================================================================
// Types & Interfaces
// ============================================================================

interface Problem {
  id?: string;
  question_latex: string;
  answer_key: string;
  difficulty: number;
  main_topic: string;
  subtopic: string;
  grading_mode: string;
  generated_at?: string;
}

interface Stats {
  generated: number;
  approved: number;
  rejected: number;
}

type ToastType = 'info' | 'success' | 'error';

interface ScriptResponse {
  script: string;
  message: string;
}

interface TestScriptResponse {
  success: boolean;
  problem?: Problem;
  message?: string;
  error?: string;
  output?: string;
}

interface ScriptInfo {
  name: string;
  filename: string;
  description: string;
  created: string;
}

interface ScriptsResponse {
  scripts: ScriptInfo[];
  count: number;
}

interface RunScriptResponse {
  success: boolean;
  problems: Problem[];
  count: number;
  errors?: string[];
}

interface MassGenerateResponse {
  success: boolean;
  total_generated: number;
  staged: number;
  scripts_run: number;
  per_script: Record<string, number>;
  errors?: string[];
  message: string;
}

interface StagedResponse {
  problems: Problem[];
  count: number;
}

interface ExportResponse {
  format: string;
  filename: string;
  path: string;
  count: number;
  message: string;
}

interface ExportFile {
  filename: string;
  format: string;
  size: number;
  created: string;
}

interface ExportsResponse {
  exports: ExportFile[];
  count: number;
}

interface DownloadExportResponse {
  filename: string;
  content: string;
  size: number;
}

interface LLMConfig {
  endpoint: string;
  api_key: string;
  model: string;
  configured: boolean;
}

interface LocusConfig {
  backend_url: string;
  api_key: string | null;
}

interface ConfigResponse {
  llm: LLMConfig;
  locus: LocusConfig;
}

// ============================================================================
// State
// ============================================================================

const API = 'http://localhost:9090';
let reviewProblems: (Problem | null)[] = [];
const stats: Stats = { generated: 0, approved: 0, rejected: 0 };

// ============================================================================
// Utilities
// ============================================================================

function toast(msg: string, type: ToastType = 'info'): void {
  const el = document.createElement('div');
  el.className = `toast ${type}`;
  el.textContent = msg;
  const container = document.getElementById('toastContainer');
  if (container) {
    container.appendChild(el);
    setTimeout(() => el.remove(), 3000);
  }
}

interface ApiOptions extends Omit<RequestInit, 'body'> {
  body?: Record<string, unknown>;
}

async function api<T = unknown>(path: string, opts: ApiOptions = {}): Promise<T> {
  const fetchOpts: RequestInit = {
    ...opts,
    headers: { 'Content-Type': 'application/json', ...opts.headers },
    body: opts.body ? JSON.stringify(opts.body) : undefined,
  };

  const res = await fetch(`${API}${path}`, fetchOpts);

  if (!res.ok) {
    const err = await res.json().catch(() => ({ detail: res.statusText })) as { detail?: string };
    throw new Error(err.detail || 'Request failed');
  }

  return res.json() as Promise<T>;
}

function getElement<T extends HTMLElement>(id: string): T | null {
  return document.getElementById(id) as T | null;
}

function requireElement<T extends HTMLElement>(id: string): T {
  const el = getElement<T>(id);
  if (!el) throw new Error(`Element #${id} not found`);
  return el;
}

// ============================================================================
// Navigation
// ============================================================================

function switchTab(tabName: string): void {
  document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
  document.querySelectorAll('.tab-content').forEach(t => t.classList.remove('active'));

  const tab = document.querySelector(`.tab[data-tab="${tabName}"]`);
  const content = document.getElementById(`tab-${tabName}`);

  if (tab) tab.classList.add('active');
  if (content) content.classList.add('active');
}

// Setup tab click handlers
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('.tab').forEach(tab => {
    tab.addEventListener('click', () => {
      const tabName = tab.getAttribute('data-tab');
      if (tabName) switchTab(tabName);
    });
  });
});

// ============================================================================
// Init
// ============================================================================

async function init(): Promise<void> {
  setInterval(() => {
    const now = new Date().toLocaleTimeString('en-US', { hour12: false });
    document.title = `Factory ${now}`;
  }, 1000);

  await checkConn();
  await refreshScripts();
  await refreshStaging();
}

async function checkConn(): Promise<void> {
  try {
    await api('/');

    const statusDot = getElement('statusDot');
    const statusLabel = getElement('statusLabel');

    if (statusDot) statusDot.classList.remove('offline');
    if (statusLabel) statusLabel.textContent = 'Connected';

    // Load config
    const cfg = await api<ConfigResponse>('/config');
    const statusEl = getElement('configStatus');

    if (cfg.llm.configured) {
      const llmEndpoint = getElement<HTMLInputElement>('llmEndpoint');
      const llmKey = getElement<HTMLInputElement>('llmKey');
      const llmModel = getElement<HTMLInputElement>('llmModel');

      if (llmEndpoint) llmEndpoint.value = cfg.llm.endpoint || '';
      if (llmKey) llmKey.placeholder = cfg.llm.api_key || 'Not set';
      if (llmModel) llmModel.value = cfg.llm.model || '';

      if (statusEl) {
        statusEl.className = 'status-box ok';
        statusEl.textContent = `✓ Configured — ${cfg.llm.endpoint} | ${cfg.llm.model} | ${cfg.llm.api_key}`;
      }
    } else {
      if (statusEl) {
        statusEl.className = 'status-box warn';
        statusEl.textContent = '⚠ Not configured — Enter credentials below';
      }
    }
  } catch {
    const statusDot = getElement('statusDot');
    const statusLabel = getElement('statusLabel');
    const statusEl = getElement('configStatus');

    if (statusDot) statusDot.classList.add('offline');
    if (statusLabel) statusLabel.textContent = 'Offline';
    if (statusEl) {
      statusEl.className = 'status-box error';
      statusEl.textContent = '✗ Backend offline';
    }
  }
}

// ============================================================================
// Config
// ============================================================================

async function saveLLMConfig(): Promise<void> {
  const endpoint = requireElement<HTMLInputElement>('llmEndpoint').value;
  const key = requireElement<HTMLInputElement>('llmKey').value;
  const model = requireElement<HTMLInputElement>('llmModel').value;

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
    toast('Save failed: ' + (e as Error).message, 'error');
  }
}

// ============================================================================
// Script Generation
// ============================================================================

async function generateScript(): Promise<void> {
  const topic = requireElement<HTMLInputElement>('genTopic').value;
  const subtopic = requireElement<HTMLInputElement>('genSubtopic').value;
  const difficulty = requireElement<HTMLSelectElement>('genDifficulty').value;
  const mode = requireElement<HTMLSelectElement>('genMode').value;

  if (!topic || !subtopic) {
    toast('Topic and subtopic required', 'error');
    return;
  }

  const btn = requireElement<HTMLButtonElement>('genBtn');
  btn.classList.add('loading');
  btn.textContent = 'Generating...';

  try {
    const d = await api<ScriptResponse>('/generate-script', {
      method: 'POST',
      body: { main_topic: topic, subtopic, difficulty_level: difficulty, grading_mode: mode }
    });

    requireElement<HTMLTextAreaElement>('scriptEditor').value = d.script;
    toast('Script generated', 'success');
  } catch (e) {
    toast('Failed: ' + (e as Error).message, 'error');
  } finally {
    btn.classList.remove('loading');
    btn.textContent = 'Generate Script';
  }
}

async function testScript(): Promise<void> {
  const script = requireElement<HTMLTextAreaElement>('scriptEditor').value;
  if (!script) {
    toast('No script', 'error');
    return;
  }

  try {
    const d = await api<TestScriptResponse>('/test-script', { method: 'POST', body: { script } });
    const resultEl = requireElement('testResult');

    if (d.success && d.problem) {
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
          <pre style="font-size:10px;color:var(--text-muted);overflow:auto">${d.error || 'Unknown error'}</pre>
        </div>
      `;
      toast('Test failed', 'error');
    }
  } catch (e) {
    toast('Error: ' + (e as Error).message, 'error');
  }
}

function saveScriptDialog(): void {
  const script = requireElement<HTMLTextAreaElement>('scriptEditor').value;
  if (!script) {
    toast('No script', 'error');
    return;
  }

  const name = prompt('Script name:');
  if (!name) return;

  const desc = prompt('Description (optional):', '');
  void saveScript(name, script, desc || '');
}

async function saveScript(name: string, script: string, desc: string): Promise<void> {
  try {
    await api('/scripts/save', {
      method: 'POST',
      body: { name, script, description: desc }
    });
    toast('Saved', 'success');
    await refreshScripts();
  } catch (e) {
    toast('Save failed: ' + (e as Error).message, 'error');
  }
}

function clearScript(): void {
  requireElement<HTMLTextAreaElement>('scriptEditor').value = '';
  const testResult = getElement('testResult');
  if (testResult) testResult.style.display = 'none';
}

// ============================================================================
// Script Library
// ============================================================================

async function refreshScripts(): Promise<void> {
  try {
    const d = await api<ScriptsResponse>('/scripts');
    const list = requireElement('scriptList');

    if (d.count === 0) {
      list.innerHTML = '<div class="empty" style="padding:40px 10px"><div class="empty-icon">📜</div><p style="font-size:11px">No scripts</p></div>';
      return;
    }

    list.innerHTML = '';
    d.scripts.forEach(s => {
      const item = document.createElement('div');
      item.className = 'list-item';
      item.onclick = () => void loadScript(s.name);
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
    toast('Load failed: ' + (e as Error).message, 'error');
  }
}

async function loadScript(name: string): Promise<void> {
  try {
    const d = await api<{ script: string }>(`/scripts/${name}`);
    requireElement<HTMLTextAreaElement>('scriptEditor').value = d.script;
    switchTab('generate');
    toast(`Loaded ${name}`, 'success');
  } catch (e) {
    toast('Load failed: ' + (e as Error).message, 'error');
  }
}

function runScriptDialog(name: string): void {
  const count = prompt('How many problems? (1-50 for review, or type "mass" for 1000)', '10');
  if (!count) return;

  if (count.toLowerCase() === 'mass') {
    void massGenerateSingle(name);
  } else {
    void runScript(name, parseInt(count));
  }
}

async function runScript(name: string, count: number): Promise<void> {
  try {
    toast(`Running ${name}...`, 'info');
    const d = await api<RunScriptResponse>('/run-script', {
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
    toast('Error: ' + (e as Error).message, 'error');
  }
}

async function massGenerateSingle(_name: string): Promise<void> {
  const count = parseInt(prompt('Mass generate count:', '1000') || '0');
  if (!count || count < 100) {
    toast('Enter a number >= 100', 'error');
    return;
  }

  try {
    toast(`Mass generating ${count} problems...`, 'info');
    const d = await api<MassGenerateResponse>('/mass-generate', {
      method: 'POST',
      body: { count_per_script: count }
    });

    if (d.success) {
      toast(`Generated ${d.total_generated}, ${d.staged} staged`, 'success');
      await refreshStaging();
    } else {
      toast('Mass generation failed', 'error');
    }
  } catch (e) {
    toast('Error: ' + (e as Error).message, 'error');
  }
}

// ============================================================================
// Problem Review
// ============================================================================

function renderProblems(): void {
  const container = requireElement('problemsContainer');
  const actions = getElement('reviewActions');
  const count = getElement('reviewCount');

  if (reviewProblems.length === 0) {
    container.innerHTML = '<div class="empty"><div class="empty-icon">?</div><p>Run a script to generate problems</p></div>';
    if (actions) actions.style.display = 'none';
    if (count) count.textContent = '0';
    return;
  }

  if (actions) actions.style.display = 'flex';
  if (count) count.textContent = reviewProblems.length.toString();
  container.innerHTML = '';

  reviewProblems.forEach((p, i) => {
    if (!p) return;

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
    reviewProblems.forEach((p, i) => {
      if (!p) return;
      const el = document.getElementById(`q-${i}`);
      if (el && (window as any).renderMathInElement) {
        (window as any).renderMathInElement(el, {
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

async function approveProblem(i: number): Promise<void> {
  const p = reviewProblems[i];
  if (!p) return;

  try {
    await api('/confirm-problem', {
      method: 'POST',
      body: { problem: p, approved: true }
    });

    const card = getElement(`card-${i}`);
    if (card) card.classList.add('approved');
    reviewProblems[i] = null;
    stats.approved++;
    await refreshStaging();

    setTimeout(() => {
      reviewProblems = reviewProblems.filter(x => x);
      renderProblems();
    }, 300);
  } catch (e) {
    toast('Approve failed: ' + (e as Error).message, 'error');
  }
}

function rejectProblem(i: number): void {
  const card = getElement(`card-${i}`);
  if (card) card.classList.add('rejected');
  reviewProblems[i] = null;
  stats.rejected++;

  setTimeout(() => {
    reviewProblems = reviewProblems.filter(x => x);
    renderProblems();
  }, 300);
}

async function approveAll(): Promise<void> {
  const active = reviewProblems.filter((x): x is Problem => x !== null);
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

function clearReview(): void {
  stats.rejected += reviewProblems.filter(x => x).length;
  reviewProblems = [];
  renderProblems();
}

// ============================================================================
// Staging
// ============================================================================

async function refreshStaging(): Promise<void> {
  try {
    const d = await api<StagedResponse>('/staged');

    const stagingCount = getElement('stagingCount');
    const stagedBadge = getElement('stagedBadge');

    if (stagingCount) stagingCount.textContent = d.count.toString();
    if (stagedBadge) stagedBadge.textContent = `${d.count} staged`;

    const list = requireElement('stagingList');
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

async function clearStaging(): Promise<void> {
  try {
    await api('/staged', { method: 'DELETE' });
    await refreshStaging();
    toast('Cleared', 'success');
  } catch (e) {
    toast('Clear failed: ' + (e as Error).message, 'error');
  }
}

async function massGenerate(): Promise<void> {
  const count = parseInt(prompt('Problems per script (e.g., 100):', '100') || '0');
  if (!count || count < 1) return;

  if (!confirm(`This will run ALL saved scripts ${count} times each. Continue?`)) return;

  try {
    toast('Mass generating...', 'info');
    const d = await api<MassGenerateResponse>('/mass-generate', {
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
    toast('Error: ' + (e as Error).message, 'error');
  }
}

// ============================================================================
// Export
// ============================================================================

async function exportSQL(): Promise<void> {
  try {
    const d = await api<ExportResponse>('/export', { method: 'POST', body: { format: 'sql' } });
    toast(`Saved to ${d.filename}`, 'success');
    await refreshStaging();
  } catch (e) {
    toast('Export failed: ' + (e as Error).message, 'error');
  }
}

async function viewExports(): Promise<void> {
  try {
    const d = await api<ExportsResponse>('/exports');

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
    toast('Failed to load exports: ' + (e as Error).message, 'error');
  }
}

async function downloadExport(filename: string): Promise<void> {
  try {
    const d = await api<DownloadExportResponse>(`/exports/${filename}`);
    const blob = new Blob([d.content], { type: filename.endsWith('.sql') ? 'text/sql' : 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    a.click();
    URL.revokeObjectURL(url);
    toast('Downloaded', 'success');
  } catch (e) {
    toast('Download failed: ' + (e as Error).message, 'error');
  }
}

// ============================================================================
// Global functions (exposed to window for onclick handlers)
// ============================================================================

(window as any).saveLLMConfig = saveLLMConfig;
(window as any).generateScript = generateScript;
(window as any).testScript = testScript;
(window as any).saveScriptDialog = saveScriptDialog;
(window as any).clearScript = clearScript;
(window as any).runScriptDialog = runScriptDialog;
(window as any).approveProblem = approveProblem;
(window as any).rejectProblem = rejectProblem;
(window as any).approveAll = approveAll;
(window as any).clearReview = clearReview;
(window as any).massGenerate = massGenerate;
(window as any).exportSQL = exportSQL;
(window as any).viewExports = viewExports;
(window as any).downloadExport = downloadExport;
(window as any).clearStaging = clearStaging;

// ============================================================================
// Init on load
// ============================================================================

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', () => void init());
} else {
  void init();
}
