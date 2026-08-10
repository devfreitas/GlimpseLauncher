<div align="center">

# Glimpse Launcher

**Um lançador de buscas para desktop ultraleve e minimalista, focado em performance absoluta, design moderno e baixo consumo de recursos para Windows 11.**


<br />

<p align="center">
  <em>
    Inicie qualquer aplicativo em menos de <strong>50ms</strong>.
</p>

<p align="center">
  <a href="#sobre-o-projeto">Sobre</a> •
  <a href="#recursos-e-funcionalidades">Recursos</a> •
  <a href="#início-rápido">Início Rápido</a> •
  <a href="#como-usar">Uso</a> •
  <a href="#arquitetura">Arquitetura</a>
</p>

</div>

<br />


## Sobre o Projeto

O **Glimpse Launcher** foi desenvolvido para usuários que buscam produtividade sem comprometer o desempenho do sistema. Construído inteiramente em **Rust**, o projeto aproveita a segurança de memória e a velocidade da linguagem para entregar uma interface gráfica que responde instantaneamente, mas com a leveza de um aplicativo nativo.

<br />

## Recursos e Funcionalidades

O Glimpse Launcher oferece um conjunto robusto de ferramentas e otimizações projetadas para o uso fluido e eficiente do sistema:

### Pesquisa Ultrarrápida e Inteligente

- **Pesquisa fuzzy ultrarrápida** — Alimentada pelo motor `nucleo-matcher` (o mesmo do editor *Helix*) para um *fuzzy finding* de alta precisão e baixíssimo consumo de CPU.
- **Indexação de aplicativos UWP e Win32** — Descobre tanto aplicativos clássicos de desktop quanto aplicativos modernos da Microsoft Store automaticamente.
- **Observador de sistema de arquivos** — Atualizações de índice em tempo real via `notify` quando aplicativos são instalados ou removidos.
- **Cache de índice persistente** — Inicialização extremamente rápida com índice serializado via `bincode`.

### Produtividade e Ferramentas Integradas

- **Calculadora inline** — Resolva expressões matemáticas (ex: `25 * 4 / 2` ou `2^10 + 3 * 7`) diretamente na barra de pesquisa para resultados instantâneos (via `evalexpr`).
- **Pesquisa na Web e Atalhos** — Utilize prefixos rápidos `g` para pesquisar no **Google** diretamente do launcher.
- **Comandos de terminal** — Digite `>` seguido do seu comando (ex: `> ping google.com`) para executá-lo no prompt sem sair do launcher.
- **Busca de Diretórios Local** — Localize instantaneamente seus diretórios através de uma indexação inteligente de múltiplos níveis.

### Interface de Usuário Moderna e Intuitiva

- **Temas e Cores de Destaque** — Transição instantânea entre Claro e Escuro, com suporte a paletas customizáveis de Cores de Destaque (Accent Colors).
- **Fluent Design do Windows 11** — Integração total com cantos arredondados, fundo fosco translúcido e animações suaves.
- **Painel de Configurações** — Sidebar de fácil navegação para controlar recursos, atalho global, aparência e mais.
- **Posicionamento Customizável** — Use um Grid 3x3 interativo para afixar a janela exatamente onde deseja, ou ative o movimento livre arrastável.
- **Resultados Detalhados** — Exibe título, subtítulo (caminhos e dicas) e tags visuais (`EXE`, `APP`, `FILE`) em uma lista refinada.

### Eficiência e Sistema

- **Arquitetura Singleton** — Proteção baseada em IPC impede múltiplas instâncias em memória usando `interprocess`.
- **Início automático com o Windows** — Configuração nativa e opcional via registro do Windows para iniciar no login.
- **Controle via Bandeja do Sistema (Tray)** — Operação em segundo plano com alternância rápida de temas e fácil acesso a configurações pelo menu de contexto.
- **Alocador personalizado** — Usa `mimalloc` para redução no consumo de memória e alocações mais rápidas.

<br />

## Início Rápido

### Opção A: Baixar a Release

1. Acesse [**Releases**](https://github.com/devfreitas/GlimpseLauncher/releases) ou o site oficial [**Glimpse Launcher**](https://glimpselauncher.vercel.app/)
2. Baixe o instalador `.exe` mais recente
3. Execute e inicie o launcher com **Alt + S**

### Opção B: Compilar a partir do Código-Fonte

```bash
# 1. Clone o repositório
git clone https://github.com/devfreitas/GlimpseLauncher.git

# 2. Acesse o diretório
cd GlimpseLauncher

# 3. Compilar em modo release (otimizado)
cargo build --release

# 4. Executar o binário gerado
./target/release/glimpse_launcher.exe
```
> Compilar a partir do código-fonte requer a [Toolchain do Rust](https://rustup.rs/) e um ambiente de desenvolvimento Windows 11 com o Windows SDK.

<br />

## Como Usar

O **Glimpse Launcher** funciona com atalhos e prefixos simples para agilizar sua produtividade:

### Atalhos Principais

| Atalho | Ação |
|:---|:---|
| `Alt + S` (Customizável)| Alternar a visibilidade do launcher |
| `↑` `↓` | Navegar pelos resultados |
| `Enter` | Iniciar o aplicativo selecionado / executar comando |
| `Escape` | Ocultar o launcher |

### Comandos e Prefixos

| Prefixo / Entrada | Ação | Exemplo |
|:---|:---|:---|
| *(apenas digite)* | Pesquisa fuzzy local de aplicativos e arquivos | `fire` → Firefox |
| `g <consulta>` | Busca rápida no Google | `g rust async` |
| `> <comando>` | Executar comando no prompt / terminal | `> ping 8.8.8.8` |
| `> config` | Abrir painel de configurações | `> config` |
| *expressão matemática* | Calculadora inline | `15 + 50 * 2` → `115` |

<br />

## Arquitetura

O Glimpse segue uma **arquitetura modular** limpa com separação clara de responsabilidades:

```
┌──────────────────────────────────────────────┐
│                   main.rs                    │
│          Ponto de entrada & orquestração     │
├──────────┬──────────────┬────────────────────┤
│  core/   │     os/      │       ui/          │
│          │              │                    │
│ indexer  │   hotkey     │   launcher UI      │
│ search   │   window     │   (egui / eframe)  │
│ config   │              │                    │
└──────────┴──────────────┴────────────────────┘
```

| Módulo | Responsabilidade |
|:---|:---|
| **`core/`** | Indexação de aplicativos (UWP + Win32), motor de pesquisa fuzzy, gerenciamento de configurações |
| **`os/`** | Registro de atalho global, manipulação de janelas nativas (Win32 API) |
| **`ui/`** | Interface completa do launcher renderizada com `egui` via `eframe` |
| **`constants.rs`** | Constantes compartilhadas e padrões de todo o aplicativo |

<br />



## Estrutura do Projeto

```
glimpse_launcher/
├── src/
│   ├── main.rs              # Ponto de entrada, proteção IPC, bandeja & event loop
│   ├── constants.rs          # Constantes & padrões da aplicação
│   ├── core/
│   │   ├── mod.rs            # Declarações de módulo
│   │   ├── indexer.rs        # Descoberta + cache de aplicativos UWP & Win32
│   │   ├── search.rs         # Pesquisa fuzzy via nucleo-matcher
│   │   └── config.rs         # Configurações de usuário (persistência TOML)
│   ├── os/
│   │   ├── mod.rs            # Declarações de módulo
│   │   ├── hotkey.rs         # Registro de atalho global (Alt+S)
│   │   └── window.rs         # Gerenciamento & posicionamento de janela Win32
│   ├── ui/
│   │   ├── mod.rs            # Declarações de módulo
│   │   └── ui.rs             # Interface completa do launcher (egui/eframe)
│   └── bin/
│       └── test_apps.rs      # Utilitário de desenvolvimento para testar a indexação de apps
├── public/
│   ├── icon.png              # Ícone do aplicativo (alta resolução)
│   └── icone.ico             # Ícone do aplicativo (formato ICO do Windows)
├── installer/
│   └── Teste.iss             # Script de instalador Inno Setup
├── build.rs                  # Script de build (incorporação de ícone winres)
├── Cargo.toml                # Dependências & configurações de build
├── Cargo.lock                # Árvore de dependências reproduzível
├── LICENSE                   # Licença MIT
└── README.md                 # Você está aqui
```

<br />

## Stack de Tecnologias

| Crate | Propósito |
|:---|:---|
| [`eframe`](https://crates.io/crates/eframe) | Framework de GUI (backend egui para renderização nativa) |
| [`nucleo-matcher`](https://crates.io/crates/nucleo-matcher) | Motor de correspondência fuzzy de alto desempenho |
| [`windows`](https://crates.io/crates/windows) | Bindings oficiais da API Win32 da Microsoft |
| [`winapi`](https://crates.io/crates/winapi) | Acesso adicional de baixo nível à API do Windows |
| [`tray-icon`](https://crates.io/crates/tray-icon) | Suporte multiplataforma à bandeja do sistema |
| [`interprocess`](https://crates.io/crates/interprocess) | IPC para imposição de instância singleton |
| [`notify`](https://crates.io/crates/notify) | Observador de eventos do sistema de arquivos para reindexação em tempo real |
| [`bincode`](https://crates.io/crates/bincode) | Serialização binária rápida para cache de índice |
| [`evalexpr`](https://crates.io/crates/evalexpr) | Avaliador de expressões matemáticas para a calculadora inline |
| [`mimalloc`](https://crates.io/crates/mimalloc) | Alocador de memória de alto desempenho da Microsoft |
| [`serde`](https://crates.io/crates/serde) / [`toml`](https://crates.io/crates/toml) | Serialização e desserialização de configurações |
| [`egui-phosphor`](https://crates.io/crates/egui-phosphor) | Conjunto de ícones Phosphor para a UI |
| [`webbrowser`](https://crates.io/crates/webbrowser) / [`urlencoding`](https://crates.io/crates/urlencoding) | Abertura de links de pesquisa na web |
| [`winreg`](https://crates.io/crates/winreg) | Acesso ao registro do Windows (ex: iniciar com Windows) |
| [`walkdir`](https://crates.io/crates/walkdir) / [`dirs`](https://crates.io/crates/dirs) | Busca em diretórios e caminhos padrão do sistema |

<br />

## Contribuindo

Contribuições são bem-vindas! Como o projeto é desenvolvido em Rust, você precisará do `cargo` instalado. Sejam relatos de bugs, solicitações de recursos ou pull requests — toda contribuição é valorizada.

Por favor, leia o [**CONTRIBUTING.md**](CONTRIBUTING.md) para diretrizes sobre como começar.

```bash
# Fork → Clone → Branch → Code → PR
git checkout -b feature/recurso-incrivel
cargo test
cargo clippy
git commit -m "feat: adicionar recurso incrivel"
git push origin feature/recurso-incrivel
```

<br />

## Licença

Este projeto está licenciado sob a **Licença MIT** — veja o arquivo [LICENSE](LICENSE) para detalhes.

<br />

---

<div align="center">
<sub>Criado com foco em eficiência em Rust 🦀</sub>
</div>
