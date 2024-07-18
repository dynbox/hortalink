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

#### GET 

- **Descrição**: Retorna os caminhos possíveis após a entrada do usuário em sua conta.

#### PATCH

- **Descrição**: Utilizado para atualizar informações de usuários ou retornar um erro caso estas sejam inseridas incorretamente.
- **Multipart Body**:
    - `query` (string): O termo de pesquisa para filtrar exemplos.
    - `id` (integer | null): ID único do exemplo a ser recuperado.

#### Caminhos

- **Descrição**: Utilizados para que o usuário possa realizar operações especificas a sua conta.
- **Nome das Rotas**:
    - `cart`: Permite ao usuário alterar, deletar ou adicionar objetos ao próprio carrinho.
    - `home`: Permite ao usuário visualizar mais pedidos e os pedidos mais recentes.
    - `notifications`: Permite ao usuário ordenar, deletar ou alterar suas notificações.

## Rota `/api/v1/users/@me/cart/:order_id`

### Métodos Suportados

#### Delete

- **Descrição**: Utilizado para deletar pedidos colocados no carrinho utilizando o id especifico do produto.

## Rota `api/v1/users/@me/cart/:order_id/reserve`

### Métodos Suportados

#### Patch

- **Descrição**: Utilizado para realizar a alteração de pedidos já pre-existentes no carrinho.
- **Json Body**:
    - `withdraw` (string | null): Escolhe um pedido pre-existente no carrinho e libera as alterações.
    - `amount` (string | null): Permite escolher, entre o valor minimo de 1 e máximo de 20, a quantidade dos produtos a serem alterados.

## Rota `api/v1/users/@me/cart/`

### Métodos Suportados

#### GET

- **Descrição**: Utilizado para selecionar pedidos pre-existentes ao carrinho do usuário, utilizando os parâmetros especificos.
- **Json Body**:
    - `latitude` (string | null): Retorna a distância atual do usuário até o produto com o minimo de -90.0000000 e máximo de 90.0000000.
    - `longitude` (string | null): Retorna a distância atual do usuário até o produto entre com o minimo de -180.0000000 e máximo de 180.0000000.

#### POST

- **Descrição**: Utilizado para adicionar novos pedidos ao carrinho do usuário.
- **Json Body**:
    - `withdraw` (string | null): Seleciona o id do(s) produto(s) do(s) Vendedor(s) e os adiciona no carrinho.
    - `amount` (string | null): Permite escolher, entre o valor minimo de 1 e máximo de 20, a quantidade dos produtos a serem adicionados ao carrinho.


## Rota `api/v1/users/@me/home`

### Descrição

Essa rota permite realizar operações relacionadas a tela principal de usuário. Ela suporta requisições GET.

## Rota `api/v1/users/@me/home/`

### Métodos Suportados

#### GET

- **Descrição**: Utilizado para retornar informações da tela inicial "Home" .
- **Json Body**:
    - `latitude` (string | null): Retorna a distância atual do usuário até o produto com o minimo de -90.0000000 e máximo de 90.0000000.
    - `longitude` (string | null): Retorna a distância atual do usuário até o produto entre com o minimo de -180.0000000 e máximo de 180.0000000.

## Rota `api/v1/users/@me/home/most_recent/`

### Métodos Suportados

#### GET

- **Descrição**: Utilizado para retornar informações dos pedidos mais recentes realizados.
- **Query Params**:
    - `latitude` (string | null): Retorna a distância atual do usuário até o produto com o minimo de -90.0000000 e máximo de 90.0000000.
    - `longitude` (string | null): Retorna a distância atual do usuário até o produto entre com o minimo de -180.0000000 e máximo de 180.0000000.
    - `page` (number): Define o número da página, com valor mínimo de 1 e máximo de 100, para carregamento da primeira (1) até a última (100).
    - `per_page` (number): Define o número de itens, com valor mínimo de 5 e máximo de 100. Carregando 5 itens por página.

## Rota `api/v1/users/@me/home/more_orders/`

### Métodos Suportados

#### GET

- **Descrição**: Utilizado para retornar informações dos pedidos mais recentes realizados.
- **Query Params**:
    - `latitude` (string | null): Retorna a distância atual do usuário até o produto com o minimo de -90.0000000 e máximo de 90.0000000.
    - `longitude` (string | null): Retorna a distância atual do usuário até o produto entre com o minimo de -180.0000000 e máximo de 180.0000000.
    - `page` (number): Define o número da página, com valor mínimo de 1 e máximo de 100, para carregamento da primeira (1) até a última (100).
    - `per_page` (number): Define o número de itens, com valor mínimo de 5 e máximo de 100. Carregando 5 itens por página.

## Rota: `/api/v1/users/@me/notifications/`

### Descrição

Esta rota permite realizar operações relacionadas as notificações do aplicativo. Ela suporta requisições GET.

### Métodos Suportados

#### GET

- **Descrição**: Utilizado para selecionar notificações, com base nos parâmetros aplicados.

## Rota: `/api/v1/users/@me/notifications/:notification_id`

### Descrição

Esta rota permite modificar as notificações do aplicativo. Ela suporta requisições DELETE e PATCH.

### Métodos Suportados

#### DELETE

- **Descrição**: Utilizado para selecionar e deletar notificações com base, nos parâmetros aplicados.

#### PATCH

- **Descrição**: Utilizado para selecionar e realizar alterações nas notificações, com base nos parâmetros aplicados.

## Rota: `/api/v1/users/@me/orders/`

### Descrição

Esta rota permite realizar operações relacionadas a pedidos. Ela suporta requisições GET.

### Métodos Suportados

#### GET

- **Descrição**: Utilizado para retornar um pedido, com base nos parâmetros aplicados.

## Rota: `/api/v1/users/@me/orders/:order_id`

### Descrição

Esta rota permite realizar operações relacionadas a apagar pedidos. Ela suporta requisições DELETE.

### Métodos Suportados

#### DELETE

- **Descrição**: Utilizado para selecionar e deletar pedidos, com base nos parâmetros aplicados.

## Rota: `/api/v1/sellers/:seller_id/products`

### Descrição

TODO

### Métodos Suportados

#### GET
- **Descrição**: TODO
- **Query Params**:
  - `latitude` (string | null): Retorna a distância atual do usuário até o produto com o minimo de -90.0000000 e máximo de 90.0000000.
  - `longitude` (string | null): Retorna a distância atual do usuário até o produto entre com o minimo de -180.0000000 e máximo de 180.0000000.
  - `page` (number): Define o número da página, com valor mínimo de 1 e máximo de 100, para carregamento da primeira (1) até a última (100).
  - `per_page` (number): Define o número de itens, com valor mínimo de 5 e máximo de 100. Carregando 5 itens por página.

#### POST
- **Descrição**: TODO
- **Multipart Body**:
  - `product_id` (number | null): Identificador único do tipo de produto a ser vendido. Deve ser um valor inteiro não negativo.
  - `price` (float): Preço do produto. Deve ser um valor decimal não negativo, com precisão até duas casas decimais após a vírgula.
  - `quantity` (number): Quantidade disponível do produto. Pode ser nulo se não aplicável.
  - `photos` (bytes[]): Vetor de fotos do produto. Cada foto deve ter um tamanho máximo de 25 MiB e o vetor pode conter de 1 a 5 fotos.
  - `schedules_id` (number[]): Vetor de identificadores de horários disponíveis para entrega ou coleta do produto. Pode conter até 5 IDs.
  - `unit` (number): Unidade de medida do produto. Deve ser um valor inteiro de 16 bits não negativo, com valores permitidos de 0 a 5.
  - `unit_quantity` (float): Quantidade por unidade de medida. Deve ser um valor decimal não negativo.
  - `description` (string | null): Descrição detalhada do produto. Pode ser nulo se não aplicável. Se fornecido, deve ter um mínimo de 10 caracteres e um máximo de 2096 caracteres.

## Rota: `/api/v1/sellers/:seller_id/products/:product_id`

### Descrição

TODO

### Métodos Suportados

#### GET
- **Descrição**: TODO

#### DELETE
- **Descrição**: TODO

#### PATCH
- **Descrição**: TODO
- **Multipart Body**:
  - `price` (float | null): Novo preço do produto. Deve ser um valor decimal positivo, com precisão até duas casas decimais após a vírgula. Pode ser nulo se não aplicável.
  - `quantity` (number | null): Nova quantidade disponível do produto. Deve ser um valor inteiro positivo. Pode ser nulo se não aplicável.
  - `unit` (number | null): Nova unidade de medida do produto. Deve ser um valor inteiro de 16 bits não negativo, com valores permitidos de 0 a 5.
  - `unit_quantity` (float | null): Nova quantidade por unidade de medida. Deve ser um valor decimal não negativo.
  - `add_photos` (bytes[]): Vetor de novas fotos para adicionar ao produto. Cada foto deve ter um tamanho máximo de 25 MiB. Não há restrição sobre o número de fotos adicionais.
  - `remove_photos` (string[]): Vetor de identificadores das fotos existentes que devem ser removidas do produto. Cada string representa o ID de uma foto.
  - `add_schedules` (number[]): Vetor de identificadores de novos horários disponíveis para entrega ou coleta do produto.
  - `remove_schedules` (number[]): Vetor de identificadores de horários existentes que devem ser removidos.
  - `description` (string | null): Nova descrição detalhada do produto. Pode ser nulo se não aplicável. Se fornecido, deve ter um mínimo de 10 caracteres e um máximo de 2096 caracteres.

## Rota: `/api/v1/sellers/:seller_id/products/:product_id/ratings`

### Descrição

TODO

### Métodos Suportados

#### GET
- **Descrição**: TODO

#### POST
- **Descrição**: TODO

## Rota: `/api/v1/sellers/:seller_id/products/:product_id/ratings/:rating_id`

### Descrição

TODO

### Métodos Suportados

#### DELETE
- **Descrição**: TODO

#### PATCH
- **Descrição**: TODO
- **Json Body**:
  - `rating` (number | null): Quantidade de estrelas de 1 á 5
  - `content` (string | null): Conteúdo da avaliação

## Rota: `/api/v1/sellers/:seller_id/schedules`

### Descrição

TODO

### Métodos Suportados

#### POST
- **Descrição**: TODO
- **Json Body**:
  - `location`: Localização do ponto de venda
    - `latitude` (float):
    - `longitude` (float):
  - `address` (string): Endereço humano do ponto de venda
  - `start_time` (time): Que horas abre
  - `end_time` (time): Que horas fecha
  - `day_of_week` (number): Número de 0 á 6 començando da segunda

#### GET
- **Descrição**: TODO

## Rota: `/api/v1/sellers/:seller_id/schedules/:schedule_id`
### Descrição

TODO

### Métodos Suportados

#### PATCH
- **Descrição**: TODO
- **Json Body**:
  - `location` (nullable): Localização do ponto de venda
    - `latitude` (float):
    - `longitude` (float):
  - `address` (string | null): Endereço humano do ponto de venda
  - `start_time` (time | null): Que horas abre
  - `end_time` (time | null): Que horas fecha
  - `day_of_week` (number | null): Número de 0 á 6 començando da segunda
#### DELETE
