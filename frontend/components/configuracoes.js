import {
  invokeCommand,
  state,
  showToast,
  setButtonLoading,
} from '../app.js';

// ============================================================================
// Elementos
// ============================================================================

const elPastaRaiz = document.getElementById('pasta-raiz');
const elBtnSelecionarRaiz = document.getElementById('btn-selecionar-raiz');
const elAvisoPastaNaoConfig = document.getElementById('aviso-pasta-nao-configurada');
const elBtnSalvar = document.getElementById('btn-salvar-config');

// ============================================================================
// T-50: Carregar config ao abrir aba
// ============================================================================

async function carregarConfig() {
  try {
    const config = await invokeCommand('carregar_config');
    state.pastaRaiz = config.pasta_raiz || null;
    state.politica = config.politica_sobrescrita || 'sobrescrever';
    state.tema = config.tema || 'escuro';

    elPastaRaiz.value = state.pastaRaiz || '';
    atualizarAvisoPasta();
    selecionarRadio('politica', state.politica);
    selecionarRadio('tema', state.tema);
  } catch (e) {
    showToast('Erro ao carregar configura\u00e7\u00f5es', 'error');
  }
}

function atualizarAvisoPasta() {
  if (state.pastaRaiz) {
    elAvisoPastaNaoConfig.classList.add('hidden');
  } else {
    elAvisoPastaNaoConfig.classList.remove('hidden');
  }
}

function selecionarRadio(name, valor) {
  const radio = document.querySelector(`input[name="${name}"][value="${valor}"]`);
  if (radio) radio.checked = true;
}

function obterRadioSelecionado(name) {
  const radio = document.querySelector(`input[name="${name}"]:checked`);
  return radio ? radio.value : null;
}

// Carrega ao iniciar e ao trocar para aba config
window.addEventListener('tab-changed', (e) => {
  if (e.detail.tab === 'config') {
    carregarConfig();
  }
});
carregarConfig();

// ============================================================================
// T-51: Campo pasta raiz
// ============================================================================

elBtnSelecionarRaiz.addEventListener('click', async () => {
  try {
    const path = await invokeCommand('selecionar_pasta_raiz');
    if (path) {
      state.pastaRaiz = path;
      elPastaRaiz.value = path;
      atualizarAvisoPasta();
      showToast('Pasta raiz selecionada', 'success');
    }
  } catch (e) {
    showToast(e.message || 'Erro ao selecionar pasta', 'error');
  }
});

// ============================================================================
// T-52: Radio pol\u00edtica (eventos para atualizar estado)
// ============================================================================

document.querySelectorAll('input[name="politica"]').forEach(radio => {
  radio.addEventListener('change', (e) => {
    state.politica = e.target.value;
  });
});

// ============================================================================
// T-53: Seletor de tema (placeholder)
// ============================================================================

document.querySelectorAll('input[name="tema"]').forEach(radio => {
  radio.addEventListener('change', (e) => {
    state.tema = e.target.value;
  });
});

// ============================================================================
// T-54 / T-55: Salvar config + toast
// ============================================================================

elBtnSalvar.addEventListener('click', async () => {
  setButtonLoading(elBtnSalvar, true);

  try {
    await invokeCommand('salvar_config', {
      config: {
        pasta_raiz: state.pastaRaiz || null,
        politica_sobrescrita: state.politica,
        tema: state.tema,
      },
    });
    showToast('Configura\u00e7\u00f5es salvas com sucesso', 'success');
  } catch (e) {
    showToast(e.message || 'Erro ao salvar configura\u00e7\u00f5es', 'error');
  } finally {
    setButtonLoading(elBtnSalvar, false);
  }
});
