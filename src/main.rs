mod kibi;
use kibi::blockchain::Blockchain;

fn main() {
   let mut blockchain = Blockchain::new();

   // Add new transactions
   let tx_data = "meu nome nao é johny".to_string(); // TODO: Lets use JSON.stringfy
   let tx_data2 = "dados ocultos".to_string();
   blockchain.add_new_transaction(tx_data);
   blockchain.add_new_transaction(tx_data2);

   // Mine
   blockchain.mine();

   let tx_data3 = "dados ocultos".to_string();
   blockchain.add_new_transaction(tx_data3);
   blockchain.mine();

   for block in &blockchain.chain {
       println!("{:?}", block);
   }
}