import {
  invokeCommand,
  state,
  showToast,
  setButtonLoading,
} from '../app.js';

// ============================================================================
// Elementos
// ============================================================================

const elMatricula = document.getElementById('matricula-envio');
const elBtnAdicionar = document.getElementById('btn-adicionar');
const elErroMatricula = document.getElementById('erro-matricula-envio');
const elListaDestinos = document.getElementById('lista-destinos');
const elDestinosEmpty = document.getElementById('destinos-empty');
const elBtnAddPasta = document.getElementById('btn-add-pasta');
const elBtnSelecionarArquivo = document.getElementById('btn-selecionar-arquivo');
const elArquivoSelecionado = document.getElementById('arquivo-selecionado');
const elBtnLimparArquivo = document.getElementById('btn-limpar-arquivo');
const elBtnEnviar = document.getElementById('btn-enviar');
const elLog = document.getElementById('log-envio');
const elBtnLimparLog = document.getElementById('btn-limpar-log');

const modalNovo = document.getElementById('modal-novo-militar');
const modalNovoTitulo = document.getElementById('modal-novo-titulo');
const modalMatricula = document.getElementById('modal-matricula');
const modalNome = document.getElementById('modal-nome');
const modalBtnCriar = document.getElementById('modal-btn-criar');
const modalBtnPular = document.getElementById('modal-btn-pular');
const modalBtnCancelar = document.getElementById('modal-btn-cancelar');

// ============================================================================
// SVGs inline (Heroicons-style)
// ============================================================================

const svgPessoa = `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path><circle cx="12" cy="7" r="4"></circle></svg>`;
const svgPasta = `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>`;
const svgX = `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>`;
const svgCheck = `<svg class="log-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><polyline points="20 6 9 17 4 12"></polyline></svg>`;
const svgErro = `<svg class="log-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>`;

// ============================================================================
// T-39: Campo matr\u00edcula
// ============================================================================

function normalizarMatricula(valor) {
  return valor.trim().replace(/\s/g, '');
}

function validarMatricula(valor) {
  const v = normalizarMatricula(valor);
  if (!v) return { ok: false, erro: 'Digite uma matr\u00edcula' };
  if (!/^\d{3,8}(-\d)?$/.test(v)) {
    return { ok: false, erro: 'Matr\u00edcula inv\u00e1lida. Use at\u00e9 8 d\u00edgitos, ex: 1111111 ou 111111-1' };
  }
  return { ok: true, valor: v };
}

async function adicionarMatricula() {
  elErroMatricula.textContent = '';
  const { ok, erro, valor } = validarMatricula(elMatricula.value);
  if (!ok) {
    elErroMatricula.textContent = erro;
    elMatricula.focus();
    return;
  }

  // Verifica duplicado
  if (state.destinos.some(d => d.tipo === 'matricula' && d.valor === valor)) {
    elErroMatricula.textContent = 'Matr\u00edcula j\u00e1 adicionada';
    return;
  }

  try {
    const info = await invokeCommand('adicionar_matricula', { matricula: valor });
    const label = info.com_hifen || info.digitos;
    state.destinos.push({ tipo: 'matricula', valor: info.digitos, label });
    elMatricula.value = '';
    renderDestinos();
    atualizarEstadoEnviar();
  } catch (e) {
    elErroMatricula.textContent = e.message || 'Erro ao adicionar matr\u00edcula';
  }
}

elBtnAdicionar.addEventListener('click', adicionarMatricula);
elMatricula.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') {
    e.preventDefault();
    adicionarMatricula();
  }
});

// ============================================================================
// T-40: Lista de destinos (badges)
// ============================================================================

function renderDestinos() {
  elListaDestinos.innerHTML = '';

  if (state.destinos.length === 0) {
    elListaDestinos.appendChild(elDestinosEmpty);
    elDestinosEmpty.hidden = false;
    return;
  }

  elDestinosEmpty.hidden = true;

  for (const destino of state.destinos) {
    const li = document.createElement('li');
    li.className = 'badge';
    li.dataset.valor = destino.valor;
    li.dataset.tipo = destino.tipo;

    const icone = destino.tipo === 'matricula' ? svgPessoa : svgPasta;
    const texto = document.createElement('span');
    texto.className = 'badge-text';
    texto.textContent = destino.label || destino.valor;
    texto.title = destino.valor;

    const btnRemover = document.createElement('button');
    btnRemover.className = 'remove-btn';
    btnRemover.type = 'button';
    btnRemover.setAttribute('aria-label', `Remover ${destino.label || destino.valor}`);
    btnRemover.innerHTML = svgX;
    btnRemover.addEventListener('click', () => {
      state.destinos = state.destinos.filter(d => d !== destino);
      renderDestinos();
      atualizarEstadoEnviar();
    });

    li.insertAdjacentHTML('beforeend', icone);
    li.appendChild(texto);
    li.appendChild(btnRemover);
    elListaDestinos.appendChild(li);
  }
}

// ============================================================================
// T-41: Adicionar pasta extra
// ============================================================================

elBtnAddPasta.addEventListener('click', async () => {
  try {
    const path = await invokeCommand('selecionar_pasta_raiz');
    if (!path) return;
    const label = path;
    if (state.destinos.some(d => d.tipo === 'pasta' && d.valor === path)) {
      showToast('Pasta j\u00e1 adicionada', 'warning');
      return;
    }
    state.destinos.push({ tipo: 'pasta', valor: path, label });
    renderDestinos();
    atualizarEstadoEnviar();
  } catch (e) {
    showToast(e.message || 'Erro ao selecionar pasta', 'error');
  }
});

// ============================================================================
// T-42: Sele\u00e7\u00e3o de documento
// ============================================================================

elBtnSelecionarArquivo.addEventListener('click', async () => {
  try {
    const result = await invokeCommand('selecionar_arquivo');
    if (!result) return;
    state.arquivo = { nome: result.split(/[\\/]/).pop(), path: result };
    elArquivoSelecionado.textContent = state.arquivo.nome;
    elBtnLimparArquivo.hidden = false;
    atualizarEstadoEnviar();
  } catch (e) {
    showToast(e.message || 'Erro ao selecionar arquivo', 'error');
  }
});

elBtnLimparArquivo.addEventListener('click', () => {
  state.arquivo = null;
  elArquivoSelecionado.textContent = '';
  elBtnLimparArquivo.hidden = true;
  atualizarEstadoEnviar();
});

// ============================================================================
// T-43: Bot\u00e3o Enviar + T-44: Modal Novo Militar
// ============================================================================

function atualizarEstadoEnviar() {
  const podeEnviar = state.destinos.length > 0 && state.arquivo != null;
  elBtnEnviar.disabled = !podeEnviar;
}

let filaModal = [];
let modalResolve = null;
let modalReject = null;

function abrirModalNovo(matricula) {
  return new Promise((resolve, reject) => {
    modalResolve = resolve;
    modalReject = reject;
    modalNovoTitulo.textContent = `Pasta n\u00e3o encontrada para ${matricula}`;
    modalMatricula.textContent = matricula;
    modalNome.value = '';
    modalNome.focus();
    modalNovo.hidden = false;
  });
}

function fecharModalNovo() {
  modalNovo.hidden = true;
  modalResolve = null;
  modalReject = null;
}

modalBtnCriar.addEventListener('click', () => {
  const nome = modalNome.value.trim().toUpperCase();
  if (!nome) {
    showToast('Informe o nome completo do militar', 'warning');
    modalNome.focus();
    return;
  }
  if (modalResolve) modalResolve({ acao: 'criar', nome });
  fecharModalNovo();
});

modalBtnPular.addEventListener('click', () => {
  if (modalResolve) modalResolve({ acao: 'pular' });
  fecharModalNovo();
});

modalBtnCancelar.addEventListener('click', () => {
  if (modalReject) modalReject(new Error('Opera\u00e7\u00e3o cancelada pelo usu\u00e1rio'));
  fecharModalNovo();
});

// Fechar com Escape
modalNovo.addEventListener('keydown', (e) => {
  if (e.key === 'Escape') {
    if (modalReject) modalReject(new Error('Opera\u00e7\u00e3o cancelada pelo usu\u00e1rio'));
    fecharModalNovo();
  }
});

elBtnEnviar.addEventListener('click', async () => {
  if (state.destinos.length === 0 || !state.arquivo) return;

  // Verifica pasta raiz
  if (!state.pastaRaiz) {
    showToast('Configure a pasta raiz em Configura\u00e7\u00f5es', 'error');
    return;
  }

  // Verifica matr\u00edculas sem pasta
  const matriculasSemPasta = [];
  for (const destino of state.destinos) {
    if (destino.tipo !== 'matricula') continue;
    try {
      const verif = await invokeCommand('verificar_pasta_militar', {
        raiz: state.pastaRaiz,
        matricula: destino.valor,
      });
      if (!verif.existe) {
        matriculasSemPasta.push(destino);
      }
    } catch (e) {
      showToast(`Erro ao verificar matr\u00edcula ${destino.label}: ${e.message}`, 'error');
      return;
    }
  }

  // Para cada sem pasta, abre modal
  const nomesCriar = new Map();
  for (const destino of matriculasSemPasta) {
    try {
      const res = await abrirModalNovo(destino.label);
      if (res.acao === 'criar') {
        nomesCriar.set(destino.valor, res.nome);
      }
      // se 'pular', n\u00e3o adiciona ao mapa
    } catch (e) {
      showToast('Envio cancelado', 'info');
      return;
    }
  }

  // Cria pastas
  for (const [matricula, nome] of nomesCriar) {
    try {
      await invokeCommand('criar_pasta_militar', {
        raiz: state.pastaRaiz,
        matricula,
        nome_completo: nome,
      });
      showToast(`Pasta criada para ${matricula}`, 'success');
    } catch (e) {
      showToast(`Erro ao criar pasta para ${matricula}: ${e.message}`, 'error');
      return;
    }
  }

  // Prepara destinos: se pular, remove da lista tempor\u00e1ria
  const destinosFinais = state.destinos
    .filter(d => {
      if (d.tipo !== 'matricula') return true;
      if (matriculasSemPasta.some(m => m.valor === d.valor) && !nomesCriar.has(d.valor)) {
        return false; // pular
      }
      return true;
    })
    .map(d => d.valor);

  if (destinosFinais.length === 0) {
    showToast('Nenhum destino v\u00e1lido para envio', 'warning');
    return;
  }

  // Envia
  elLog.innerHTML = '';
  setButtonLoading(elBtnEnviar, true);

  try {
    const relatorio = await invokeCommand('enviar_documento', {
      arquivo: state.arquivo.path,
      destinos: destinosFinais,
      raiz: state.pastaRaiz,
      politica: state.politica,
    });

    adicionarLog(`Resumo: ${relatorio.sucesso} de ${relatorio.total} enviados com sucesso`, 'summary');
    if (relatorio.falha > 0) {
      showToast(`${relatorio.falha} envio(s) falharam. Verifique o log.`, 'warning');
    } else {
      showToast('Envio conclu\u00eddo com sucesso!', 'success');
    }
  } catch (e) {
    adicionarLog(`Falha geral no envio: ${e.message}`, 'error');
    showToast(e.message || 'Erro no envio', 'error');
  } finally {
    setButtonLoading(elBtnEnviar, false);
  }
});

// ============================================================================
// T-45: \u00c1rea de log
// ============================================================================

function adicionarLog(mensagem, tipo = 'info') {
  const li = document.createElement('li');
  li.className = tipo;

  const icone = tipo === 'success' ? svgCheck : svgErro;
  li.innerHTML = `${icone}<span>${escapeHtml(mensagem)}</span>`;

  elLog.appendChild(li);
  elLog.scrollTop = elLog.scrollHeight;
}

function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

// Escuta evento progresso-envio do Tauri
if (window.__TAURI__?.event?.listen) {
  window.__TAURI__.event.listen('progresso-envio', (event) => {
    const d = event.payload || event;
    const tipo = d.sucesso ? 'success' : 'error';
    const msg = d.sucesso
      ? `${d.destino}: OK`
      : `${d.destino}: ${d.erro || 'Erro'}`;
    adicionarLog(msg, tipo);
  });
}

elBtnLimparLog.addEventListener('click', () => {
  elLog.innerHTML = '';
});

// Inicializa estado
atualizarEstadoEnviar();
