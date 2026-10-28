# Arquitetura do Bancada

## Objetivo

Bancada será um harness de terminal escrito em Rust para trabalhar em projetos com modelos executados localmente pelo Ollama. O usuário descreve uma tarefa; o modelo consulta e solicita ferramentas; o programa executa as operações permitidas e devolve os resultados ao modelo.

O projeto também serve para aprender Rust e observar o ciclo de ferramentas. Uma motivação é investigar por que um modelo local responde apenas com texto, sem transformar pedidos em alterações nos arquivos.

Um teste bem-sucedido no Bancada não explica automaticamente as falhas de outros agentes. Para diagnosticar esses agentes, ainda seria necessário analisar suas configurações e registros.

## Decisões já tomadas

- Implementar o harness em Rust.
- Começar com uma interface de terminal.
- Usar o Ollama como servidor local de inferência.
- Discutir e implementar a arquitetura gradualmente, com exemplos pequenos.
- Começar pelas ferramentas de listar e ler arquivos; acrescentar edição depois.
- Observar as solicitações e os resultados das ferramentas para distinguir respostas em texto de ações executadas.

Este documento descreve o desenho inicial. Os nomes dos módulos, contratos e dependências abaixo são propostas; ainda não representam funcionalidades implementadas.

## Responsabilidades

O Ollama carrega e executa o modelo. O Bancada mantém a conversa, apresenta as ferramentas, valida solicitações, acessa arquivos, executa processos e entrega os resultados ao modelo.

O modelo não acessa o disco diretamente. Ele produz uma solicitação estruturada, e o código Rust decide como executá-la. Uma resposta que afirma ter alterado um arquivo não é evidência de que uma edição ocorreu: o executor precisa confirmar a operação.

## Fluxo principal

1. O usuário inicia o Bancada em uma pasta de projeto e informa uma tarefa.
2. O controlador monta a conversa com instruções, mensagens anteriores e descrições das ferramentas disponíveis.
3. O cliente envia a requisição HTTP ao Ollama.
4. O cliente interpreta a resposta e o controlador registra a mensagem do assistente no histórico.
5. Se houver chamadas de ferramentas, o executor valida nomes, argumentos, caminhos e permissões.
6. As chamadas são executadas inicialmente em sequência. Cada resultado volta ao histórico associado à chamada correspondente.
7. O controlador consulta o modelo novamente com esses resultados.
8. Quando houver uma resposta final sem chamadas de ferramentas, ela é apresentada ao usuário. O usuário pode continuar a conversa.

O ciclo precisa ter um limite configurável de rodadas e permitir cancelamento. Ao atingir o limite ou encontrar uma falha que impeça continuar, a interface explica o que foi concluído e o que ficou pendente.

Para começar, a resposta do Ollama pode ser recebida inteira, sem streaming. Streaming pode ser acrescentado depois que o ciclo básico estiver funcionando.

## Componentes propostos

| Componente | Responsabilidade |
| --- | --- |
| Interface de terminal | Ler pedidos, mostrar respostas e resultados, apresentar diferenças e solicitar aprovação de operações. |
| Controlador do agente | Manter o histórico e coordenar consultas ao modelo e execuções de ferramentas. |
| Cliente do Ollama | Enviar requisições HTTP, interpretar respostas e converter os dados do servidor para tipos internos. |
| Catálogo de ferramentas | Guardar nomes, descrições, formatos dos argumentos e implementações correspondentes. |
| Executor de ferramentas | Validar solicitações, aplicar permissões e executar as funções Rust. |
| Contexto do projeto | Manter a pasta raiz e resolver os caminhos utilizados nas ferramentas. |
| Registro de execução | Registrar solicitações, resultados, erros e motivos de interrupção para diagnóstico. |

Uma possível organização quando o projeto passar a usar Cargo:

```text
Cargo.toml
src/
  main.rs
  cli.rs
  agent.rs
  ollama.rs
  config.rs
  workspace.rs
  tools/
    mod.rs
    list_files.rs
    read_file.rs
    edit_file.rs
    run_command.rs
```

Essa separação é um destino possível. O primeiro exercício pode continuar pequeno, em um único arquivo, até haver motivo concreto para extrair módulos.

## Comunicação com o Ollama

O cliente utiliza HTTP local e JSON. A configuração deve permitir escolher o endereço do servidor e o nome exato do modelo instalado.

A requisição leva a conversa e o catálogo das ferramentas disponíveis. Cada descrição informa o nome da ferramenta, sua finalidade e o formato dos argumentos aceitos. O cliente deve preservar a relação entre solicitações de ferramentas e seus resultados conforme o protocolo do Ollama utilizado.

Bibliotecas candidatas são `reqwest` para HTTP e `serde` com `serde_json` para serialização. Se adotarmos comunicação assíncrona, podemos utilizar `tokio`. A escolha entre cliente bloqueante e assíncrono fica para a primeira integração; não é necessário introduzir concorrência no exercício de listar arquivos.

A versão exata do modelo Qwen usado anteriormente ainda não foi confirmada. Precisamos verificar se o modelo e sua configuração no Ollama produzem chamadas estruturadas de ferramentas. O catálogo, sozinho, não garante esse comportamento.

Texto livre que menciona uma ferramenta não deve virar um comando executável automaticamente. Se o modelo responder apenas com texto, o registro deve mostrar que nenhuma chamada estruturada foi recebida.

## Contrato das ferramentas

Cada ferramenta tem:

- Um nome estável e uma descrição fornecida ao modelo.
- Argumentos estruturados, convertidos para tipos Rust e validados antes da execução.
- Uma função executora que recebe os argumentos e o contexto do projeto.
- Um resultado de sucesso ou um erro explícito, que possa ser apresentado ao usuário e devolvido ao modelo.

O catálogo deve manter a descrição e a implementação da mesma ferramenta associadas. Uma ferramenta inexistente ou argumentos inválidos geram um resultado de erro; não devem encerrar o programa por um `unwrap`.

### Listar arquivos

Nome proposto: `list_files`.

Recebe um caminho relativo à raiz do projeto. Usa `std::fs::read_dir` e retorna as entradas com nome, caminho relativo e tipo. A primeira versão lista apenas uma pasta por chamada, sem recursão, com ordem estável e limite de entradas.

Abrir a pasta pode falhar, e a leitura de uma entrada também pode falhar. Essas situações precisam ser tratadas e informadas. Não devemos apresentar uma listagem incompleta como se fosse completa; se houver truncamento, o resultado indica isso.

### Ler arquivo

Nome proposto: `read_file`.

Recebe um caminho relativo e retorna conteúdo textual. Deve limitar o volume retornado e informar quando o arquivo excede esse limite ou não pode ser lido como texto. A política inicial pode recusar arquivos acima do limite; leitura por trechos pode vir depois.

### Editar arquivo

Nome proposto: `edit_file`.

Será acrescentada após validar o ciclo de leitura. A proposta é receber uma substituição explícita de um trecho conhecido ou a criação de um arquivo, gerar a diferença para revisão e salvar após aprovação do usuário.

Antes de salvar, o executor confere se o conteúdo ainda corresponde à versão usada para preparar a edição. Se o arquivo mudou nesse intervalo, a alteração precisa ser recalculada. Uma operação recusada ou uma escrita que falhou retorna esse estado ao modelo.

O formato exato da edição ainda precisa ser decidido. Não é necessário implementar inicialmente um interpretador completo de patches.

### Executar programa

Nome proposto: `run_command`.

Recebe o executável, uma lista de argumentos e o diretório de trabalho. Usa `std::process::Command` para capturar saída padrão, saída de erro e código de saída quando disponível.

A mesma função de execução pode atender a pedidos digitados pelo usuário e a chamadas do modelo. A origem do pedido é preservada para aplicar a política de autorização correspondente. Pedidos do modelo para executar programas precisam de aprovação na versão inicial.

Executar um programa diretamente não interpreta operadores de shell. Se uma funcionalidade futura exigir um shell, isso deve ser explícito. A execução deve ter limite de saída, tempo máximo e cancelamento; um código de saída diferente de zero precisa voltar como parte do resultado.

## Caminhos e permissões

A pasta de trabalho é definida ao iniciar a sessão. Caminhos relativos das ferramentas são resolvidos a partir dessa raiz, não de uma pasta que mude silenciosamente durante a conversa.

As ferramentas de arquivos devem verificar caminhos e links simbólicos para manter o acesso no projeto. Para criar arquivos, é necessário validar o diretório pai, mesmo quando o destino ainda não existe.

Essa validação não transforma `Command` em uma sandbox. Um processo iniciado dentro da pasta do projeto ainda pode acessar outros locais conforme as permissões do usuário. Isolamento de processos é uma possível evolução, distinta da validação de caminhos das ferramentas.

## Diagnóstico e erros

O registro deve permitir distinguir:

- Falha de conexão com o Ollama.
- Resposta que contém apenas texto.
- Chamada de ferramenta com nome ou argumentos inválidos.
- Operação recusada pelo usuário.
- Erro de leitura, escrita ou execução.
- Operação concluída e resultado devolvido ao modelo.

Dados enviados ao modelo, saídas de ferramentas e logs podem conter conteúdo do projeto. O registro detalhado deve ser local e configurável, com limites de volume. Não precisamos imprimir todo o histórico a cada rodada.

Nas funções internas que podem falhar, retornar `Result` permite propagar erros com `?`. O controlador decide quais erros viram resultados de ferramentas e quais interrompem a interação; erros recuperáveis não devem derrubar a sessão inteira.

## Etapas de implementação

1. **Exercício local:** listar uma pasta e imprimir as entradas, sem modelo.
2. **Ferramentas de leitura:** separar a listagem e a leitura em funções com argumentos e resultados definidos.
3. **Primeira conversa:** conectar ao Ollama e enviar um pedido simples, sem ferramentas.
4. **Primeiro ciclo de ferramenta:** oferecer `list_files`, executar uma chamada recebida e devolver o resultado ao modelo.
5. **Contexto de arquivos:** acrescentar `read_file` e permitir consultas sucessivas no mesmo histórico.
6. **Edição revisável:** mostrar diferenças, pedir aprovação, salvar e informar o resultado.
7. **Execução de programas:** integrar `Command` com autorização, captura de saída e limites.

A primeira prova do ciclo completo é pedir ao modelo que liste a pasta do projeto e descreva as entradas que a ferramenta efetivamente retornou. A prova seguinte é pedir uma alteração em um arquivo de exemplo e verificar o conteúdo salvo, sem depender apenas da afirmação do modelo.

## Questões em aberto

- Nome e versão exatos do modelo instalado no Ollama.
- Compatibilidade desse modelo e de sua configuração com chamadas de ferramentas.
- Cliente HTTP bloqueante ou assíncrono na primeira integração.
- Formato inicial das edições e comportamento para criação de arquivos.
- Valores dos limites de rodadas, tamanho de leitura, saída e duração de processos.
- Forma de configurar o projeto e persistir ou descartar o histórico de conversa.
