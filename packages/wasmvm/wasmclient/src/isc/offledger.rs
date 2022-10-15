// func (c *WaspClient) PostOffLedgerRequest(chainID *isc.ChainID, req isc.OffLedgerRequest) error {
// 	data := model.OffLedgerRequestBody{
// 		Request: model.NewBytes(req.Bytes()),
// 	}
// 	return c.do(http.MethodPost, routes.NewRequest(chainID.String()), data, nil)
// }

use crate::keypair::*;
use crypto::signatures::ed25519;
use wasmlib::*;

//TODO generalize this trait
pub trait OffLedgerRequest {
    fn new(
        chain_id: ScChainID,
        contract: ScHname,
        entry_point: ScHname,
        params: ScDict,
        signature_scheme: Option<OffLedgerSignatureScheme>,
        nonce: u64,
    ) -> Self;
    fn with_nonce(&self, nonce: u64) -> &Self;
    fn with_gas_budget(&self, gas_budget: u64) -> &Self;
    fn with_allowance(&self, allowance: &ScAssets) -> &Self;
    fn sign(&self, key: KeyPair) -> &Self;
}

pub struct OffLedgerRequestData {
    chain_id: ScChainID,
    contract: ScHname,
    entry_point: ScHname,
    params: ScDict,
    signature_scheme: Option<OffLedgerSignatureScheme>, // None if unsigned
    nonce: u64,
    allowance: ScAssets,
    gas_budget: u64,
}

pub struct OffLedgerSignatureScheme {
    public_key: ed25519::PublicKey,
    signature: Vec<u8>,
}

impl OffLedgerRequest for OffLedgerRequestData {
    fn new(
        chain_id: ScChainID,
        contract: ScHname,
        entry_point: ScHname,
        params: ScDict,
        signature_scheme: Option<OffLedgerSignatureScheme>,
        nonce: u64,
    ) -> Self {
        return OffLedgerRequestData {
            chain_id: chain_id,
            contract: contract,
            entry_point: entry_point,
            params: params,
            signature_scheme: signature_scheme,
            nonce: nonce,
            allowance: ScAssets::new(&Vec::new()),
            gas_budget: super::gas::MAX_GAS_PER_REQUEST,
        };
    }
    fn with_nonce(&self, nonce: u64) -> &Self {
        todo!()
    }
    fn with_gas_budget(&self, gas_budget: u64) -> &Self {
        todo!()
    }
    fn with_allowance(&self, allowance: &ScAssets) -> &Self {
        todo!()
    }
    fn sign(&self, key: KeyPair) -> &Self {
        todo!()
    }
}

impl OffLedgerRequestData {
    pub fn id(&self) -> ScRequestID {
        todo!()
    }
}
