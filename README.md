# Glimpse Launcher

<div align="center">
  <img alt="Rust" src="https://img.shields.io/badge/language-Rust-orange.svg">
  <img alt="Hardware Usage" src="https://img.shields.io/badge/hardware-low--usage-blue">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20-lightgrey">
</div>

<br>

<p align="center">
  <b>Um lançador de buscas ultraleve e minimalista, focado em performance absoluta e baixo consumo de recursos para Windows 11.</b>
</p>

## Sobre o Projeto

O **Glimpse Launcher** foi desenvolvido para usuários que buscam produtividade sem comprometer o desempenho do sistema. Construído inteiramente em **Rust**, o projeto aproveita a segurança de memória e a velocidade da linguagem para entregar uma interface gráfica minimalista que responde instantaneamente, seja para encontrar arquivos locais ou realizar buscas na web.

## Diferenciais

- **Powered by Rust:** Performance de nível nativo com footprint de memória mínimo.
- **Baixo Uso de Hardware:** Ideal tanto para máquinas de alto desempenho quanto para sistemas com recursos limitados.
- **Busca Local:** Localização rápida de arquivos, pastas e aplicações no computador.
- **Integração Web:** Atalhos inteligentes para motores de busca sem precisar abrir o navegador manualmente primeiro.
- **Interface Minimalista:** Design "Distraction-free" focado no que importa: o resultado da busca.

## Como Usar

O **Glimpse Launcher** funciona com atalhos simples para agilizar sua produtividade:

- **Abrir o Launcher:** Pressione `Alt + S` para exibir a barra de busca a qualquer momento.
- **Busca Rápida na Web:** Utilize prefixos antes da sua pesquisa para buscar diretamente em sites específicos. Por exemplo:
  - Digite `g sua pesquisa` para buscar no **Google**.
  - Digite `y sua pesquisa` para buscar no **YouTube**.
- **Busca Local:** Digite o nome de um aplicativo ou arquivo local para encontrá-lo instantaneamente.

## Configuração

O **Glimpse Launcher** é altamente configurável através de arquivos simples, permitindo definir:

- Motores de busca web favoritos.
- Diretórios de indexação para busca local.
- Atalhos de teclado personalizados.

## Contribuições
Este é um projeto de código aberto e focado na comunidade. Se você deseja otimizar ainda mais o código ou adicionar novas funcionalidades:
***Faça um Fork.***

### Como ser um contribuidor ?

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
- Crie uma branch: git checkout -b feature/nova-melhoria.
- Commit suas mudanças: git commit -m 'feat: Add nova funcionalidade'.
- Push: git push origin feature/nova-melhoria.
- Abra um Pull Request.


<br>

---

<p align="center">
Criado com foco em eficiência por <a href="HTTPS://github.com/devfreitas">DevFreitas</a>
</p>
