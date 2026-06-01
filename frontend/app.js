// Wrapper invokeCommand para Tauri IPC (v2)
export async function invokeCommand(cmd, args = {}) {
  if (!window.__TAURI__?.core?.invoke) {
    throw new Error('Tauri n\u00e3o est\u00e1 dispon\u00edvel. Execute dentro do aplicativo Tauri.');
  }
  const { invoke } = window.__TAURI__.core;
  try {
    return await invoke(cmd, args);
  } catch (e) {
    throw new Error(formatError(e));
  }
}

// Formatar erro para exibi\u00e7\u00e3o
export function formatError(error) {
  if (typeof error === 'string') return error;
  if (error?.message) return error.message;
  if (error?.error) return String(error.error);
  return 'Erro desconhecido';
}

// Estado global
export const state = {
  pastaRaiz: null,
  politica: 'sobrescrever',
  tema: 'escuro',
  destinos: [],        // { tipo: 'matricula'|'pasta', valor: string, label: string }
  arquivo: null,       // { nome: string, path: string }
};

// Mostrar toast
export function showToast(message, type = 'info', duration = 4000) {
  const container = document.getElementById('toast-container');
  if (!container) return;

  const toast = document.createElement('div');
  toast.className = `toast ${type}`;
  toast.setAttribute('role', 'alert');
  toast.textContent = message;

  container.appendChild(toast);

  if (duration > 0) {
    setTimeout(() => {
      toast.style.opacity = '0';
      toast.style.transform = 'translateX(100%)';
      toast.style.transition = 'opacity 0.3s, transform 0.3s';
      setTimeout(() => toast.remove(), 300);
    }, duration);
  }
}

// Tab navigation
export function initTabs() {
  const tabBtns = document.querySelectorAll('.tab-btn');
  const panels = document.querySelectorAll('.tab-panel');

  tabBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      const tab = btn.dataset.tab;
      switchTab(tab);
    });
  });
}

export function switchTab(tabId) {
  const tabBtns = document.querySelectorAll('.tab-btn');
  const panels = document.querySelectorAll('.tab-panel');

  tabBtns.forEach(btn => {
    const isActive = btn.dataset.tab === tabId;
    btn.classList.toggle('active', isActive);
    btn.setAttribute('aria-selected', String(isActive));
  });

  panels.forEach(panel => {
    const isActive = panel.id === `tab-${tabId}`;
    panel.classList.toggle('active', isActive);
    if (isActive) {
      panel.hidden = false;
    } else {
      panel.hidden = true;
    }
  });

  // Disparar evento para m\u00f3dulos saberem que a aba mudou
  window.dispatchEvent(new CustomEvent('tab-changed', { detail: { tab: tabId } }));
}

// Carregar configura\u00e7\u00e3o inicial
export async function carregarConfigInicial() {
  try {
    const config = await invokeCommand('carregar_config');
    if (config) {
      state.pastaRaiz = config.pasta_raiz || null;
      state.politica = config.politica_sobrescrita || 'sobrescrever';
      state.tema = config.tema || 'escuro';
    }
  } catch {
    // Se falhar, mant\u00e9m defaults
  }
}

// Helper: set loading state em bot\u00e3o
export function setButtonLoading(btn, isLoading) {
  const text = btn.querySelector('.btn-text');
  const spinner = btn.querySelector('.btn-spinner');
  if (text) text.hidden = isLoading;
  if (spinner) spinner.hidden = !isLoading;
  btn.disabled = isLoading;
}

// Helper: abrir link externo (pasta)
export async function abrirPasta(path) {
  try {
    await invokeCommand('plugin:shell|open', { path });
  } catch {
    showToast('N\u00e3o foi poss\u00edvel abrir a pasta', 'error');
  }
}

// Inicializar
initTabs();
carregarConfigInicial();

// Handler para links que mudam de aba
document.addEventListener('click', (e) => {
  const link = e.target.closest('a[data-goto-tab]');
  if (link) {
    e.preventDefault();
    switchTab(link.dataset.gotoTab);
  }
});
