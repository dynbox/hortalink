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

## Rota: `/api/v1/users/:id`

### Descrição

Esta rota permite realizar operações relacionadas ao recurso "Users". Ela suporta requisições GET.

### Métodos Suportados

#### GET

- **Descrição**: Utilizado para filtragem de usuários, utilizando o parâmetro de id especifico.

## Rota `/api/v1/users/@me`

### Descrição

Essa rota permite realizar operações relacionadas ao próprio usuário. Ela serve como caminho para estas operações.

### Métodos Suportados

#### Caminhos

- **Descrição**: Utilizados para que o usuário possa realizar operações especificas a sua conta.
- **Nome das Rotas**:
    - `cart`: Permite ao usuário alterar, deletar ou adicionar objetos ao próprio carrinho.
    - `home`: Permite ao usuário visualizar mais pedidos e os pedidos mais recentes.
    - `notifications`: Permite ao usuário ordenar, deletar ou alterar suas notificações.


## Rota `/api/v1/users/@me/cart`

### Descrição

Essa rota permite realizar operações relacionadas ao carrinho do usuário. Ela suporta requisições Delete, Patch, GET e Patch

### Componentes 

## Rota `/api/v1/users/@me/cart/:order_id`

### Métodos Suportados

#### Delete

- **Descrição**: Utilizado para deletar pedidos colocados no carrinho utilizando o id especifico do produto.

## Rota `api/v1/users/@me/cart/:order_id/reserve`

### Métodos Suportados

#### Patch

- **Descrição**: Utilizado para realizar a alteração de pedidos já pre-existentes no carrinho.
- **Json Body**:
    - `PatchProductCart` (string | null): Permite escolher, entre o valor minimo de 1 e máximo de 20, produtos no carrinho para realizar alterações nos mesmos.

## Rota `api/v1/users/@me/cart/`

### Métodos Suportados

#### GET

- **Descrição**: Utilizado para selecionar pedidos pre-existentes ao carrinho do usuário, utilizando os parâmetros especificos.
- **Json Body**:
    - `Location` (string | null): Retorna a distância atual do usuário até o produto entre a latitude minima de -90.0000000 e máxima de 90.0000000 e a longitude minima de -180.0000000 e máxima de 180.0000000.

## Rota `api/v1/users/@me/cart/`

### Métodos Suportados

#### POST

- **Descrição**: Utilizado para adicionar novos pedidos ao carrinho do usuário.
- **Json Body**:
    - `PostProductCart` (string | null): Seleciona o id do(s) produto(s) do(s) Vendedor(s) e retorna, entre o valor minimo de 1 e máximo de 20, produtos a serem adicionados ao carrinho.


## Rota `api/v1/users/@me/home`

### Descrição

Essa rota permite realizar operações relacionadas a tela principal de usuário. Ela suporta requisições GET.

### Componentes

## Rota `api/v1/users/@me/home/`

### Métodos Suportados

#### GET

- **Descrição**: Utilizado para retornar informações da tela inicial "Home" .
- **Json Body**:
    - `Location` (string | null): Retorna a localização atual do usuário entre a latitude minima de -90.0000000 e máxima de 90.0000000 e a longitude minima de -180.0000000 e máxima de 180.0000000.

## Rota `api/v1/users/@me/home/most_recent`

### Métodos Suportados

#### GET

- **Descrição**: Utilizado para retornar informações da tela inicial "Home" .
- **Json Body**:
    - `Location` (string | null): Retorna a localização atual do usuário entre a latitude minima de -90.0000000 e máxima de 90.0000000 e a longitude minima de -180.0000000 e máxima de 180.0000000.