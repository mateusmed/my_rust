

Site oficial
```
https://www.rust-lang.org/tools/install
```

instalando rust em maquina linux (WSL)
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

criando o projeto
```
cargo new first_test
```

instalando essential no linux
```
sudo apt-get update
sudo apt install build-essential
```

executando
```
cargo run
```

gerando versão de release
```
cargo build --release
```

executando versao de release
```
./first_test/release/firt_test
```

gerando documentação de um unico arquivo:
```
rustdoc /src/main.rs
```

gerando documentação do projeto:
```
cargo doc
```

---

necessario instalar o cargo-watch
```
cargo install cargo-watch
```
vai compilanod ao salvar os arquivos
```
cargo watch -x run
```