// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
//////// DO NOT CHANGE THIS FILE! ////////
// Change the json schema instead

#![allow(dead_code)]

use wasmlib::*;

pub const SC_NAME: &str = "example1";
pub const SC_DESCRIPTION: &str = "Example1 ISCP smart contract to the tutorial";
pub const SC_HNAME: ScHname = ScHname(0xffb07aeb);

pub const PARAM_STRING: &str = "paramString";

pub const VAR_STRING: &str = "storedString";

pub const FUNC_STORE_STRING: &str = "storeString";
pub const FUNC_WITHDRAW_IOTA: &str = "withdrawIota";
pub const VIEW_GET_STRING: &str = "getString";

pub const HFUNC_STORE_STRING: ScHname = ScHname(0x711eafc1);
pub const HFUNC_WITHDRAW_IOTA: ScHname = ScHname(0xd8f57bf6);
pub const HVIEW_GET_STRING: ScHname = ScHname(0xe0b209f9);
