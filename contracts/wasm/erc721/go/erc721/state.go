// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package erc721

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

type MapHashToImmutableAgentID struct {
	objID int32
}

func (m MapHashToImmutableAgentID) GetAgentID(key wasmlib.ScHash) wasmlib.ScImmutableAgentID {
	return wasmlib.NewScImmutableAgentID(m.objID, key.KeyID())
}

type MapAgentIDToImmutableOperators struct {
	objID int32
}

func (m MapAgentIDToImmutableOperators) GetOperators(key wasmlib.ScAgentID) ImmutableOperators {
	subID := wasmlib.GetObjectID(m.objID, key.KeyID(), wasmlib.TYPE_MAP)
	return ImmutableOperators{objID: subID}
}

type MapAgentIDToImmutableUint64 struct {
	objID int32
}

func (m MapAgentIDToImmutableUint64) GetUint64(key wasmlib.ScAgentID) wasmlib.ScImmutableUint64 {
	return wasmlib.NewScImmutableUint64(m.objID, key.KeyID())
}

type ImmutableErc721State struct {
	id int32
}

func (s ImmutableErc721State) ApprovedAccounts() MapHashToImmutableAgentID {
	mapID := wasmlib.GetObjectID(s.id, wasmlib.KeyID(StateApprovedAccounts), wasmlib.TYPE_MAP)
	return MapHashToImmutableAgentID{objID: mapID}
}

func (s ImmutableErc721State) ApprovedOperators() MapAgentIDToImmutableOperators {
	mapID := wasmlib.GetObjectID(s.id, wasmlib.KeyID(StateApprovedOperators), wasmlib.TYPE_MAP)
	return MapAgentIDToImmutableOperators{objID: mapID}
}

func (s ImmutableErc721State) Balances() MapAgentIDToImmutableUint64 {
	mapID := wasmlib.GetObjectID(s.id, wasmlib.KeyID(StateBalances), wasmlib.TYPE_MAP)
	return MapAgentIDToImmutableUint64{objID: mapID}
}

func (s ImmutableErc721State) Name() wasmlib.ScImmutableString {
	return wasmlib.NewScImmutableString(s.id, wasmlib.KeyID(StateName))
}

func (s ImmutableErc721State) Owners() MapHashToImmutableAgentID {
	mapID := wasmlib.GetObjectID(s.id, wasmlib.KeyID(StateOwners), wasmlib.TYPE_MAP)
	return MapHashToImmutableAgentID{objID: mapID}
}

func (s ImmutableErc721State) Symbol() wasmlib.ScImmutableString {
	return wasmlib.NewScImmutableString(s.id, wasmlib.KeyID(StateSymbol))
}

type MapHashToMutableAgentID struct {
	objID int32
}

func (m MapHashToMutableAgentID) Clear() {
	wasmlib.Clear(m.objID)
}

func (m MapHashToMutableAgentID) GetAgentID(key wasmlib.ScHash) wasmlib.ScMutableAgentID {
	return wasmlib.NewScMutableAgentID(m.objID, key.KeyID())
}

type MapAgentIDToMutableOperators struct {
	objID int32
}

func (m MapAgentIDToMutableOperators) Clear() {
	wasmlib.Clear(m.objID)
}

func (m MapAgentIDToMutableOperators) GetOperators(key wasmlib.ScAgentID) MutableOperators {
	subID := wasmlib.GetObjectID(m.objID, key.KeyID(), wasmlib.TYPE_MAP)
	return MutableOperators{objID: subID}
}

type MapAgentIDToMutableUint64 struct {
	objID int32
}

func (m MapAgentIDToMutableUint64) Clear() {
	wasmlib.Clear(m.objID)
}

func (m MapAgentIDToMutableUint64) GetUint64(key wasmlib.ScAgentID) wasmlib.ScMutableUint64 {
	return wasmlib.NewScMutableUint64(m.objID, key.KeyID())
}

type MutableErc721State struct {
	id int32
}

func (s MutableErc721State) AsImmutable() ImmutableErc721State {
	return ImmutableErc721State(s)
}

func (s MutableErc721State) ApprovedAccounts() MapHashToMutableAgentID {
	mapID := wasmlib.GetObjectID(s.id, wasmlib.KeyID(StateApprovedAccounts), wasmlib.TYPE_MAP)
	return MapHashToMutableAgentID{objID: mapID}
}

func (s MutableErc721State) ApprovedOperators() MapAgentIDToMutableOperators {
	mapID := wasmlib.GetObjectID(s.id, wasmlib.KeyID(StateApprovedOperators), wasmlib.TYPE_MAP)
	return MapAgentIDToMutableOperators{objID: mapID}
}

func (s MutableErc721State) Balances() MapAgentIDToMutableUint64 {
	mapID := wasmlib.GetObjectID(s.id, wasmlib.KeyID(StateBalances), wasmlib.TYPE_MAP)
	return MapAgentIDToMutableUint64{objID: mapID}
}

func (s MutableErc721State) Name() wasmlib.ScMutableString {
	return wasmlib.NewScMutableString(s.id, wasmlib.KeyID(StateName))
}

func (s MutableErc721State) Owners() MapHashToMutableAgentID {
	mapID := wasmlib.GetObjectID(s.id, wasmlib.KeyID(StateOwners), wasmlib.TYPE_MAP)
	return MapHashToMutableAgentID{objID: mapID}
}

func (s MutableErc721State) Symbol() wasmlib.ScMutableString {
	return wasmlib.NewScMutableString(s.id, wasmlib.KeyID(StateSymbol))
}
