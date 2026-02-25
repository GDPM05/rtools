### Rust Coreutils Reimplementation

Este repositório contém a minha implementação pessoal de algumas das ferramentas de **CLI** mais comuns do ecossistema Linux/Unix, desenvolvidas inteiramente em Rust.

#### 🎯 Objetivo
O propósito deste projeto é estritamente **educacional**. O foco principal é:
*   Consolidar a sintaxe e os conceitos fundamentais de Rust (**Ownership**, **Borrowing**, **Lifetimes**, etc.).
*   Explorar a **Standard Library** (`std`) para manipulação de **File Systems** e **I/O Streams**.
*   Praticar **Error Handling** idiomático em Rust.
*   Compreender o funcionamento interno de ferramentas que utilizamos diariamente no terminal.

#### 🛠 Ferramentas Implementadas
O projeto foca-se em recriar versões simplificadas das seguintes utilidades:

*   `echo`: Exibição de texto e variáveis no terminal.
*   `cat`: Leitura e concatenação de ficheiros.
*   `ls`: Listagem de diretórios e metadados de ficheiros.
*   `find`: Procura de ficheiros numa hierarquia de diretórios.
*   `grep`: Procura de padrões de texto utilizando **Regular Expressions** ou strings simples.

#### 📝 Notas
Estas implementações não pretendem substituir as versões oficiais (**GNU Coreutils** ou **uutils**), servindo apenas como um exercício de programação para aprofundar conhecimentos na linguagem.
