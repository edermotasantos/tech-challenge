# Engineering Tech Challenge

Olá, nós estamos muito animados pelo seu interesse em fazer parte do time de
Engenharia da Nano Capital. Muitas das soluções implementadas internamente são
desenvolvidas em Rust, por isso esperamos que você tenha interesse em trabalhar
com essa linguagem. Não esperamos que você domine Rust, mas apreciamos seu
interesse em aprender e trabalhar com a linguagem.

Assumindo que você já tenha experiência com alguma linguagem de programação,
mas não tenha experiência prévia com Rust, o presente desafio foi desenhado para
avaliar a sua capacidade de assimilar e se adaptar a novas linguagens e tecnologias.

Se você é novo(a) em Rust, os seguintes links podem ser um bom ponto de partida. 
- [A Gentle Introduction To Rust](https://stevedonovan.github.io/rust-gentle-intro/readme.html)
- [The Book](https://doc.rust-lang.org/book/), o livro oficial da linguagem.
- [The Rust Lang Book](https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8) Uma playlist no youtube, que tem como pano de fundo o livro oficial da linguagem.
- Se você prefere aprender por exemplos, você pode gostar [deste conteudo](https://doc.rust-lang.org/rust-by-example/hello.html).

## The challenge: Inventory Update
Para este desafio, considere que um inventário pode ser representado pela seguinte estrutura de dados:

```rust
inventario = [
    [<Inteiro>, <String>],
    [<Inteiro>, <String>],
    [<Inteiro>, <String>],
]
```

Os arrays internos sempre terão dois elementos, sendo o primeiro um número inteiro que representa 
a quantidade do item em estoque. O segundo elemento é uma string, e representa o nome do item.

Nesse desafio, você deve implementar uma função para atualizar os itens em um inventário.
A função deve receber dois argumentos:

- O primeiro argumento `cur_inv` representa o inventário atual (Um array bi-dimensional).
- O segundo argumento `new_inv` representa uma atualizção ao inventario atual (Um array bi-dimensional).

Sua função deve atualizar o inventário atual com base no inventário de atualização e retornar um inventário atualizado.

Ex:

Se a função receber os seguintes inputs:

```rust
cur_inv = [
    [21, "Bowling Ball"],
    [2, "Dirty Sock"],
]

new_inv = [
    [4, "Bowling Ball"],
    [2, "Dirty Sock"],
    [5, "Microphone"]
]
```

O retorno deverá ser:

```rust
output = [
    [25, "Bowling Ball"],
    [4, "Dirty Sock"],
    [5, "Microphone"]
]
```

### Regras

1. Se os inputs forem vazios, o output deve ser vazio
2. Se apenas `cur_inv` for vazio, o resultado deve ser igual à `new_inv`
3. Se apenas `new_inv` for vazio, o resultado deve ser igual à `cur_inv`
4. Se um produto aparece em `cur_inv` mas não em `new_inv`, então este produto
deve aparecer no output com sua quantidade inalterada.
5. Se um produto aparece em `new_inv` mas não em `cur_inv`, então este produto
deve aparecer no output com a mesma quantidade que apareceu em `new_inv`
6. O output deve está ordenado, alfbeticamente pelo nome do item.

P.S: Este desafio foi adaptado de [freecodecamp.org](https://www.freecodecamp.org/learn/coding-interview-prep/algorithms/inventory-update)

P.S.S: Já escrevemos os testes unitários para este desafio. Eles estão em `src/lib.rs`.
Como os testes já validam as regras acima, Você pode usa-los guiar o desenvolvimento, mesmo que você decida implementar a solução em outra linguagem.
Os testes podem ser executados com o seguinte comando:

```shell
cargo test
```
