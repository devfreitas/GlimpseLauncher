# Glimpse Launcher

<div align="center">
  <img alt="Rust" src="https://img.shields.io/badge/language-Rust-orange.svg">
  <img alt="Hardware Usage" src="https://img.shields.io/badge/hardware-low--usage-blue">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20-lightgrey">
</div>

<br>

<p align="center">
  <b>Um lançador de buscas ultraleve e minimalista, focado em performance absoluta, design moderno e baixo consumo de recursos para Windows 11.</b>
</p>

## Sobre o Projeto

O **Glimpse Launcher** foi desenvolvido para usuários que buscam produtividade sem comprometer o desempenho do sistema. Construído inteiramente em **Rust**, o projeto aproveita a segurança de memória e a velocidade da linguagem para entregar uma interface gráfica que responde instantaneamente, misturando a elegância de ferramentas como o *Raycast* e o *Wox*, com a leveza de um aplicativo nativo.

## Funcionalidades e Recursos

O Glimpse Launcher oferece um conjunto robusto de ferramentas e otimizações projetadas para o uso fluido e eficiente do sistema operacional:

- **Busca Ultrarrápida e Inteligente:** Utiliza o motor `nucleo-matcher` (o mesmo do editor *Helix*) para um *fuzzy finding* de alta precisão e baixíssimo consumo de CPU.
- **Eficiência e Desempenho:** Arquitetura *Singleton* com comunicação IPC (evita múltiplas instâncias em memória) e pré-processamento inteligente de índices, garantindo respostas em milissegundos.
- **Interface Moderna e Intuitiva:** 
  - Design premium inspirado no *Raycast* e *Wox*, com cantos arredondados, sombras dinâmicas e integração total com o Fluent Design do Windows 11.
  - Resultados detalhados com título, subtítulo (caminhos e dicas) e tags visuais (`EXE`, `APP`, `FILE`) para rápida identificação.
- **Personalização de Temas:** Suporte completo a Modo Claro e Modo Escuro, com paletas de cores refinadas e transição instantânea de layout.
- **Controle via Bandeja do Sistema (Tray):**
  - Alternância rápida entre temas.
  - Configuração nativa para inicialização automática com o Windows.
  - Restauração imediata do launcher.
- **Ferramentas Produtivas Integradas:**
  - **Calculadora Inline:** Resolva expressões matemáticas (ex: `25 * 4 / 2`) diretamente na barra de pesquisa.
  - **Busca de Arquivos:** Localize instantaneamente seus documentos pessoais, imagens e downloads através de uma indexação inteligente de múltiplos níveis.
  - **Atalhos Web:** Faça pesquisas rápidas na internet utilizando atalhos pré-definidos (ex: `g sua pesquisa`).

## Como Usar

O **Glimpse Launcher** funciona com atalhos simples para agilizar sua produtividade:

- **Abrir o Launcher:** Pressione `Alt + S` ou clique no ícone da bandeja do sistema.
- **Busca Rápida na Web:** Utilize prefixos. Digite `g sua pesquisa` para buscar no **Google**.
- **Busca Local:** Digite o nome de um aplicativo ou arquivo local para encontrá-lo instantaneamente.
- **Cálculos:** Digite uma equação (ex: `15 + 50 * 2`) para obter a resposta via calculadora embutida.

## Como ser um contribuidor?

Como o projeto é desenvolvido em Rust, você precisará do `cargo` instalado.

#### Instalação e Build

```bash
# 1. Clone o repositório
$ git clone https://github.com/devfreitas/GlimpseLauncher.git

# 2. Acesse o diretório
$ cd GlimpseLauncher

# 3. Compile para a versão de release (otimizada)
$ cargo build --release

# 4. Execute o binário gerado
$ .\target\release\native_launcher.exe
```
- Crie uma branch: `git checkout -b feature/nova-melhoria`.
- Commit suas mudanças: `git commit -m 'feat: Add nova funcionalidade'`.
- Push: `git push origin feature/nova-melhoria`.
- Abra um Pull Request.

<br>

---

<p align="center">
Criado com foco em eficiência por <a href="HTTPS://github.com/devfreitas">DevFreitas</a>
</p>
