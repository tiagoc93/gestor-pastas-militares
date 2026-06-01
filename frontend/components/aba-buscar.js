import {
  invokeCommand,
  state,
  showToast,
  setButtonLoading,
} from '../app.js';

// ============================================================================
// Elementos
// ============================================================================

const elMatricula = document.getElementById('matricula-busca');
const elBtnBuscar = document.getElementById('btn-buscar');
const elErroMatricula = document.getElementById('erro-matricula-busca');
const elResultado = document.getElementById('resultado-busca');
const elAvisoPastaRaiz = document.getElementById('aviso-pasta-raiz');

const modalSobrescrita = document.getElementById('modal-sobrescrita');
const modalSobrescritaPath = document.getElementById('modal-sobrescrita-path');
const modalBtnSobrescrever = document.getElementById('modal-btn-sobrescrever');
const modalBtnCancelarSobrescrita = document.getElementById('modal-btn-cancelar-sobrescrita');

// ============================================================================
// T-46: Campo matr\u00edcula
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

// ============================================================================
// T-47 / T-48: Buscar e Copiar + resultado
// ============================================================================

async function realizarBusca(sobrescrever = false) {
  elErroMatricula.textContent = '';
  elResultado.classList.add('hidden');
  elResultado.innerHTML = '';
  elAvisoPastaRaiz.classList.add('hidden');

  const { ok, erro, valor } = validarMatricula(elMatricula.value);
  if (!ok) {
    elErroMatricula.textContent = erro;
    elMatricula.focus();
    return;
  }

  if (!state.pastaRaiz) {
    elAvisoPastaRaiz.classList.remove('hidden');
    return;
  }

  setButtonLoading(elBtnBuscar, true);

  try {
    const comando = sobrescrever ? 'confirmar_sobrescrita_busca' : 'buscar_matricula';
    const result = await invokeCommand(comando, {
      raiz: state.pastaRaiz,
      matricula: valor,
    });

    if (result.status === 'ja_existe') {
      modalSobrescritaPath.textContent = result.pathDestino || 'Downloads/Busca-Matriculas';
      modalSobrescrita.hidden = false;
      setButtonLoading(elBtnBuscar, false);
      return;
    }

    exibirSucesso(result);
  } catch (e) {
    exibirErro(e.message || 'Erro na busca');
  } finally {
    setButtonLoading(elBtnBuscar, false);
  }
}

function exibirSucesso(result) {
  elResultado.classList.remove('hidden', 'error');
  elResultado.classList.add('success');

  const path = result.pathDestino || '';
  const arquivos = result.arquivosCopiados ?? 0;
  const matches = result.matchesEncontrados ?? 0;

  elResultado.innerHTML = `
    <p><strong>Busca conclu\u00edda com sucesso!</strong></p>
    <p>Destino: ${escapeHtml(path)}</p>
    <p>Arquivos copiados: ${arquivos}</p>
    <p>Matches encontrados: ${matches}</p>
    <button class="btn btn-secondary btn-sm" id="btn-abrir-pasta-busca" type="button">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" style="width:16px;height:16px"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>
      Abrir pasta
    </button>
  `;

  document.getElementById('btn-abrir-pasta-busca')?.addEventListener('click', () => {
    if (path) {
      invokeCommand('plugin:shell|open', { path }).catch(() => {
        showToast('N\u00e3o foi poss\u00edvel abrir a pasta', 'error');
      });
    }
  });
}

function exibirErro(mensagem) {
  elResultado.classList.remove('hidden', 'success');
  elResultado.classList.add('error');
  elResultado.innerHTML = `<p><strong>Erro na busca</strong></p><p>${escapeHtml(mensagem)}</p>`;
}

function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

elBtnBuscar.addEventListener('click', () => realizarBusca(false));
elMatricula.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') {
    e.preventDefault();
    realizarBusca(false);
  }
});

// ============================================================================
// T-49: Modal confirma\u00e7\u00e3o sobrescrita
// ============================================================================

modalBtnSobrescrever.addEventListener('click', () => {
  modalSobrescrita.hidden = true;
  realizarBusca(true);
});

modalBtnCancelarSobrescrita.addEventListener('click', () => {
  modalSobrescrita.hidden = true;
});

modalSobrescrita.addEventListener('keydown', (e) => {
  if (e.key === 'Escape') {
    modalSobrescrita.hidden = true;
  }
});
