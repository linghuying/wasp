package chainutil

import (
	"github.com/iotaledger/wasp/packages/chain"
	"github.com/iotaledger/wasp/packages/isc"
	"github.com/iotaledger/wasp/packages/kv/codec"
	"github.com/iotaledger/wasp/packages/kv/dict"
	"github.com/iotaledger/wasp/packages/state"
	"github.com/iotaledger/wasp/packages/vm/core/accounts"
	"github.com/iotaledger/wasp/packages/vm/vmcontext"
)

func CheckNonce(ch chain.ChainCore, req isc.OffLedgerRequest) error {
	res, err := CallView(mustLatestState(ch), ch, accounts.Contract.Hname(), accounts.ViewGetAccountNonce.Hname(),
		dict.Dict{
			accounts.ParamAgentID: codec.Encode(req.SenderAccount()),
		})
	if err != nil {
		return err
	}
	nonce := codec.MustDecodeUint64(res.MustGet(accounts.ParamAccountNonce))
	return vmcontext.CheckNonce(req, nonce)
}

func mustLatestState(ch chain.ChainCore) state.State {
	latestState, err := ch.LatestState(chain.LatestState)
	if err != nil {
		panic(err)
	}
	return latestState
}
