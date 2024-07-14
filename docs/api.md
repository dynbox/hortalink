# Documentação das Rotas API

## Rota: `/api/v1/exemplo`

### Descrição

Esta rota permite realizar operações relacionadas ao recurso "Exemplo". Ela suporta tanto requisições GET quanto POST.

### Métodos Suportados

#### GET

- **Descrição**: Recupera uma lista de exemplos ou um exemplo específico, dependendo dos parâmetros fornecidos.
- **Query Params**:
    - `query` (string): O termo de pesquisa para filtrar exemplos.
    - `id` (integer | null): ID único do exemplo a ser recuperado.
- **Json Body**:
    - `query` (string): O termo de pesquisa para filtrar exemplos.
    - `id` (integer | null): ID único do exemplo a ser recuperado.
- **Multipart Body**:
    - `query` (string): O termo de pesquisa para filtrar exemplos.
    - `id` (integer | null): ID único do exemplo a ser recuperado.
- **Possíveis Respostas**:
    - **200 OK**: Operação com sucesso.
    - **Json Body**:
        - `id` (integer): Id do exemplo.
        - `description` (string): Descrição do exemplo

#### POST

- **Descrição**: Recupera uma lista de exemplos ou um exemplo específico, dependendo dos parâmetros fornecidos.
- **Query Params**:
    - `query` (string): O termo de pesquisa para filtrar exemplos.
    - `id` (integer | null): ID único do exemplo a ser recuperado.
- **Json Body**:
    - `query` (string): O termo de pesquisa para filtrar exemplos.
    - `id` (integer | null): ID único do exemplo a ser recuperado.
- **Multipart Body**:
    - `query` (string): O termo de pesquisa para filtrar exemplos.
    - `id` (integer | null): ID único do exemplo a ser recuperado.
- **Possíveis Respostas**:
    - **200 OK**: Operação com sucesso.
    - **Json Body**:
        - `id` (integer): Id do exemplo.
        - `description` (string): Descrição do exemplo