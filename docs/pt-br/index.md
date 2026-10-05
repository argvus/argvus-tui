---
title: ARGVUS TUI
description: Framework Terminal User Interface para ARGVUS
---

# ARGVUS TUI

ARGVUS TUI é um framework Terminal User Interface que fornece componentes de UI consistentes, orientados por teclado, para aplicativos e ferramentas ARGVUS baseados em texto.

## Características

- **Navegação Orientada por Teclado**: Projetado para interação eficiente orientada por teclado
- **Estilo Consistente**: Aparência visual unificada em aplicativos TUI
- **Suporte a Temas**: Se adapta aos esquemas de cores ARGVUS
- **Acessibilidade**: Construído com interfaces de texto acessíveis em mente

## Componentes

O framework TUI inclui:
- Campos de entrada de texto
- Menus de seleção e listas
- Indicadores de progresso
- Caixas de diálogo
- Exibições de status
- Visualizadores de ajuda e documentação

### Lista única de menu (`menu`)

Cada página é uma lista vertical única de linhas tipadas (`RowKind`):
**Info** (rótulo e valor, nunca recebe foco), **Action**, **Submenu**
(marcador `›`), **Toggle** (`[x]`/`[ ]`), **Choice** (`●` na opção atual),
**Value** (Enter edita; `←/→` ajustam quando há passo), **Destructive**
(sempre passa pela confirmação) e **Separator**. Cada linha carrega o
próprio ícone do catálogo `icons`; linhas Info não têm ícone decorativo.

O cursor (`MenuState`) só para em linhas selecionáveis: Info, separadores e
linhas desabilitadas são pulados por `↑/↓`, `j/k`, `PgUp/PgDn` e
`Home/End`, e o cursor para nas pontas, sem voltar ao início. Uma página
sem linhas selecionáveis apenas rola. Com os ícones desligados, a coluna de
ícone some e os rótulos continuam alinhados.

### Confirmação (`confirm`)

Um único componente confirma ações destrutivas ou privilegiadas: título,
mensagem e duas linhas, Confirmar e Cancelar. O foco começa em Cancelar.
`Enter` executa a linha focada, `y` confirma e `n`/`Esc` cancelam. Pode
exibir uma contagem regressiva (por exemplo, para reverter a configuração
de monitores).

### Rodapé contextual (`hints`)

O rodapé de ajuda é montado a partir do tipo da linha selecionada e das
capacidades da página (busca, atualizar, voltar, sair), mostrando só as
teclas que valem para a linha atual. Os textos vêm do catálogo
`control-center` do `argvus-i18n` (chaves `control_center.hint.*`).

## Uso

ARGVUS TUI é usado em:
- Ferramentas de administração do sistema
- Utilitários de instalação e configuração
- Exibições de status e monitoramento
- Interfaces de configuração

## Para Desenvolvedores

Para usar ARGVUS TUI em seu aplicativo:

1. Adicione `argvus-tui` como dependência
2. Importe componentes TUI em seu projeto
3. Use a API fornecida para elementos de UI
4. Siga as diretrizes de UI do ARGVUS

Para documentação detalhada da API, veja o repositório.
