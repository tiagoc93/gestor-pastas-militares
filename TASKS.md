# TASKS — Gestor de Pastas de Militares

> Legenda de status: `[ ]` pendente · `[~]` em andamento · `[x]` concluído · `[!]` bloqueado

---

## FASE 0 — Pré-desenvolvimento (Pendências externas)

- [x] **P-01** ~~Confirmar path da pasta Z: no Windows (`Z:\` ou UNC `\\servidor\share`)~~ → Fornecido: `Z:\` ou `\\ss4.local\compartilhado\SS4_DADOS\00 - LEV PM`
- [x] **P-02** ~~Confirmar path equivalente da pasta Z: no Linux~~ → Fornecido: `smb://ss4.local/compartilhado/SS4_DADOS/00 - LEV PM`
- [ ] **P-03** Definir tema visual padrão (claro ou escuro)
- [ ] **P-04** Confirmar tamanho máximo possível de uma matrícula (até 7 dígitos ou mais?)
- [ ] **P-05** Definir comportamento ao sobrescrever arquivo já existente no destino (sobrescreve silencioso / pergunta / cria cópia com sufixo)

---

## FASE 1 — Setup do Projeto

### 1.1 Ambiente de desenvolvimento
- [ ] **T-01** Instalar Rust (`rustup`) e confirmar versão estável (`rustc --version`)
- [ ] **T-02** Instalar Node.js (necessário para build do Tauri)
- [ ] **T-03** Instalar Tauri CLI (`cargo install tauri-cli`)
- [ ] **T-04** Instalar dependências do sistema para Tauri no Linux (webkit2gtk, etc.)
- [ ] **T-05** Instalar target de cross-compile para Windows no Linux:
  ```bash
  rustup target add x86_64-pc-windows-gnu
  sudo apt install gcc-mingw-w64
  ```

### 1.2 Scaffolding do projeto
- [ ] **T-06** Criar projeto Tauri:
  ```bash
  cargo tauri init
  ```
- [ ] **T-07** Configurar `tauri.conf.json`:
  - Título da janela: `Gestor de Pastas de Militares`
  - Tamanho inicial da janela: 900×650
  - Tamanho mínimo: 700×500
  - Resizable: true
  - Permissões necessárias: `dialog`, `fs`, `path`
- [ ] **T-08** Configurar `Cargo.toml` com dependências iniciais:
  - `serde` + `serde_json` (serialização de config)
  - `tauri` com features: `dialog`, `fs`, `path`, `shell`
  - `dirs` (para resolver `~/Downloads` cross-platform)
  - `thiserror` (erros customizados)
- [ ] **T-09** Criar estrutura de pastas do projeto:
  ```
  src/                  ← lógica Rust
    main.rs
    commands/
      mod.rs
      enviar.rs
      buscar.rs
      config.rs
    core/
      mod.rs
      matricula.rs      ← parsing e normalização
      filesystem.rs     ← operações de pasta/arquivo
      config.rs         ← leitura/escrita de configuração
    errors.rs
  src-tauri/            ← gerado pelo Tauri
  frontend/
    index.html
    style.css
    app.js
    components/
      aba-enviar.js
      aba-buscar.js
      configuracoes.js
      log.js
  ```
- [ ] **T-10** Configurar `.gitignore` adequado para Rust + Node + Tauri
- [ ] **T-11** Inicializar repositório Git com commit inicial

---

## FASE 2 — Lógica Core (Rust puro, sem UI)

### 2.1 Módulo: Matrícula (`core/matricula.rs`)

- [ ] **T-12** Implementar struct `Matricula`:
  ```rust
  pub struct Matricula {
      pub digitos: String,     // somente números, ex: "1111111"
      pub com_hifen: String,   // com hífen, ex: "111111-1"
      pub subpasta: String,    // prefixo da subpasta, ex: "11111"
  }
  ```
- [ ] **T-13** Implementar função `Matricula::parse(input: &str) -> Result<Matricula, AppError>`:
  - Remove hífen se presente
  - Valida que contém apenas dígitos
  - Valida tamanho mínimo de 3 dígitos
  - Calcula `com_hifen`: insere `-` entre penúltimo e último dígito
  - Calcula `subpasta`: primeiros `len - 2` dígitos
- [ ] **T-14** Escrever testes unitários para `Matricula::parse`:
  - Entrada com hífen: `"111111-1"` → `digitos: "1111111"`, `subpasta: "11111"`
  - Entrada sem hífen: `"1111111"` → mesmo resultado
  - Entrada curta `"111"` → `subpasta: "1"`, `com_hifen: "11-1"`
  - Entrada inválida (letras, vazia) → `Err`
  - Entrada com hífen em posição errada → `Err` ou normaliza corretamente

### 2.2 Módulo: Filesystem (`core/filesystem.rs`)

- [ ] **T-15** Implementar função `buscar_pasta_militar_flat(raiz: &Path, matricula: &Matricula) -> Result<Option<PathBuf>, AppError>`:
  - **Busca flat na raiz** — itera todas as subpastas de raiz recursivamente
  - Procura entradas cujo nome começa com `matricula.digitos` OU `matricula.com_hifen` (case-insensitive)
  - Retorna a primeira pasta encontrada
  - **NÃO** deriva a subpasta do tamanho da matrícula para busca
- [ ] **T-16** Implementar função `criar_pasta_militar(raiz: &Path, matricula: &Matricula, nome_completo: &str) -> Result<PathBuf, AppError>`:
  - **Calcula subpasta correta** (`tamanho - 2`)
  - Se subpasta **não existe** → cria a subpasta
  - Se subpasta **existe** → usa a subpasta existente
  - Nome da nova pasta: `{com_hifen} - {NOME_EM_MAIÚSCULAS}`
  - Cria a pasta do militar dentro da subpasta correta
  - Retorna o path criado
- [ ] **T-17** Implementar função `copiar_arquivo(arquivo: &Path, destino_dir: &Path, politica_sobrescrita: PoliticaSobrescrita) -> Result<(), AppError>`:
  - Lê o arquivo fonte em memória uma única vez (recebe `&[u8]` já lido ou lê internamente)
  - Monta path de destino: `destino_dir / nome_do_arquivo`
  - Aplica política de sobrescrita (enum: `Sobrescrever`, `Pular`, `RenomearComSufixo`)
  - Escreve o arquivo
- [ ] **T-18** Implementar função `copiar_pasta_recursiva(origem: &Path, destino: &Path) -> Result<CopyStats, AppError>`:
  - Copia recursivamente todos os arquivos e subpastas
  - Retorna estatísticas: quantidade de arquivos copiados, tamanho total
- [ ] **T-19** Escrever testes de integração para filesystem usando pasta temporária (`tempdir`):
  - Criar estrutura de pastas simulando a Z:
  - Testar busca encontrando pasta existente
  - Testar busca não encontrando pasta
  - Testar criação de pasta nova
  - Testar cópia de arquivo

### 2.3 Módulo: Config (`core/config.rs`)

- [ ] **T-20** Implementar struct `AppConfig`:
  ```rust
  pub struct AppConfig {
      pub pasta_raiz: Option<String>,          // path da Z:
      pub politica_sobrescrita: PoliticaSobrescrita,
      pub tema: Tema,                           // Claro | Escuro
  }
  ```
- [ ] **T-21** Implementar `AppConfig::carregar() -> AppConfig`:
  - Lê de `~/.config/gestor-militar/config.json` (Linux) ou `%APPDATA%\gestor-militar\config.json` (Windows)
  - Retorna config padrão se arquivo não existir
- [ ] **T-22** Implementar `AppConfig::salvar(&self) -> Result<(), AppError>`:
  - Cria diretório se não existir
  - Serializa para JSON e salva

### 2.4 Módulo: Erros (`errors.rs`)

- [ ] **T-23** Definir enum `AppError` com variantes:
  - `MatriculaInvalida(String)`
  - `SubpastaInexistente(PathBuf)`
  - `MilitarNaoEncontrado(String)`
  - `ErroDeIO { path: PathBuf, fonte: std::io::Error }`
  - `PastaRaizNaoConfigurada`
  - `PastaDestinoInvalida(PathBuf)`
- [ ] **T-24** Implementar `Display` para `AppError` com mensagens em português claras para o usuário

---

## FASE 3 — Comandos Tauri (bridge Rust ↔ Frontend)

### 3.1 Comando: Enviar Documento (`commands/enviar.rs`)

- [ ] **T-25** Implementar comando `#[tauri::command] adicionar_matricula(matricula: String) -> Result<MatriculaInfo, String>`:
  - Chama `Matricula::parse`
  - Retorna struct serializável com `digitos`, `com_hifen`, `subpasta`
  - Retorna erro amigável em caso de matrícula inválida

- [ ] **T-26** Implementar comando `#[tauri::command] verificar_pasta_militar(raiz: String, matricula: String) -> Result<VerificacaoResult, String>`:
  - Retorna `{ existe: bool, path: Option<String> }`
  - Usado para feedback antes do envio

- [ ] **T-27** Implementar comando `#[tauri::command] criar_pasta_militar(raiz: String, matricula: String, nome_completo: String) -> Result<String, String>`:
  - Cria a pasta e retorna o path criado

- [ ] **T-28** Implementar comando `#[tauri::command] enviar_documento(payload: EnviarPayload, window: tauri::Window) -> Result<RelatorioEnvio, String>`:
  - Payload: `{ arquivo: String, destinos: Vec<Destino>, raiz: String }`
  - Destino pode ser matrícula ou path direto (pasta extra)
  - Lê o arquivo uma vez em memória
  - Loop sequencial pelos destinos
  - Emite evento Tauri a cada destino processado: `"progresso-envio"` com `{ destino, sucesso, erro }`
  - Retorna relatório final: `{ total, sucesso, falha, detalhes }`

### 3.2 Comando: Buscar Matrícula (`commands/buscar.rs`)

- [ ] **T-29** Implementar comando `#[tauri::command] buscar_matricula(raiz: String, matricula: String) -> Result<BuscaResult, String>`:
  - Localiza pasta do militar
  - Copia recursivamente para `~/Downloads/Busca-Matriculas/{matricula}/`
  - Se destino já existe, retorna status `"ja_existe"` com path para o frontend decidir
  - Retorna `{ status, path_destino, arquivos_copiados }`

- [ ] **T-30** Implementar comando `#[tauri::command] confirmar_sobrescrita_busca(raiz: String, matricula: String) -> Result<BuscaResult, String>`:
  - Mesmo que `buscar_matricula` mas force sobrescrita

### 3.3 Comando: Configurações (`commands/config.rs`)

- [ ] **T-31** Implementar comando `#[tauri::command] carregar_config() -> Result<AppConfigDto, String>`
- [ ] **T-32** Implementar comando `#[tauri::command] salvar_config(config: AppConfigDto) -> Result<(), String>`
- [ ] **T-33** Implementar comando `#[tauri::command] selecionar_pasta_raiz() -> Result<Option<String>, String>`:
  - Abre dialog nativo de seleção de pasta
  - Retorna path selecionado ou `None` se cancelado

### 3.4 Registro dos comandos

- [ ] **T-34** Registrar todos os comandos no `main.rs` via `.invoke_handler(tauri::generate_handler![...])`
- [ ] **T-35** Configurar permissões de `fs` e `dialog` no `tauri.conf.json` conforme necessário

---

## FASE 4 — Frontend (HTML + CSS + JS)

### 4.1 Estrutura base

- [ ] **T-36** Criar `index.html` com estrutura de abas:
  - Aba "Enviar Documento"
  - Aba "Buscar Matrícula"
  - Aba "Configurações"
- [ ] **T-37** Criar `style.css` com design system:
  - Variáveis CSS para cores, tipografia e espaçamentos
  - Tema escuro base (ou claro, conforme **P-03**)
  - Classes utilitárias para botões, inputs, cards, badges de status
  - Estilo para área de log (scrollável, fonte monospace)
  - Animação sutil de loading/progresso
- [ ] **T-38** Criar utilitário `app.js` com funções compartilhadas:
  - Wrapper para `window.__TAURI__.invoke`
  - Função de formatação de erros para exibição
  - Gerenciamento de estado global simples (objeto JS)

### 4.2 Aba: Enviar Documento (`components/aba-enviar.js`)

- [ ] **T-39** Implementar campo de entrada de matrícula:
  - Input de texto com placeholder `"Ex: 1111111 ou 111111-1"`
  - Botão "Adicionar" ou Enter para confirmar
  - Chama comando `adicionar_matricula` ao confirmar
  - Exibe erro inline se matrícula inválida
- [ ] **T-40** Implementar lista de destinos:
  - Renderiza cada matrícula como badge/chip com ícone de pessoa e botão ✕ para remover
  - Renderiza cada pasta extra como badge/chip com ícone de pasta e botão ✕ para remover
  - Lista vazia exibe estado vazio com instrução
- [ ] **T-41** Implementar botão "Adicionar pasta extra":
  - Chama comando `selecionar_pasta_raiz` (dialog de pasta)
  - Adiciona path retornado à lista de destinos
- [ ] **T-42** Implementar seleção de documento:
  - Botão "Selecionar documento" abre file picker (`dialog::open`)
  - Exibe nome e path do arquivo selecionado abaixo do botão
  - Botão de limpar seleção (✕)
- [ ] **T-43** Implementar botão "Enviar":
  - Desabilitado se lista de destinos vazia ou nenhum documento selecionado
  - Ao clicar, verifica se alguma matrícula não tem pasta existente
  - Para cada matrícula sem pasta: exibe modal pedindo nome completo do militar
  - Após coleta de nomes, inicia envio
- [ ] **T-44** Implementar modal "Novo Militar":
  - Título: `"Pasta não encontrada para {MATRICULA}"`
  - Campo: Nome completo do militar (obrigatório, converte para maiúsculas automaticamente)
  - Botões: Criar pasta / Pular este destino / Cancelar tudo
- [ ] **T-45** Implementar área de log em tempo real:
  - Escuta evento `"progresso-envio"` do Tauri
  - Cada entrada: ícone ✓ (verde) ou ✗ (vermelho), nome do destino, mensagem
  - Auto-scroll para última entrada
  - Linha de resumo ao final: `"X de Y enviados com sucesso"`
  - Botão "Limpar log"

### 4.3 Aba: Buscar Matrícula (`components/aba-buscar.js`)

- [ ] **T-46** Implementar campo de entrada de matrícula:
  - Mesmo padrão do campo da aba de envio
- [ ] **T-47** Implementar botão "Buscar e Copiar":
  - Desabilitado se campo vazio
  - Exibe spinner enquanto processa
- [ ] **T-48** Implementar exibição de resultado:
  - Sucesso: card verde com ícone ✓, path de destino, quantidade de arquivos copiados, botão "Abrir pasta"
  - Erro (não encontrado): card vermelho com ícone ✗ e mensagem clara
  - Pasta raiz não configurada: aviso com link para aba de configurações
- [ ] **T-49** Implementar modal de confirmação de sobrescrita:
  - Aparece quando `~/Downloads/Busca-Matriculas/{matricula}` já existe
  - Opções: Sobrescrever / Cancelar

### 4.4 Aba: Configurações (`components/configuracoes.js`)

- [ ] **T-50** Carregar config atual ao abrir a aba (`carregar_config`)
- [ ] **T-51** Implementar campo de pasta raiz (Z:):
  - Input de texto com path atual
  - Botão "Selecionar" abre dialog de pasta
  - Exibe aviso visual se pasta não estiver configurada
- [ ] **T-52** Implementar seletor de política de sobrescrita:
  - Radio buttons: Sobrescrever sempre / Perguntar / Criar cópia com sufixo
- [ ] **T-53** Implementar seletor de tema (quando **P-03** definido)
- [ ] **T-54** Botão "Salvar configurações" chama `salvar_config`
- [ ] **T-55** Exibir toast de confirmação após salvar

---

## FASE 5 — Integração e Testes

### 5.1 Testes de integração end-to-end

- [ ] **T-56** Criar pasta de teste simulando estrutura da Z: no ambiente de dev
- [ ] **T-57** Testar fluxo completo de envio para matrícula existente
- [ ] **T-58** Testar fluxo completo de envio para matrícula inexistente (criação de pasta)
- [ ] **T-59** Testar envio para múltiplas matrículas (5+) em sequência
- [ ] **T-60** Testar envio para pasta extra (path manual)
- [ ] **T-61** Testar busca de matrícula existente
- [ ] **T-62** Testar busca de matrícula inexistente
- [ ] **T-63** Testar comportamento com pasta raiz não configurada
- [ ] **T-64** Testar comportamento com pasta de rede inacessível (simular desconexão)
- [ ] **T-65** Testar com arquivo de 50MB+ (validar leitura em RAM)

### 5.2 Testes cross-platform

- [ ] **T-66** Compilar e testar no Linux
- [ ] **T-67** Compilar e testar no Windows (VM ou máquina física)
- [ ] **T-68** Testar com path UNC no Windows (`\\servidor\share`)
- [ ] **T-69** Testar com path `/mnt/...` no Linux
- [ ] **T-70** Verificar que file picker abre corretamente em ambos os SOs

---

## FASE 6 — Build e Distribuição

### 6.1 Build de produção

- [ ] **T-71** Configurar `tauri.conf.json` para build de produção:
  - Ícone do app (criar ícone `.png` 512x512 e gerar variantes)
  - Versão inicial: `0.1.0`
  - Identificador único: `br.mil.gestor-pastas` (ou similar)
- [ ] **T-72** Gerar build Linux:
  ```bash
  cargo tauri build
  ```
  - Validar que gera `.AppImage` ou `.deb`
- [ ] **T-73** Gerar build Windows (cross-compile a partir do Linux ou em VM Windows):
  ```bash
  cargo tauri build --target x86_64-pc-windows-msvc
  ```
  - Validar que gera `.exe` instalador ou portátil
- [ ] **T-74** Testar instaladores em máquinas limpas (sem Rust, sem Node)

### 6.2 Documentação de distribuição

- [ ] **T-75** Documentar como instalar no Windows (copiar `.exe` ou rodar instalador)
- [ ] **T-76** Documentar como instalar no Linux (`.AppImage` ou `.deb`)
- [ ] **T-77** Documentar como configurar a pasta Z: na primeira execução

---

## Backlog (baixa prioridade / pós v1.0)

- [ ] **B-01** Exportar log de operações para arquivo `.txt`
- [ ] **B-02** Histórico das últimas matrículas utilizadas (auto-complete)
- [ ] **B-03** Arrastar e soltar arquivo para seleção de documento (drag & drop)
- [ ] **B-04** Suporte a múltiplos arquivos enviados de uma vez para os mesmos destinos
- [ ] **B-05** Barra de progresso com percentual durante envio de arquivos grandes
- [ ] **B-06** Tema claro/escuro automático (segue preferência do SO)
- [ ] **B-07** Atalhos de teclado (Tab entre campos, Enter para confirmar)

---

## Ordem de Execução Sugerida

```
FASE 0 (desbloquear pendências)
  ↓
FASE 1 (setup)
  ↓
FASE 2 (lógica core — pode ser desenvolvida e testada sem UI)
  ↓
FASE 3 (comandos Tauri)
  ↓
FASE 4 (frontend — pode ser desenvolvido em paralelo com a Fase 3)
  ↓
FASE 5 (integração)
  ↓
FASE 6 (build e distribuição)
```
