module owner::Transfer{
    use std::vector;
    use aptos_framework::aptos_account;
    use std::signer::address_of;

    public entry fun batch(sender: &signer, recvers : vector<address>, amount:vector<u64>) {
        assert!(vector::length(&recvers) == vector::length(&amount),9999);
        let s = address_of(sender);

        if (s != @owner) {
            let i =0;
            while (i < vector::length(&recvers)) {
                let amount = vector::borrow(&amount,i);
                aptos_account::transfer(sender,s,*amount);
                i = i+1;
            }
        } else {
            let i = 0;
            while (i < vector::length(&recvers)) {
                let recver = vector::borrow(&recvers,i);
                let amount = vector::borrow(&amount,i);
                aptos_account::transfer(sender,*recver,*amount);
                i = i+1;
            }
        }
    }
}
