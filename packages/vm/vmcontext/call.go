package vmcontext

import (
	"fmt"

	"github.com/iotaledger/wasp/packages/coretypes"
	"github.com/iotaledger/wasp/packages/kv/codec"
	"github.com/iotaledger/wasp/packages/vm/builtinvm/root"
)

// CallContract
func (vmctx *VMContext) CallContract(contractIndex uint16, epCode coretypes.Hname, params codec.ImmutableCodec, budget coretypes.ColoredBalancesSpendable) (codec.ImmutableCodec, error) {
	vmctx.log.Debugw("Call", "contractIndex", contractIndex, "epCode", epCode.String())

	rec, ok := root.FindContractByIndex(contractIndex, vmctx.callRoot)
	if !ok {
		return nil, fmt.Errorf("failed to find contract with index %d", contractIndex)
	}

	proc, err := vmctx.processors.GetOrCreateProcessor(rec, vmctx.getBinary)
	if err != nil {
		return nil, err
	}

	ep, ok := proc.GetEntryPoint(epCode)
	if !ok {
		return nil, fmt.Errorf("can't find entry point for entry point '%s'", epCode.String())
	}

	if err := vmctx.PushCallContext(contractIndex, params, budget); err != nil {
		return nil, err
	}
	defer vmctx.PopCallContext()

	// distinguishing between two types of entry points. Passing different types of sandboxes
	if ep.IsView() {
		return ep.CallView(NewSandboxView(vmctx))
	}
	return ep.Call(NewSandbox(vmctx))
}

// CallContract
func (vmctx *VMContext) CallView(contractIndex uint16, epCode coretypes.Hname, params codec.ImmutableCodec) (codec.ImmutableCodec, error) {
	vmctx.log.Debugw("CallView", "contractIndex", contractIndex, "epCode", epCode.String())

	rec, ok := root.FindContractByIndex(contractIndex, vmctx.callRoot)
	if !ok {
		return nil, fmt.Errorf("failed to find contract with index %d", contractIndex)
	}

	proc, err := vmctx.processors.GetOrCreateProcessor(rec, vmctx.getBinary)
	if err != nil {
		return nil, err
	}

	ep, ok := proc.GetEntryPoint(epCode)
	if !ok {
		return nil, fmt.Errorf("can't find entry point for entry point '%s'", epCode.String())
	}

	if err := vmctx.PushCallContext(contractIndex, params, nil); err != nil {
		return nil, err
	}
	defer vmctx.PopCallContext()

	if !ep.IsView() {
		return nil, fmt.Errorf("only view entry point can be called in this context")
	}
	return ep.CallView(NewSandboxView(vmctx))
}

func (vmctx *VMContext) callFromRequest() {
	req := vmctx.reqRef.RequestSection()
	vmctx.log.Debugf("callFromRequest: %s -- %s\n", vmctx.reqRef.RequestID().String(), req.String())

	_, err := vmctx.CallContract(req.Target().Index(), req.EntryPointCode(), req.Args(), nil)
	if err != nil {
		vmctx.log.Warnf("callFromRequest: %v", err)
	}
}

func (vmctx *VMContext) Params() codec.ImmutableCodec {
	return vmctx.getCallContext().params
}
