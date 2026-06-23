use std::collections::HashMap;
use crate::vm::{add_u256,sub_u256};

#[derive(Clone, Default,Debug)]
struct Account { 
    balance : [u8; 32], 
    nonce   :u64,       //tx counter
    code    :Vec<u8>, 
    storage :HashMap<[u8;32], [u8;32]>, 
}

type Address = [u8;20]; 

struct WorldState { 
    accounts    :HashMap<Address ,Account>,    
}

impl WorldState {
    fn new() -> Self{
        WorldState { accounts: HashMap::new(),}
    }

    fn get_account(&self, addr: &Address) -> Account {
        match self.accounts.get(addr) {
            Some(x) => { return x.clone()}
            None => Account::default()
        }
    }

    fn apply_transaction(&mut self, tx: &Transaction) ->Result<(), String> {
        let mut sender      = self.get_account(&tx.from);
        let mut recipient   = self.get_account(&tx.to);

        if sender.nonce != tx.nonce {
            return Err("Sender nonce mismatch.".to_string());
        }

        if sender.balance >= tx.value {
            sender.balance = sub_u256(sender.balance, tx.value);
            recipient.balance = add_u256(recipient.balance, tx.value);
            sender.nonce += 1;

            self.accounts.insert(tx.from, sender);
            self.accounts.insert(tx.to, recipient);
        }
         else { 
            return Err("Sender has Insufficient balance".to_string());
        }
         Ok(())
    }
}

#[derive(Default, Clone, Debug)]
struct Transaction { 
    from:   Address,
    to:     Address,
    value:  [u8;32], 
    nonce:  u64,
    data:   Vec<u8>,
}

impl Transaction {
    fn new(
        from: Address , 
        to: Address , 
        value : [u8;32], 
        nonce: u64, 
        data: Vec<u8>) 
        -> Self {
            Transaction { from, to, value, nonce, data } //probably risky
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // tiny helper: turn a small number into a 256-bit big-endian balance,
    // same trick as your vm.rs tests (low bytes hold the value).
    fn u(n: u64) -> [u8; 32] {
        let mut x = [0u8; 32];
        x[24..32].copy_from_slice(&n.to_be_bytes());
        x
    }

    #[test]
    fn transfer_moves_value_and_bumps_nonce() {
        let mut world = WorldState::new();

        // two addresses — Alice is 0x0101.., Bob is 0x0202..
        let alice = [1u8; 20];
        let bob   = [2u8; 20];

        // fund Alice with 100. `..Default::default()` fills nonce/code/storage.
        world.accounts.insert(alice, Account { balance: u(100), ..Default::default() });

        // Alice sends 30 to Bob. Her current nonce is 0, so the tx says nonce 0.
        let tx = Transaction::new(alice, bob, u(30), 0, vec![]);
        world.apply_transaction(&tx).unwrap();   // unwrap: blow up if it returned Err

        // books balance: 100 - 30 = 70, and Bob (who didn't exist) now has 30.
        assert_eq!(world.get_account(&alice).balance, u(70));
        assert_eq!(world.get_account(&bob).balance,   u(30));

        // nonce bumped 0 -> 1, so the *same* tx can't be replayed.
        assert_eq!(world.get_account(&alice).nonce, 1);
    }

    #[test]
    fn replay_is_rejected() {
        let mut world = WorldState::new();
        let alice = [1u8; 20];
        let bob   = [2u8; 20];
        world.accounts.insert(alice, Account { balance: u(100), ..Default::default() });

        let tx = Transaction::new(alice, bob, u(30), 0, vec![]);

        world.apply_transaction(&tx).unwrap();          // first time: fine
        assert!(world.apply_transaction(&tx).is_err()); // second time: nonce is now 1, tx says 0 -> rejected
    }

    #[test]
    fn broke_sender_is_rejected() {
        let mut world = WorldState::new();
        let alice = [1u8; 20];
        let bob   = [2u8; 20];
        world.accounts.insert(alice, Account { balance: u(10), ..Default::default() });

        // Alice has 10 but tries to send 30 -> insufficient balance.
        let tx = Transaction::new(alice, bob, u(30), 0, vec![]);
        assert!(world.apply_transaction(&tx).is_err());
    }
}





