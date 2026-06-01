# PRD — Gestor de Pastas de Militares

**Versão:** 0.1 (rascunho)  
**Status:** Em definição  
**Stack:** Rust + Tauri + HTML/CSS/JS

---

## 1. Visão Geral

Aplicativo desktop cross-platform (Windows e Linux) para gerenciar o envio e busca de documentos em uma pasta compartilhada de rede organizada por matrículas de militares.

---

## 2. Stack Técnica

| Camada | Tecnologia |
|---|---|
| Backend / lógica | Rust |
| Framework desktop | Tauri |
| Frontend (UI) | HTML + CSS + JavaScript |
| File picker | API nativa do Tauri (`dialog::open`) |
| Navegação de pastas | API nativa do Tauri (`dialog::open` com `directory: true`) |

**Justificativa do Rust:**
- Binário único sem dependências (sem instalar runtime nas máquinas)
- Compila nativamente para Windows (`.exe`) e Linux
- Tratamento de erros obrigatório em tempo de compilação — elimina falhas silenciosas
- Leitura única do arquivo em RAM para múltiplas cópias sequenciais

---

## 3. Pasta Raiz Compartilhada (Z:)

**Path da rede:**
- **Linux:** `smb://ss4.local/compartilhado/SS4_DADOS/00 - LEV PM`
- **Windows:** `Z:\` ou `\\ss4.local\compartilhado\SS4_DADOS\00 - LEV PM`

> ⚠️ **Path fornecido pelo usuário.** O app deve aceitar o path UNC/SMB diretamente — não converter nem normalizar. A pasta raiz é configurável na interface e **não deve ser hardcoded no binário**.

---

## 4. Estrutura de Pastas na Z:

```
Z:/
├── 11/               ← subpasta para matrículas com 3 dígitos (ex: 111)
│   └── 111-1 - FULANO DE TAL/
├── 111/              ← subpasta para matrículas com 5 dígitos (ex: 11111)
│   ├── 11111-1 - FULANO DE TAL/
│   └── 11122-3 - CICLANO DE TAL/
├── 1111/             ← subpasta para matrículas com 6 dígitos (ex: 111111)
│   ├── 111111-1 - FULANO DE TAL/
│   └── 111222-3 - CICLANO DE TAL/
└── ...
```

### Regra de Busca (Flat Search)

> ⚠️ **Busca flat na raiz** — o app procura a pasta do militar em TODAS as subpastas da raiz, não apenas na subpasta derivada da matrícula. Rationale: militares podem estar em pastas erradas na estrutura original.

Ao buscar por matrícula `1111111`:
- Procura em `*/111111-1 *` e `*/1111111 *` em qualquer subpasta da raiz
- Se encontrar → copia o documento lá dentro
- Se não encontrar → cria na **subpasta correta** (`tamanho - 2`)

### Regra de Subpasta (para criação)

```
tamanho_da_matricula_sem_hifen - 2 = quantidade de dígitos da subpasta
```

**Exemplos:**

| Matrícula | Dígitos | Subpasta |
|---|---|---|
| 111 | 3 | `11` |
| 11111 | 5 | `111` |
| 111111 | 6 | `1111` |
| 9207901 | 7 | `92079` |

### Formato do Nome da Pasta do Militar

```
{MATRICULA-COM-HIFEN} - {NOME COMPLETO EM MAIÚSCULAS}
```

Exemplos:
- `111111-1 - FULANO DE TAL`
- `92079-1 - CICLANO DE TAL`

---

## 5. Regra do Hífen na Matrícula

O hífen sempre é inserido **entre o penúltimo e o último dígito:**

```
1111111  →  111111-1
920791   →  92079-1
111      →  11-1
```

O sistema deve aceitar a matrícula com ou sem hífen na entrada do usuário e normalizar internamente para ambas as formas na hora da busca.

---

## 6. Funções do App

### 6.1 Função: Enviar Documento

**Fluxo completo:**

1. Usuário digita uma matrícula (com ou sem hífen)
2. App exibe a matrícula normalizada e a adiciona à lista de destinos
3. Usuário pode adicionar mais matrículas (uma por uma)
4. Usuário pode adicionar pastas extras manualmente (navegando pela Z: via file picker)
5. Usuário seleciona o documento a enviar (file picker nativo)
6. App processa sequencialmente cada destino:
   - **Busca flat na raiz** — procura a pasta do militar em qualquer subpasta
   - **Se encontrar** → copia o documento lá dentro
   - **Se não encontrar** → solicita o nome completo do militar e cria a pasta
   - Calcula a subpasta correta (`tamanho - 2`)
   - Se a subpasta não existe → cria a subpasta
   - Cria a pasta do militar (`{MATRICULA-COM-HIFEN} - {NOME EM MAIÚSCULAS}`) dentro da subpasta correta
   - Copia o documento
7. App exibe log em tempo real: ✓ sucesso ou ✗ erro com motivo para cada destino

**Lógica de busca e criação:**

| Situação | Ação |
|---|---|
| Pasta do militar **encontrada** (flat search) | Copia documento na pasta existente |
| Pasta **não encontrada** + subpasta **existe** | Cria só a pasta do militar na subpasta |
| Pasta **não encontrada** + subpasta **não existe** | Cria subpasta → cria pasta do militar → copia |

**Cópia do arquivo:**
- O arquivo é lido uma única vez em memória (RAM)
- Escrito sequencialmente em cada destino (sem paralelismo)
- Nunca move o arquivo original — sempre copia

**Tratamento de erros:**
- Cada falha é registrada no log com o destino e o motivo
- Uma falha em um destino não interrompe os demais
- Ao final, exibe resumo: X destinos com sucesso, Y com falha

---

### 6.2 Função: Buscar Matrícula

**Fluxo completo:**

1. Usuário digita a matrícula (com ou sem hífen)
2. App faz **busca flat na raiz** — procura em todas as subpastas
3. Se não encontrar: exibe mensagem de erro
4. Se encontrar: copia a pasta inteira (com subpastas e arquivos) para:
   ```
   ~/Downloads/Busca-Matriculas/{MATRICULA}/
   ```
5. Exibe confirmação com o path de destino

**Comportamento da cópia:**
- Copia recursiva (pasta inteira, incluindo subpastas)
- Não remove da Z: — apenas copia
- Se já existir em `Busca-Matriculas`, pergunta se deseja sobrescrever

---

## 7. Interface

### 7.1 Estilo Geral

- GUI desktop via Tauri (janela nativa)
- Visual limpo, profissional, com tema escuro ou claro (a definir)
- Inspiração: ferramentas CLI modernas (Claude Code, etc.) mas em janela GUI
- Tipografia clara, sem poluição visual

### 7.2 Tela Principal

Duas seções/abas:

| Aba | Descrição |
|---|---|
| **Enviar Documento** | Função 6.1 |
| **Buscar Matrícula** | Função 6.2 |

### 7.3 Componentes — Aba Enviar Documento

- Campo de texto para digitar matrícula
- Botão "Adicionar"
- Lista de destinos acumulados (matrículas + pastas extras)
  - Cada item com botão de remover
- Botão "Adicionar pasta extra" → abre navegador de pastas na Z:
- Campo / botão para selecionar o documento → abre file picker
- Botão "Enviar"
- Área de log (scroll) com resultado de cada destino em tempo real

### 7.4 Componentes — Aba Buscar Matrícula

- Campo de texto para digitar matrícula
- Botão "Buscar e Copiar"
- Área de resultado com path de destino ou mensagem de erro

### 7.5 Configurações

- Campo para definir/alterar o path da pasta Z: (salvo localmente)

---

## 8. Comportamentos Gerais

- A pasta Z: (path raiz) é configurável e persiste entre sessões (salva em config local)
- Matrículas são normalizadas internamente: remove hífen para cálculo de subpasta, mantém com hífen para nome de pasta
- Busca por pasta do militar é case-insensitive
- O app nunca move ou apaga arquivos da Z: — apenas lê e escreve
- Logs de operação podem ser exportados (opcional, baixa prioridade)

---

## 9. Requisitos Não-Funcionais

- **Cross-platform:** Windows 10+ e Linux (Ubuntu 20.04+)
- **Sem instalação de runtime:** binário único via Tauri
- **Sem internet:** funciona completamente offline
- **Confiabilidade:** nenhuma falha pode passar silenciosa — toda operação retorna sucesso ou erro explícito

---

## 10. Fora de Escopo (por ora)

- Edição ou visualização de documentos
- Controle de acesso / autenticação
- Histórico de envios persistido em banco de dados
- Sincronização automática / watcher de pasta
- Envio em paralelo (decisão técnica: sequencial por ora)

---

## 11. Pendências

| # | Item | Responsável |
|---|---|---|
| 1 | Path completo da pasta Z: (Windows e Linux) | Usuário |
| 2 | Definição de tema visual (claro/escuro) | Usuário |
| 3 | Confirmar se matrícula pode ter mais de 7 dígitos | Usuário |
| 4 | Confirmar comportamento ao sobrescrever arquivo já existente no destino | Usuário |
