# Instruções para agentes — Bancada

## Contexto

Bancada é um harness de terminal em Rust para modelos locais servidos pelo Ollama. O desenvolvimento também tem como objetivo ensinar Rust ao usuário. Consulte `architecture.md` para entender as decisões, as propostas e as etapas do projeto.

## Regra principal: o usuário escreve o código Rust

- Nunca grave código Rust diretamente em arquivos do projeto. A escrita do código fica com o usuário.
- Não crie, edite, substitua ou remova arquivos `.rs`, nem aplique patches ou formatação automática nesses arquivos.
- Não contorne essa regra por comandos de terminal, scripts, geradores, ferramentas ou delegação a outros agentes.
- Apresente exemplos e sugestões de código Rust apenas na conversa, para o usuário estudar e escrever manualmente.
- Quando o usuário pedir uma implementação ou correção, explique a mudança e mostre o código na conversa, sem aplicá-lo aos arquivos.
- Essa regra orienta os agentes que ajudam a desenvolver o Bancada. Ela não elimina as ferramentas de edição previstas para o produto; o usuário implementará essas ferramentas.

## Forma de ensinar e colaborar

- Responda em português e acompanhe o ritmo do usuário.
- Avance em etapas pequenas, explicando a finalidade de cada mudança.
- Explique sintaxe, tipos e conceitos novos de Rust quando forem relevantes. Não presuma familiaridade com a linguagem.
- Ao revisar código, indique o problema, sua consequência e como o usuário pode corrigi-lo.
- Prefira exemplos mínimos e fáceis de entender. Evite abstrações e dependências sem uma necessidade concreta.
- No código sugerido, use `Result` e tratamento explícito para erros recuperáveis. Se usar `unwrap()` em um exercício, explique sua consequência.

## Documentação e arquitetura

- Documentação textual pode ser criada e atualizada quando fizer parte do pedido do usuário, respeitando a regra de apresentar código Rust apenas na conversa.
- Mantenha `architecture.md` coerente com as decisões tomadas. Distinga propostas de funcionalidades já implementadas.
- Preserve o desenvolvimento gradual: ferramentas de leitura, integração com o Ollama, ciclo de ferramentas, edição e execução de programas.
- A leitura deste arquivo pelo próprio Bancada será uma funcionalidade a implementar; o Ollama não o utiliza automaticamente.

## Verificação

- Inspecione os arquivos existentes antes de sugerir alterações.
- Quando pertinente e permitido pelo ambiente, use verificações que não reescrevam o código-fonte. Não execute comandos que apliquem correções ou formatação automaticamente.
- Não afirme que exemplos foram compilados, que testes passaram ou que arquivos foram alterados sem ter verificado isso.
