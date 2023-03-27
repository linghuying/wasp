// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::*;
use crate::coreblocklog::*;

#[derive(Clone)]
pub struct ImmutableGetBlockInfoParams {
    pub(crate) proxy: Proxy,
}

impl ImmutableGetBlockInfoParams {
    pub fn new() -> ImmutableGetBlockInfoParams {
        ImmutableGetBlockInfoParams {
            proxy: params_proxy(),
        }
    }

    // default last block
    pub fn block_index(&self) -> ScImmutableUint32 {
        ScImmutableUint32::new(self.proxy.root(PARAM_BLOCK_INDEX))
    }
}

#[derive(Clone)]
pub struct MutableGetBlockInfoParams {
    pub(crate) proxy: Proxy,
}

impl MutableGetBlockInfoParams {
    // default last block
    pub fn block_index(&self) -> ScMutableUint32 {
        ScMutableUint32::new(self.proxy.root(PARAM_BLOCK_INDEX))
    }
}

#[derive(Clone)]
pub struct ImmutableGetEventsForBlockParams {
    pub(crate) proxy: Proxy,
}

impl ImmutableGetEventsForBlockParams {
    pub fn new() -> ImmutableGetEventsForBlockParams {
        ImmutableGetEventsForBlockParams {
            proxy: params_proxy(),
        }
    }

    // default last block
    pub fn block_index(&self) -> ScImmutableUint32 {
        ScImmutableUint32::new(self.proxy.root(PARAM_BLOCK_INDEX))
    }
}

#[derive(Clone)]
pub struct MutableGetEventsForBlockParams {
    pub(crate) proxy: Proxy,
}

impl MutableGetEventsForBlockParams {
    // default last block
    pub fn block_index(&self) -> ScMutableUint32 {
        ScMutableUint32::new(self.proxy.root(PARAM_BLOCK_INDEX))
    }
}

#[derive(Clone)]
pub struct ImmutableGetEventsForContractParams {
    pub(crate) proxy: Proxy,
}

impl ImmutableGetEventsForContractParams {
    pub fn new() -> ImmutableGetEventsForContractParams {
        ImmutableGetEventsForContractParams {
            proxy: params_proxy(),
        }
    }

    pub fn contract_hname(&self) -> ScImmutableHname {
        ScImmutableHname::new(self.proxy.root(PARAM_CONTRACT_HNAME))
    }

    // default first block
    pub fn from_block(&self) -> ScImmutableUint32 {
        ScImmutableUint32::new(self.proxy.root(PARAM_FROM_BLOCK))
    }

    // default last block
    pub fn to_block(&self) -> ScImmutableUint32 {
        ScImmutableUint32::new(self.proxy.root(PARAM_TO_BLOCK))
    }
}

#[derive(Clone)]
pub struct MutableGetEventsForContractParams {
    pub(crate) proxy: Proxy,
}

impl MutableGetEventsForContractParams {
    pub fn contract_hname(&self) -> ScMutableHname {
        ScMutableHname::new(self.proxy.root(PARAM_CONTRACT_HNAME))
    }

    // default first block
    pub fn from_block(&self) -> ScMutableUint32 {
        ScMutableUint32::new(self.proxy.root(PARAM_FROM_BLOCK))
    }

    // default last block
    pub fn to_block(&self) -> ScMutableUint32 {
        ScMutableUint32::new(self.proxy.root(PARAM_TO_BLOCK))
    }
}

#[derive(Clone)]
pub struct ImmutableGetEventsForRequestParams {
    pub(crate) proxy: Proxy,
}

impl ImmutableGetEventsForRequestParams {
    pub fn new() -> ImmutableGetEventsForRequestParams {
        ImmutableGetEventsForRequestParams {
            proxy: params_proxy(),
        }
    }

    // target request ID
    pub fn request_id(&self) -> ScImmutableRequestID {
        ScImmutableRequestID::new(self.proxy.root(PARAM_REQUEST_ID))
    }
}

#[derive(Clone)]
pub struct MutableGetEventsForRequestParams {
    pub(crate) proxy: Proxy,
}

impl MutableGetEventsForRequestParams {
    // target request ID
    pub fn request_id(&self) -> ScMutableRequestID {
        ScMutableRequestID::new(self.proxy.root(PARAM_REQUEST_ID))
    }
}

#[derive(Clone)]
pub struct ImmutableGetRequestIDsForBlockParams {
    pub(crate) proxy: Proxy,
}

impl ImmutableGetRequestIDsForBlockParams {
    pub fn new() -> ImmutableGetRequestIDsForBlockParams {
        ImmutableGetRequestIDsForBlockParams {
            proxy: params_proxy(),
        }
    }

    // default last block
    pub fn block_index(&self) -> ScImmutableUint32 {
        ScImmutableUint32::new(self.proxy.root(PARAM_BLOCK_INDEX))
    }
}

#[derive(Clone)]
pub struct MutableGetRequestIDsForBlockParams {
    pub(crate) proxy: Proxy,
}

impl MutableGetRequestIDsForBlockParams {
    // default last block
    pub fn block_index(&self) -> ScMutableUint32 {
        ScMutableUint32::new(self.proxy.root(PARAM_BLOCK_INDEX))
    }
}

#[derive(Clone)]
pub struct ImmutableGetRequestReceiptParams {
    pub(crate) proxy: Proxy,
}

impl ImmutableGetRequestReceiptParams {
    pub fn new() -> ImmutableGetRequestReceiptParams {
        ImmutableGetRequestReceiptParams {
            proxy: params_proxy(),
        }
    }

    // target request ID
    pub fn request_id(&self) -> ScImmutableRequestID {
        ScImmutableRequestID::new(self.proxy.root(PARAM_REQUEST_ID))
    }
}

#[derive(Clone)]
pub struct MutableGetRequestReceiptParams {
    pub(crate) proxy: Proxy,
}

impl MutableGetRequestReceiptParams {
    // target request ID
    pub fn request_id(&self) -> ScMutableRequestID {
        ScMutableRequestID::new(self.proxy.root(PARAM_REQUEST_ID))
    }
}

#[derive(Clone)]
pub struct ImmutableGetRequestReceiptsForBlockParams {
    pub(crate) proxy: Proxy,
}

impl ImmutableGetRequestReceiptsForBlockParams {
    pub fn new() -> ImmutableGetRequestReceiptsForBlockParams {
        ImmutableGetRequestReceiptsForBlockParams {
            proxy: params_proxy(),
        }
    }

    // default last block
    pub fn block_index(&self) -> ScImmutableUint32 {
        ScImmutableUint32::new(self.proxy.root(PARAM_BLOCK_INDEX))
    }
}

#[derive(Clone)]
pub struct MutableGetRequestReceiptsForBlockParams {
    pub(crate) proxy: Proxy,
}

impl MutableGetRequestReceiptsForBlockParams {
    // default last block
    pub fn block_index(&self) -> ScMutableUint32 {
        ScMutableUint32::new(self.proxy.root(PARAM_BLOCK_INDEX))
    }
}

#[derive(Clone)]
pub struct ImmutableIsRequestProcessedParams {
    pub(crate) proxy: Proxy,
}

impl ImmutableIsRequestProcessedParams {
    pub fn new() -> ImmutableIsRequestProcessedParams {
        ImmutableIsRequestProcessedParams {
            proxy: params_proxy(),
        }
    }

    // target request ID
    pub fn request_id(&self) -> ScImmutableRequestID {
        ScImmutableRequestID::new(self.proxy.root(PARAM_REQUEST_ID))
    }
}

#[derive(Clone)]
pub struct MutableIsRequestProcessedParams {
    pub(crate) proxy: Proxy,
}

impl MutableIsRequestProcessedParams {
    // target request ID
    pub fn request_id(&self) -> ScMutableRequestID {
        ScMutableRequestID::new(self.proxy.root(PARAM_REQUEST_ID))
    }
}
