# AlariaChain

1º Compile the programm
cargo build

2º Show the help
./target/debug/alariachain -h

3º Add a transaction to the buffer
./target/debug/alariachain addtransaction --to diego --data hola

4º Print transactions buffer
./target/debug/alariachain printtransactions

5º Remove transactions buffer
./target/debug/alariachain printtransactions

<!-- 3º Init the Blockchain
./target/debug/alariachain init

4º Print the genesis file
./target/debug/alariachain printgenesis

5º Add a block
./target/debug/alariachain addblock test

6º Print the Blockchain
./target/debug/alariachain printchain

7º Remove the Blockchain
./target/debug/alariachain dropchain -->



NEXT STEPS:

- Implement Merklle tree in order to have a addtransaction and mineblock commands