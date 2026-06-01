# Gestor de Pastas de Militares

Aplicativo desktop para envio e busca de documentos em pasta compartilhada de rede, organizada por matrículas de militares.

---

## Como rodar em desenvolvimento

**Pré-requisitos:** Rust, Node.js 20+, Tauri CLI

```bash
git clone https://github.com/tiagoc93/gestor-pastas-militares.git
cd gestor-pastas-militares

# Instalar Tauri CLI (uma vez)
cargo install tauri-cli

# Rodar em modo dev
cargo tauri dev
```

O app abre em modo janela com hot-reload do frontend.

---

## Como compilar para produção

### Linux
```bash
cargo tauri build
# gera: src-tauri/target/release/bundle/appimage/gestor-pastas-militares.AppImage
#       src-tauri/target/release/bundle/deb/gestor-pastas-militares_*.deb
```

### Windows (a partir do Linux — cross-compile)
```bash
rustup target add x86_64-pc-windows-gnu
cargo tauri build --target x86_64-pc-windows-gnu
# gera: src-tauri/target/x86_64-pc-windows-gnu/release/gestor-pastas-militares.exe
```

### Windows (a partir do Windows — cmd/powershell)
```bash
cargo tauri build
# gera: src-tauri/target/release/bundle/nsis/gestor-pastas-militares_*.exe
```

---

## Estrutura do Projeto

```
gestor-pastas-militares/
├── Cargo.toml                    # workspace root (lib + bin)
├── Cargo.lock
├── src/
│   ├── main.rs                   # entry point — registro de comandos Tauri
│   ├── lib.rs                    # exporting dos módulos (pub mod)
│   ├── errors.rs                 # enum AppError centralizado + testes
│   ├── commands/
│   │   ├── mod.rs                # pub use ... (re-exports)
│   │   ├── enviar.rs             # comandos Tauri: enviar_documento, adicionar_matricula, criar_pasta_militar, verificar_pasta_militar
│   │   ├── buscar.rs             # comandos Tauri: buscar_matricula, confirmar_sobrescrita_busca
│   │   └── config.rs             # comandos Tauri: carregar_config, salvar_config, selecionar_pasta_raiz, selecionar_arquivo
│   └── core/
│       ├── mod.rs                # pub use ... (re-exports)
│       ├── matricula.rs          # parsing e normalização de matrícula + testes
│       ├── filesystem.rs         # operações de pasta/arquivo + testes
│       └── config.rs            # leitura/escrita de config local (AppConfig) + testes
├── frontend/
│   ├── index.html                # estrutura HTML com tabs + modais
│   ├── style.css                 # design system OLED Dark
│   ├── app.js                    # utilitários (toast, log)
│   └── components/
│       ├── aba-enviar.js         # tab Enviar Documento
│       ├── aba-buscar.js         # tab Buscar Matrícula
│       └── configuracoes.js      # tab Configurações
├── src-tauri/
│   ├── Cargo.toml                # pacote Tauri (lib + bin pointing to ../src/)
│   ├── build.rs                  # tauri_build::build()
│   ├── tauri.conf.json           # título, tamanho, CSP, bundling
│   └── icons/                    # ícones do app (icon.png, icon.ico, etc)
├── capabilities/
│   └── main.json                 # permissões filesystem e dialog
├── tests/
│   └── integration_tests.rs      # testes de integração
├── gen/schemas/                  # gerado pelo Tauri (não editar)
├── icons/                        # ícones do repositório (backup)
├── PRD.md                        # Product Requirements Document
├── TASKS.md                      # checklist de desenvolvimento
└── README.md
```

---

## Stack

- **Lógica e filesystem:** Rust
- **Framework desktop:** Tauri v2
- **Interface:** HTML + CSS + JavaScript (vanilla)
- **Build output:** binário único (~3-5 MB), sem runtime

**Por que Rust + Tauri:**
- Binário único sem dependências — distribui copiando um arquivo
- Compila nativamente para Windows e Linux
- Erros de filesystem nunca passam silenciosos
- Nenhuma dependência de runtime nas máquinas dos usuários

---

## Regras de Negócio

### Matrícula
- Aceita com ou sem hífen: `1111111` e `111111-1` são equivalentes
- Hífen sempre entre penúltimo e último dígito
- Tamanho: 3 a 8 dígitos

### Subpasta na rede
A raiz é organizada em subpastas de `tamanho - 2` dígitos:

| Matrícula | Dígitos | Subpasta |
|---|---|---|
| `111` | 3 | `1/` |
| `11111` | 5 | `111/` |
| `111111` | 6 | `1111/` |
| `9207901` | 7 | `92079/` |

### Nome da pasta do militar
```
{MATRÍCULA-COM-HIFEN} - {NOME EM MAIÚSCULAS}
Ex: 111111-1 - FULANO DE TAL
```

### Busca flat
Itera **todas** as subpastas da raiz procurando entries cujo nome comece com a matrícula, case-insensitive. Não deriva subpasta — encontra militares mesmo que estejam em pasta errada.

---

## Configuração

O app salva config em:
- **Linux:** `~/.config/gestor-militar/config.json`
- **Windows:** `%APPDATA%\gestor-militar\config.json`

Parâmetros:
- `pasta_raiz`: path da pasta compartilhada de rede
- `politica_sobrescrita`: `sobrescrever`, `pular` ou `renomear`
- `tema`: `escuro` (padrão) ou `claro`

Caminhos típicos:
- **Linux:** `smb://ss4.local/compartilhado/SS4_DADOS/00 - LEV PM`
- **Windows:** `Z:\SS4_DADOS\00 - LEV PM`

---

## Pendências resolvidas

| # | Item | Decisão |
|---|---|---|
| P-01 | Path da pasta Z: no Windows | `Z:\SS4_DADOS\00 - LEV PM` |
| P-02 | Path equivalente da pasta Z: no Linux | `smb://ss4.local/compartilhado/SS4_DADOS/00 - LEV PM` |
| P-03 | Tema visual padrão | Escuro |
| P-04 | Tamanho máximo de uma matrícula | 8 dígitos |
| P-05 | Comportamento ao sobrescrever arquivo | Criar cópia com sufixo timestamp |

---

## Próximas implementações (Fase 2+)

### F-01: Envio de múltiplos documentos
Na tela "Enviar Documento", permitir selecionar mais de um arquivo para enviar em lote. O comportamento atual só permite 1 arquivo por envio. Precisa:
- Permitir seleção múltipla de arquivos no seletor nativo
- Exibir lista de arquivos selecionados antes do envio
- Copiar cada arquivo para todos os destinos selecionados
- Log individual por arquivo no log de envio

### F-02: Investigar falhas na busca de matrícula
A busca flat (`encontrar_todos_militares`) aparentemente encontra algumas matrículas mas não outras. Possíveis causas a investigar:
- Caracteres especiais ou espaços no nome da pasta do militar
- Case sensitivity no matching (deve ser case-insensitive, mas pode haver edge cases)
- Nomes que começam igual mas têm prefixos diferentes (ex: `111` matchando `1111` vs `11111`)
- Subpastas com nomes não-numéricos sendo ignoradas
- Encoding de caracteres especiais no filesystem Linux vs Windows
- Pastas de militares dentro de outras pastas (deep nesting)

---

## Fora de escopo (v1.0)

- Edição ou visualização de documentos
- Controle de acesso ou autenticação
- Histórico de envios em banco de dados
- Sincronização automática ou watcher de pasta
- Envio em paralelo
- Tema claro