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



## Rota: `/api/v1/users/`

### Descrição

Esta rota permite realizar operações relacionadas ao recurso "Users". Ela suporta requisições GET.

### Métodos Suportados

#### GET

- **Descrição**: Utilizado para filtragem de usuários, sem utilização de parâmetros especificos.
- **Query Params**:
    - `query` (string | null ): Especifica os usuários que começam com o texto inserido, tendo entre 3 e 64 caracteres.
    - `page` (number): Define o número da página, com valor mínimo de 1 e máximo de 100, para carregamento da primeira (1) até a última (100).
    - `per_page` (number): Define o número de itens, com valor mínimo de 5 e máximo de 100. Carregando 5 itens por página.
- **Possíveis Respostas**:
    - **200 OK**: Operação com sucesso.


## Rota: `/api/v1/users/:id`

### Descrição

Esta rota permite realizar operações relacionadas ao recurso "Users". Ela suporta requisições GET.

### Métodos Suportados

#### GET

- **Descrição**: Utilizado para filtragem de usuários, utilizando o parâmetro de id especifico.
- **Json Body**:
    - `UserResponse` (string): profile (PublicProfile), Definição de Perfil.
- **Possíveis Respostas**:
    - **200 OK**: Operação com sucesso.


## Rota `/api/v1/users/@me`

### Descrição

Essa rota permite realizar operações relacionadas ao próprio Usuário. Ela serve como caminho para estas operações.

### Métodos Suportados

#### Caminhos

- **Descrição**: Utilizados para que o usuário possa realizar operações especificas a sua conta.
- **Nome das Rotas**:
    - `cart`: Permite ao usuário alterar, deletar ou adicionar objetos ao próprio carrinho.
    - `home`: Permite ao usuário visualizar mais pedidos e os pedidos mais recentes.
    - `notifications`: Permite ao usuário ordenar, deletar ou alterar suas notificações.

## Rota 