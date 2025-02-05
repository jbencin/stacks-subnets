use crate::burnchains::{Burnchain, StacksSubnetOp, StacksSubnetOpType};
use crate::chainstate::burn::db::sortdb::SortitionHandleTx;
use crate::chainstate::burn::operations::Error as op_error;
use crate::chainstate::burn::operations::WithdrawNftOp;
use clarity::types::chainstate::BurnchainHeaderHash;
use std::convert::TryFrom;

impl TryFrom<&StacksSubnetOp> for WithdrawNftOp {
    type Error = op_error;

    fn try_from(value: &StacksSubnetOp) -> Result<Self, Self::Error> {
        if let StacksSubnetOpType::WithdrawNft {
            ref l1_contract_id,
            ref id,
            ref recipient,
        } = value.event
        {
            Ok(WithdrawNftOp {
                txid: value.txid.clone(),
                // use the StacksBlockId in the L1 event as the burnchain header hash
                burn_header_hash: BurnchainHeaderHash(value.in_block.0.clone()),
                l1_contract_id: l1_contract_id.clone(),
                id: id.clone(),
                recipient: recipient.clone(),
            })
        } else {
            Err(op_error::InvalidInput)
        }
    }
}

impl WithdrawNftOp {
    pub fn check(
        &self,
        _burnchain: &Burnchain,
        _tx: &mut SortitionHandleTx,
    ) -> Result<(), op_error> {
        // good to go!
        Ok(())
    }

    #[cfg(test)]
    pub fn set_burn_height(&mut self, _height: u64) {}
}
