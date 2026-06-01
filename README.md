# Gestor de Pastas de Militares

Aplicativo desktop para envio e busca de documentos em pasta compartilhada de rede, organizada por matrículas de militares.

---

## O que o app faz

**Enviar documento:** selecione um arquivo e uma ou mais matrículas — o app localiza automaticamente a pasta de cada militar na rede e copia o documento para lá, sequencialmente, com log de sucesso e falha para cada destino. É possível adicionar pastas extras além das pastas dos militares.

**Buscar matrícula:** digite uma matrícula e o app localiza a pasta do militar na rede e copia ela inteira para `~/Downloads/Busca-Matriculas`, sem precisar navegar manualmente pela estrutura de rede.

---

## Stack

| Camada | Tecnologia |
|---|---|
| Lógica e filesystem | Rust |
| Framework desktop | Tauri |
| Interface | HTML + CSS + JavaScript |

**Por que Rust + Tauri:**
- Binário único sem dependências — distribui copiando um arquivo
- Compila nativamente para Windows e Linux
- Erros de filesystem nunca passam silenciosos (o compilador obriga o tratamento)
- Sem instalar Python, Node, .NET ou qualquer runtime nas máquinas dos usuários

---

## Pré-requisitos de desenvolvimento

### Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version  # deve retornar 1.70+
```

### Node.js (necessário para o build do Tauri)
```bash
# Ubuntu/Debian
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# Windows: baixar instalador em https://nodejs.org
```

### Tauri CLI
```bash
cargo install tauri-cli
```

### Dependências do sistema (Linux apenas)
```bash
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libsmbclient-dev  # Para suportar paths SMB/smb://
```

### Montagem da pasta de rede (Linux)

O app aceita paths SMB diretamente. Opcionalmente, pode montar manualmente:
```bash
# Opção 1: smbclient (navegação sem montagem)
sudo apt install smbclient

# Opção 2: montagem permanente via fstab
sudo mount -t cifs //ss4.local/compartilhado/SS4_DADOS /mnt/ss4 \
  -o username=<user>,password=<senha>,workgroup=SS4
```

### Cross-compile para Windows (a partir do Linux)
```bash
rustup target add x86_64-pc-windows-gnu
sudo apt install gcc-mingw-w64
```

---

## Como rodar em desenvolvimento

```bash
# clonar o repositório
git clone <url-do-repo>
cd gestor-pastas-militares

# rodar em modo dev (abre a janela com hot-reload)
cargo tauri dev
```

---

## Como compilar para produção

### Linux
```bash
cargo tauri build
# gera: src-tauri/target/release/bundle/
#   - appimage/gestor-pastas-militares.AppImage
#   - deb/gestor-pastas-militares_*.deb
```

### Windows (a partir do Linux)
```bash
cargo tauri build --target x86_64-pc-windows-gnu
# gera: src-tauri/target/x86_64-pc-windows-gnu/release/gestor-pastas-militares.exe
```

### Windows (a partir do Windows)
```bash
cargo tauri build
# gera: src-tauri/target/release/bundle/
#   - msi/gestor-pastas-militares_*.msi
#   - nsis/gestor-pastas-militares_*.exe
```

---

## Estrutura do Projeto

```
gestor-pastas-militares/
├── src/                          # lógica Rust
│   ├── main.rs                   # entry point, registro de comandos Tauri
│   ├── errors.rs                 # enum AppError centralizado
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── enviar.rs             # comandos da função Enviar Documento
│   │   ├── buscar.rs             # comandos da função Buscar Matrícula
│   │   └── config.rs             # comandos de configuração
│   └── core/
│       ├── mod.rs
│       ├── matricula.rs          # parsing e normalização de matrícula
│       ├── filesystem.rs         # operações de pasta e arquivo
│       └── config.rs             # leitura/escrita da config local
├── frontend/                     # interface HTML/CSS/JS
│   ├── index.html
│   ├── style.css
│   ├── app.js                    # utilitários compartilhados
│   └── components/
│       ├── aba-enviar.js
│       ├── aba-buscar.js
│       ├── configuracoes.js
│       └── log.js
├── src-tauri/                    # configuração do Tauri (gerado)
│   ├── tauri.conf.json
│   ├── Cargo.toml
│   └── icons/
├── Cargo.toml
├── PRD.md
├── TASKS.md
└── README.md
```

---

## Regras de Negócio Principais

### Matrícula

- Aceita entrada com ou sem hífen: `1111111` e `111111-1` são equivalentes
- O hífen sempre fica entre o penúltimo e o último dígito
- Tamanho mínimo: 3 dígitos

### Subpasta na Z:

A pasta raiz da rede é organizada em subpastas cujo nome tem `tamanho_da_matrícula - 2` dígitos:

| Matrícula | Dígitos | Subpasta |
|---|---|---|
| `111` | 3 | `1/` |
| `11111` | 5 | `111/` |
| `111111` | 6 | `1111/` |
| `9207901` | 7 | `92079/` |

### Nome da pasta do militar

```
{MATRICULA-COM-HIFEN} - {NOME COMPLETO EM MAIÚSCULAS}

Exemplos:
  111111-1 - FULANO DE TAL
  92079-1 - CICLANO DE TAL
```

### Busca de pasta

A busca é feita por prefixo, case-insensitive. A pasta `111111-1 - FULANO DE TAL` é encontrada buscando por `1111111` ou `111111-1`.

### Criação de pasta

Se a pasta do militar não existir, o app solicita o nome completo e cria:
```
Z:/{subpasta}/{matricula-com-hifen} - {NOME EM MAIÚSCULAS}/
```

---

## Configuração

Na primeira execução, acesse a aba **Configurações** e defina:

- **Pasta raiz (Z:):** path da pasta compartilhada de rede
  - Linux: `smb://ss4.local/compartilhado/SS4_DADOS/00 - LEV PM`
  - Windows: `Z:\` ou `\\ss4.local\compartilhado\SS4_DADOS\00 - LEV PM`
- **Política de sobrescrita:** o que fazer se o arquivo já existir no destino

A configuração é salva localmente em:
- Linux: `~/.config/gestor-militar/config.json`
- Windows: `%APPDATA%\gestor-militar\config.json`

---

## Comportamentos importantes

- O app **nunca move ou apaga** arquivos da pasta compartilhada — apenas lê e escreve
- O arquivo a ser enviado é **lido uma vez em memória** e copiado sequencialmente para todos os destinos (sem reler do disco a cada cópia)
- Uma falha em um destino **não interrompe** os demais — o log registra cada resultado individualmente
- O app funciona **completamente offline** — nenhuma chamada de rede além do acesso à pasta compartilhada

---

## Pendências abertas

Itens que aguardam informação do usuário antes de serem implementados:

| # | Item |
|---|---|
| P-01 | Path da pasta Z: no Windows |
| P-02 | Path equivalente da pasta Z: no Linux |
| P-03 | Tema visual padrão (claro ou escuro) |
| P-04 | Tamanho máximo de uma matrícula |
| P-05 | Comportamento ao sobrescrever arquivo já existente no destino |

---

## Fora de escopo (v1.0)

- Edição ou visualização de documentos
- Controle de acesso ou autenticação
- Histórico de envios em banco de dados
- Sincronização automática ou watcher de pasta
- Envio em paralelo

---

## Licença

*(a definir)*
