// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use wasmlib::*;
use wasmlib::host::*;

use crate::*;
use crate::keys::*;
use crate::structs::*;

#[derive(Clone, Copy)]
pub struct ImmutableDonationResults {
    pub(crate) id: i32,
}

impl ImmutableDonationResults {
    pub fn amount(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.id, RESULT_AMOUNT.get_key_id())
	}

    pub fn donator(&self) -> ScImmutableAgentID {
		ScImmutableAgentID::new(self.id, RESULT_DONATOR.get_key_id())
	}

    pub fn error(&self) -> ScImmutableString {
		ScImmutableString::new(self.id, RESULT_ERROR.get_key_id())
	}

    pub fn feedback(&self) -> ScImmutableString {
		ScImmutableString::new(self.id, RESULT_FEEDBACK.get_key_id())
	}

    pub fn timestamp(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.id, RESULT_TIMESTAMP.get_key_id())
	}
}

#[derive(Clone, Copy)]
pub struct MutableDonationResults {
    pub(crate) id: i32,
}

impl MutableDonationResults {
    pub fn amount(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.id, RESULT_AMOUNT.get_key_id())
	}

    pub fn donator(&self) -> ScMutableAgentID {
		ScMutableAgentID::new(self.id, RESULT_DONATOR.get_key_id())
	}

    pub fn error(&self) -> ScMutableString {
		ScMutableString::new(self.id, RESULT_ERROR.get_key_id())
	}

    pub fn feedback(&self) -> ScMutableString {
		ScMutableString::new(self.id, RESULT_FEEDBACK.get_key_id())
	}

    pub fn timestamp(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.id, RESULT_TIMESTAMP.get_key_id())
	}
}

#[derive(Clone, Copy)]
pub struct ImmutableDonationInfoResults {
    pub(crate) id: i32,
}

impl ImmutableDonationInfoResults {
    pub fn count(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.id, RESULT_COUNT.get_key_id())
	}

    pub fn max_donation(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.id, RESULT_MAX_DONATION.get_key_id())
	}

    pub fn total_donation(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.id, RESULT_TOTAL_DONATION.get_key_id())
	}
}

#[derive(Clone, Copy)]
pub struct MutableDonationInfoResults {
    pub(crate) id: i32,
}

impl MutableDonationInfoResults {
    pub fn count(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.id, RESULT_COUNT.get_key_id())
	}

    pub fn max_donation(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.id, RESULT_MAX_DONATION.get_key_id())
	}

    pub fn total_donation(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.id, RESULT_TOTAL_DONATION.get_key_id())
	}
}
