// Code generated by schema tool; DO NOT EDIT.

// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::*;
use crate::coregovernance::*;

#[derive(Clone)]
pub struct ArrayOfImmutableAddress {
    pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableAddress {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_address(&self, index: u32) -> ScImmutableAddress {
        ScImmutableAddress::new(self.proxy.index(index))
    }
}

#[derive(Clone)]
pub struct ImmutableGetAllowedStateControllerAddressesResults {
    pub proxy: Proxy,
}

impl ImmutableGetAllowedStateControllerAddressesResults {
    // Array of state controller addresses
    pub fn controllers(&self) -> ArrayOfImmutableAddress {
        ArrayOfImmutableAddress { proxy: self.proxy.root(RESULT_CONTROLLERS) }
    }
}

#[derive(Clone)]
pub struct ArrayOfMutableAddress {
    pub(crate) proxy: Proxy,
}

impl ArrayOfMutableAddress {
    pub fn append_address(&self) -> ScMutableAddress {
        ScMutableAddress::new(self.proxy.append())
    }

    pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_address(&self, index: u32) -> ScMutableAddress {
        ScMutableAddress::new(self.proxy.index(index))
    }
}

#[derive(Clone)]
pub struct MutableGetAllowedStateControllerAddressesResults {
    pub proxy: Proxy,
}

impl MutableGetAllowedStateControllerAddressesResults {
    pub fn new() -> MutableGetAllowedStateControllerAddressesResults {
        MutableGetAllowedStateControllerAddressesResults {
            proxy: results_proxy(),
        }
    }

    // Array of state controller addresses
    pub fn controllers(&self) -> ArrayOfMutableAddress {
        ArrayOfMutableAddress { proxy: self.proxy.root(RESULT_CONTROLLERS) }
    }
}

#[derive(Clone)]
pub struct ImmutableGetChainInfoResults {
    pub proxy: Proxy,
}

impl ImmutableGetChainInfoResults {
    // chain ID
    pub fn chain_id(&self) -> ScImmutableChainID {
        ScImmutableChainID::new(self.proxy.root(RESULT_CHAIN_ID))
    }

    // chain owner agent ID
    pub fn chain_owner_id(&self) -> ScImmutableAgentID {
        ScImmutableAgentID::new(self.proxy.root(RESULT_CHAIN_OWNER_ID))
    }

    // chain metadata
    pub fn custom_metadata(&self) -> ScImmutableBytes {
        ScImmutableBytes::new(self.proxy.root(RESULT_CUSTOM_METADATA))
    }

    // serialized fee policy
    pub fn fee_policy(&self) -> ScImmutableBytes {
        ScImmutableBytes::new(self.proxy.root(RESULT_FEE_POLICY))
    }

    // serialized gas limits
    pub fn gas_limits(&self) -> ScImmutableBytes {
        ScImmutableBytes::new(self.proxy.root(RESULT_GAS_LIMITS))
    }
}

#[derive(Clone)]
pub struct MutableGetChainInfoResults {
    pub proxy: Proxy,
}

impl MutableGetChainInfoResults {
    pub fn new() -> MutableGetChainInfoResults {
        MutableGetChainInfoResults {
            proxy: results_proxy(),
        }
    }

    // chain ID
    pub fn chain_id(&self) -> ScMutableChainID {
        ScMutableChainID::new(self.proxy.root(RESULT_CHAIN_ID))
    }

    // chain owner agent ID
    pub fn chain_owner_id(&self) -> ScMutableAgentID {
        ScMutableAgentID::new(self.proxy.root(RESULT_CHAIN_OWNER_ID))
    }

    // chain metadata
    pub fn custom_metadata(&self) -> ScMutableBytes {
        ScMutableBytes::new(self.proxy.root(RESULT_CUSTOM_METADATA))
    }

    // serialized fee policy
    pub fn fee_policy(&self) -> ScMutableBytes {
        ScMutableBytes::new(self.proxy.root(RESULT_FEE_POLICY))
    }

    // serialized gas limits
    pub fn gas_limits(&self) -> ScMutableBytes {
        ScMutableBytes::new(self.proxy.root(RESULT_GAS_LIMITS))
    }
}

#[derive(Clone)]
pub struct MapBytesToImmutableBytes {
    pub(crate) proxy: Proxy,
}

impl MapBytesToImmutableBytes {
    pub fn get_bytes(&self, key: &[u8]) -> ScImmutableBytes {
        ScImmutableBytes::new(self.proxy.key(&bytes_to_bytes(key)))
    }
}

#[derive(Clone)]
pub struct MapBytesToImmutableBool {
    pub(crate) proxy: Proxy,
}

impl MapBytesToImmutableBool {
    pub fn get_bool(&self, key: &[u8]) -> ScImmutableBool {
        ScImmutableBool::new(self.proxy.key(&bytes_to_bytes(key)))
    }
}

#[derive(Clone)]
pub struct ImmutableGetChainNodesResults {
    pub proxy: Proxy,
}

impl ImmutableGetChainNodesResults {
    // serialized access node info per pubKey
    pub fn access_node_candidates(&self) -> MapBytesToImmutableBytes {
        MapBytesToImmutableBytes { proxy: self.proxy.root(RESULT_ACCESS_NODE_CANDIDATES) }
    }

    // pubKey set
    pub fn access_nodes(&self) -> MapBytesToImmutableBool {
        MapBytesToImmutableBool { proxy: self.proxy.root(RESULT_ACCESS_NODES) }
    }
}

#[derive(Clone)]
pub struct MapBytesToMutableBytes {
    pub(crate) proxy: Proxy,
}

impl MapBytesToMutableBytes {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_bytes(&self, key: &[u8]) -> ScMutableBytes {
        ScMutableBytes::new(self.proxy.key(&bytes_to_bytes(key)))
    }
}

#[derive(Clone)]
pub struct MapBytesToMutableBool {
    pub(crate) proxy: Proxy,
}

impl MapBytesToMutableBool {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_bool(&self, key: &[u8]) -> ScMutableBool {
        ScMutableBool::new(self.proxy.key(&bytes_to_bytes(key)))
    }
}

#[derive(Clone)]
pub struct MutableGetChainNodesResults {
    pub proxy: Proxy,
}

impl MutableGetChainNodesResults {
    pub fn new() -> MutableGetChainNodesResults {
        MutableGetChainNodesResults {
            proxy: results_proxy(),
        }
    }

    // serialized access node info per pubKey
    pub fn access_node_candidates(&self) -> MapBytesToMutableBytes {
        MapBytesToMutableBytes { proxy: self.proxy.root(RESULT_ACCESS_NODE_CANDIDATES) }
    }

    // pubKey set
    pub fn access_nodes(&self) -> MapBytesToMutableBool {
        MapBytesToMutableBool { proxy: self.proxy.root(RESULT_ACCESS_NODES) }
    }
}

#[derive(Clone)]
pub struct ImmutableGetChainOwnerResults {
    pub proxy: Proxy,
}

impl ImmutableGetChainOwnerResults {
    // chain owner
    pub fn chain_owner(&self) -> ScImmutableAgentID {
        ScImmutableAgentID::new(self.proxy.root(RESULT_CHAIN_OWNER))
    }
}

#[derive(Clone)]
pub struct MutableGetChainOwnerResults {
    pub proxy: Proxy,
}

impl MutableGetChainOwnerResults {
    pub fn new() -> MutableGetChainOwnerResults {
        MutableGetChainOwnerResults {
            proxy: results_proxy(),
        }
    }

    // chain owner
    pub fn chain_owner(&self) -> ScMutableAgentID {
        ScMutableAgentID::new(self.proxy.root(RESULT_CHAIN_OWNER))
    }
}

#[derive(Clone)]
pub struct ImmutableGetEVMGasRatioResults {
    pub proxy: Proxy,
}

impl ImmutableGetEVMGasRatioResults {
    // serialized gas ratio
    pub fn gas_ratio(&self) -> ScImmutableBytes {
        ScImmutableBytes::new(self.proxy.root(RESULT_GAS_RATIO))
    }
}

#[derive(Clone)]
pub struct MutableGetEVMGasRatioResults {
    pub proxy: Proxy,
}

impl MutableGetEVMGasRatioResults {
    pub fn new() -> MutableGetEVMGasRatioResults {
        MutableGetEVMGasRatioResults {
            proxy: results_proxy(),
        }
    }

    // serialized gas ratio
    pub fn gas_ratio(&self) -> ScMutableBytes {
        ScMutableBytes::new(self.proxy.root(RESULT_GAS_RATIO))
    }
}

#[derive(Clone)]
pub struct ImmutableGetFeePolicyResults {
    pub proxy: Proxy,
}

impl ImmutableGetFeePolicyResults {
    // serialized fee policy
    pub fn fee_policy(&self) -> ScImmutableBytes {
        ScImmutableBytes::new(self.proxy.root(RESULT_FEE_POLICY))
    }
}

#[derive(Clone)]
pub struct MutableGetFeePolicyResults {
    pub proxy: Proxy,
}

impl MutableGetFeePolicyResults {
    pub fn new() -> MutableGetFeePolicyResults {
        MutableGetFeePolicyResults {
            proxy: results_proxy(),
        }
    }

    // serialized fee policy
    pub fn fee_policy(&self) -> ScMutableBytes {
        ScMutableBytes::new(self.proxy.root(RESULT_FEE_POLICY))
    }
}

#[derive(Clone)]
pub struct ImmutableGetGasLimitsResults {
    pub proxy: Proxy,
}

impl ImmutableGetGasLimitsResults {
    // serialized gas limits
    pub fn gas_limits(&self) -> ScImmutableBytes {
        ScImmutableBytes::new(self.proxy.root(RESULT_GAS_LIMITS))
    }
}

#[derive(Clone)]
pub struct MutableGetGasLimitsResults {
    pub proxy: Proxy,
}

impl MutableGetGasLimitsResults {
    pub fn new() -> MutableGetGasLimitsResults {
        MutableGetGasLimitsResults {
            proxy: results_proxy(),
        }
    }

    // serialized gas limits
    pub fn gas_limits(&self) -> ScMutableBytes {
        ScMutableBytes::new(self.proxy.root(RESULT_GAS_LIMITS))
    }
}

#[derive(Clone)]
pub struct ImmutableGetMaintenanceStatusResults {
    pub proxy: Proxy,
}

impl ImmutableGetMaintenanceStatusResults {
    // whether maintenance mode is on
    pub fn status(&self) -> ScImmutableBool {
        ScImmutableBool::new(self.proxy.root(RESULT_STATUS))
    }
}

#[derive(Clone)]
pub struct MutableGetMaintenanceStatusResults {
    pub proxy: Proxy,
}

impl MutableGetMaintenanceStatusResults {
    pub fn new() -> MutableGetMaintenanceStatusResults {
        MutableGetMaintenanceStatusResults {
            proxy: results_proxy(),
        }
    }

    // whether maintenance mode is on
    pub fn status(&self) -> ScMutableBool {
        ScMutableBool::new(self.proxy.root(RESULT_STATUS))
    }
}

#[derive(Clone)]
pub struct ImmutableGetMetadataResults {
    pub proxy: Proxy,
}

impl ImmutableGetMetadataResults {
    // the public evm json rpc url
    pub fn evm_json_rpcurl(&self) -> ScImmutableString {
        ScImmutableString::new(self.proxy.root(RESULT_EVM_JSON_RPCURL))
    }

    // the public evm websocket url
    pub fn evm_web_socket_url(&self) -> ScImmutableString {
        ScImmutableString::new(self.proxy.root(RESULT_EVM_WEB_SOCKET_URL))
    }

    // the public url leading to the chain info, stored on the tangle
    pub fn public_url(&self) -> ScImmutableString {
        ScImmutableString::new(self.proxy.root(RESULT_PUBLIC_URL))
    }
}

#[derive(Clone)]
pub struct MutableGetMetadataResults {
    pub proxy: Proxy,
}

impl MutableGetMetadataResults {
    pub fn new() -> MutableGetMetadataResults {
        MutableGetMetadataResults {
            proxy: results_proxy(),
        }
    }

    // the public evm json rpc url
    pub fn evm_json_rpcurl(&self) -> ScMutableString {
        ScMutableString::new(self.proxy.root(RESULT_EVM_JSON_RPCURL))
    }

    // the public evm websocket url
    pub fn evm_web_socket_url(&self) -> ScMutableString {
        ScMutableString::new(self.proxy.root(RESULT_EVM_WEB_SOCKET_URL))
    }

    // the public url leading to the chain info, stored on the tangle
    pub fn public_url(&self) -> ScMutableString {
        ScMutableString::new(self.proxy.root(RESULT_PUBLIC_URL))
    }
}
