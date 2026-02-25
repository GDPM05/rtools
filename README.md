### Rust Coreutils Reimplementation

Este repositório contém a minha implementação pessoal de algumas das ferramentas de linha de comando mais comuns do ecossistema Linux/Unix, desenvolvidas inteiramente em Rust.

#### 🎯 Objetivo
O objetivo principal deste projeto é estritamente **educacional**. O foco está em:
*   Aprender a sintaxe e os conceitos fundamentais do Rust (Ownership, Borrowing, Enums, etc).
*   Explorar a biblioteca padrão (`std`) para manipulação de ficheiros e fluxos de sistema.
*   Praticar a gestão de erros idiomática em Rust.
*   Compreender o funcionamento interno de ferramentas que utilizamos diariamente no terminal.

#### 🛠 Ferramentas Implementadas
A ideia é recriar versões simplificadas das seguintes utilidades:

*   `echo`: Exibição de texto e variáveis no terminal.
*   `cat`: Leitura e concatenação de ficheiros.
*   `ls`: Listagem de diretórios e metadados de ficheiros.
*   `find`: Procura de ficheiros numa hierarquia de diretórios.
*   `grep`: Procura de padrões de texto utilizando expressões regulares ou strings simples.

#### 📝 Notas
Estas implementações não pretendem substituir as versões oficiais (GNU Coreutils ou uutils), mas sim servir como um exercício de programação para consolidar conhecimentos na linguagem.
