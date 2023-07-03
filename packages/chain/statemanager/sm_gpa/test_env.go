package sm_gpa

import (
	"context"
	"math/rand"
	"testing"
	"time"

	"github.com/stretchr/testify/require"

	"github.com/iotaledger/hive.go/kvstore/mapdb"
	"github.com/iotaledger/hive.go/logger"
	"github.com/iotaledger/wasp/packages/chain/statemanager/sm_gpa/sm_gpa_utils"
	"github.com/iotaledger/wasp/packages/chain/statemanager/sm_gpa/sm_inputs"
	"github.com/iotaledger/wasp/packages/chain/statemanager/sm_snapshots"
	"github.com/iotaledger/wasp/packages/chain/statemanager/sm_utils"
	"github.com/iotaledger/wasp/packages/gpa"
	"github.com/iotaledger/wasp/packages/isc"
	"github.com/iotaledger/wasp/packages/kv/codec"
	"github.com/iotaledger/wasp/packages/kv/dict"
	"github.com/iotaledger/wasp/packages/metrics"
	"github.com/iotaledger/wasp/packages/origin"
	"github.com/iotaledger/wasp/packages/state"
	"github.com/iotaledger/wasp/packages/testutil/testlogger"
	"github.com/iotaledger/wasp/packages/util"
)

type testEnv struct {
	t            *testing.T
	bf           *sm_gpa_utils.BlockFactory
	nodeIDs      []gpa.NodeID
	timeProvider sm_gpa_utils.TimeProvider
	sms          map[gpa.NodeID]gpa.GPA
	stores       map[gpa.NodeID]state.Store
	snapms       map[gpa.NodeID]sm_snapshots.SnapshotManagerTest
	snaprchs     map[gpa.NodeID]<-chan error
	snaprsis     map[gpa.NodeID]sm_snapshots.SnapshotInfo
	tc           *gpa.TestContext
	log          *logger.Logger
}

func newTestEnv(
	t *testing.T,
	nodeIDs []gpa.NodeID,
	createWALFun func() sm_gpa_utils.TestBlockWAL,
	createSnapMFun func(origStore, nodeStore state.Store, tp sm_gpa_utils.TimeProvider, log *logger.Logger) sm_snapshots.SnapshotManagerTest,
	parametersOpt ...StateManagerParameters,
) *testEnv {
	createWALVariedFun := func(gpa.NodeID) sm_gpa_utils.TestBlockWAL {
		return createWALFun()
	}
	createSnapMVariedFun := func(nodeID gpa.NodeID, origStore, nodeStore state.Store, tp sm_gpa_utils.TimeProvider, log *logger.Logger) sm_snapshots.SnapshotManagerTest {
		return createSnapMFun(origStore, nodeStore, tp, log)
	}
	return newVariedTestEnv(t, nodeIDs, createWALVariedFun, createSnapMVariedFun, parametersOpt...)
}

func newVariedTestEnv(
	t *testing.T,
	nodeIDs []gpa.NodeID,
	createWALFun func(gpa.NodeID) sm_gpa_utils.TestBlockWAL,
	createSnapMFun func(nodeID gpa.NodeID, origStore, nodeStore state.Store, tp sm_gpa_utils.TimeProvider, log *logger.Logger) sm_snapshots.SnapshotManagerTest,
	parametersOpt ...StateManagerParameters,
) *testEnv {
	var bf *sm_gpa_utils.BlockFactory
	sms := make(map[gpa.NodeID]gpa.GPA)
	stores := make(map[gpa.NodeID]state.Store)
	snapms := make(map[gpa.NodeID]sm_snapshots.SnapshotManagerTest)
	snaprchs := make(map[gpa.NodeID]<-chan error)
	snaprsis := make(map[gpa.NodeID]sm_snapshots.SnapshotInfo)
	var parameters StateManagerParameters
	var chainInitParameters dict.Dict
	if len(parametersOpt) > 0 {
		parameters = parametersOpt[0]
		chainInitParameters = dict.New()
		chainInitParameters.Set(origin.ParamBlockKeepAmount, codec.EncodeInt32(int32(parameters.PruningMinStatesToKeep)))
	} else {
		parameters = NewStateManagerParameters()
		chainInitParameters = nil
	}

	bf = sm_gpa_utils.NewBlockFactory(t, chainInitParameters)
	chainID := bf.GetChainID()
	log := testlogger.NewLogger(t)
	parameters.TimeProvider = sm_gpa_utils.NewArtifficialTimeProvider()
	for _, nodeID := range nodeIDs {
		var err error
		smLog := log.Named(nodeID.ShortString())
		nr := sm_utils.NewNodeRandomiser(nodeID, nodeIDs, smLog)
		wal := createWALFun(nodeID)
		store := state.NewStore(mapdb.NewMapDB())
		snapshotManager := createSnapMFun(nodeID, bf.GetStore(), store, parameters.TimeProvider, smLog)
		snapshotExistsFun := snapshotManager.SnapshotExists
		origin.InitChain(store, chainInitParameters, 0)
		stores[nodeID] = store
		sms[nodeID], err = New(chainID, nr, wal, snapshotExistsFun, store, mockStateManagerMetrics(), smLog, parameters)
		require.NoError(t, err)
		snapms[nodeID] = snapshotManager
		snaprchs[nodeID] = nil
		snaprsis[nodeID] = nil
	}
	result := &testEnv{
		t:            t,
		bf:           bf,
		nodeIDs:      nodeIDs,
		timeProvider: parameters.TimeProvider,
		sms:          sms,
		snapms:       snapms,
		snaprchs:     snaprchs,
		snaprsis:     snaprsis,
		stores:       stores,
		log:          log,
	}
	result.tc = gpa.NewTestContext(sms).WithOutputHandler(func(nodeID gpa.NodeID, outputOrig gpa.Output) {
		output, ok := outputOrig.(StateManagerOutput)
		require.True(result.t, ok)
		result.checkSnapshotsLoaded()
		snapshotManager, ok := result.snapms[nodeID]
		require.True(result.t, ok)
		snapshotRespChannel, ok := result.snaprchs[nodeID]
		require.True(result.t, ok)
		if snapshotRespChannel == nil {
			snapshotInfo := output.TakeSnapshotToLoad()
			if snapshotInfo != nil {
				result.snaprchs[nodeID] = snapshotManager.LoadSnapshotAsync(snapshotInfo)
				result.snaprsis[nodeID] = snapshotInfo
			}
		}
		for _, snapshotInfo := range output.TakeBlocksCommitted() {
			snapshotManager.BlockCommittedAsync(snapshotInfo)
		}
		if output.TakeUpdateSnapshots() {
			snapshotManager.UpdateAsync()
		}
	})
	return result
}

func (teT *testEnv) finalize() {
	_ = teT.log.Sync()
}

func (teT *testEnv) checkBlock(nodeID gpa.NodeID, origBlock state.Block) {
	store, ok := teT.stores[nodeID]
	require.True(teT.t, ok)
	sm_gpa_utils.CheckBlockInStore(teT.t, store, origBlock)
}

func (teT *testEnv) doesNotContainBlock(nodeID gpa.NodeID, block state.Block) {
	store, ok := teT.stores[nodeID]
	require.True(teT.t, ok)
	require.False(teT.t, store.HasTrieRoot(block.TrieRoot()))
}

func (teT *testEnv) checkSnapshotsLoaded() {
	inputs := make(map[gpa.NodeID]gpa.Input)
	for nodeID, ch := range teT.snaprchs {
		select {
		case result, ok := <-ch:
			if ok {
				snapshotInfo, ok := teT.snaprsis[nodeID]
				require.True(teT.t, ok)
				input := sm_inputs.NewSnapshotManagerSnapshotDone(snapshotInfo.GetStateIndex(), snapshotInfo.GetCommitment(), result)
				inputs[nodeID] = input
			}
			teT.snaprchs[nodeID] = nil
		default:
		}
	}
	teT.tc.WithInputs(inputs).RunAll()
}

func (teT *testEnv) sendBlocksToNode(nodeID gpa.NodeID, timeStep time.Duration, blocks ...state.Block) {
	// If `ConsensusBlockProduced` is sent to the node, the node has definitely obtained all the blocks
	// needed to commit this block. This is ensured by consensus.
	require.True(teT.t, teT.sendAndEnsureCompletedConsensusStateProposal(blocks[0].PreviousL1Commitment(), nodeID, 100, timeStep))
	for i := range blocks {
		teT.t.Logf("Supplying block %s to node %s", blocks[i].L1Commitment(), nodeID.ShortString())
		teT.sendAndEnsureCompletedConsensusBlockProduced(blocks[i], nodeID, 100, timeStep)
	}

	store, ok := teT.stores[nodeID]
	require.True(teT.t, ok)
	err := store.SetLatest(blocks[len(blocks)-1].TrieRoot())
	require.NoError(teT.t, err)
}

func (teT *testEnv) sendBlocksToRandomNode(nodeIDs []gpa.NodeID, timeStep time.Duration, blocks ...state.Block) {
	for _, block := range blocks {
		teT.sendBlocksToNode(nodeIDs[rand.Intn(len(nodeIDs))], timeStep, block)
	}
}

// --------

func (teT *testEnv) sendAndEnsureCompletedConsensusBlockProduced(block state.Block, nodeID gpa.NodeID, maxTimeIterations int, timeStep time.Duration) bool {
	responseCh := teT.sendConsensusBlockProduced(block, nodeID)
	return teT.ensureCompletedConsensusBlockProduced(responseCh, maxTimeIterations, timeStep)
}

func (teT *testEnv) sendConsensusBlockProduced(block state.Block, nodeID gpa.NodeID) <-chan state.Block {
	input, responseCh := sm_inputs.NewConsensusBlockProduced(context.Background(), teT.bf.GetStateDraft(block))
	teT.tc.WithInputs(map[gpa.NodeID]gpa.Input{nodeID: input}).RunAll()
	return responseCh
}

func (teT *testEnv) ensureCompletedConsensusBlockProduced(respChan <-chan state.Block, maxTimeIterations int, timeStep time.Duration) bool {
	return teT.ensureTrue("response from ConsensusBlockProduced", func() bool {
		select {
		case block := <-respChan:
			require.NotNil(teT.t, block)
			return true
		default:
			return false
		}
	}, maxTimeIterations, timeStep)
}

// --------

func (teT *testEnv) sendAndEnsureCompletedConsensusStateProposal(commitment *state.L1Commitment, nodeID gpa.NodeID, maxTimeIterations int, timeStep time.Duration) bool {
	responseCh := teT.sendConsensusStateProposal(commitment, nodeID)
	return teT.ensureCompletedConsensusStateProposal(responseCh, maxTimeIterations, timeStep)
}

func (teT *testEnv) sendConsensusStateProposal(commitment *state.L1Commitment, nodeID gpa.NodeID) <-chan interface{} {
	input, responseCh := sm_inputs.NewConsensusStateProposal(context.Background(), teT.bf.GetAliasOutput(commitment))
	teT.tc.WithInputs(map[gpa.NodeID]gpa.Input{nodeID: input}).RunAll()
	return responseCh
}

func (teT *testEnv) ensureCompletedConsensusStateProposal(respChan <-chan interface{}, maxTimeIterations int, timeStep time.Duration) bool {
	return teT.ensureTrue("response from ConsensusStateProposal", func() bool {
		select {
		case result := <-respChan:
			require.Nil(teT.t, result)
			return true
		default:
			return false
		}
	}, maxTimeIterations, timeStep)
}

// --------

func (teT *testEnv) sendAndEnsureCompletedConsensusDecidedState(commitment *state.L1Commitment, nodeID gpa.NodeID, maxTimeIterations int, timeStep time.Duration) bool {
	responseCh := teT.sendConsensusDecidedState(commitment, nodeID)
	return teT.ensureCompletedConsensusDecidedState(responseCh, commitment, maxTimeIterations, timeStep)
}

func (teT *testEnv) sendConsensusDecidedState(commitment *state.L1Commitment, nodeID gpa.NodeID) <-chan state.State {
	input, responseCh := sm_inputs.NewConsensusDecidedState(context.Background(), teT.bf.GetAliasOutput(commitment))
	teT.tc.WithInputs(map[gpa.NodeID]gpa.Input{nodeID: input}).RunAll()
	return responseCh
}

func (teT *testEnv) ensureCompletedConsensusDecidedState(respChan <-chan state.State, expectedCommitment *state.L1Commitment, maxTimeIterations int, timeStep time.Duration) bool {
	return teT.ensureTrue("response from ConsensusDecidedState", func() bool {
		select {
		case s := <-respChan:
			sm_gpa_utils.CheckStateInStore(teT.t, teT.bf.GetStore(), s)
			return true
		default:
			return false
		}
	}, maxTimeIterations, timeStep)
}

// --------

func (teT *testEnv) sendAndEnsureCompletedChainFetchStateDiff(oldCommitment, newCommitment *state.L1Commitment, expectedOldBlocks, expectedNewBlocks []state.Block, nodeID gpa.NodeID, maxTimeIterations int, timeStep time.Duration) bool {
	responseCh := teT.sendChainFetchStateDiff(oldCommitment, newCommitment, nodeID)
	return teT.ensureCompletedChainFetchStateDiff(responseCh, expectedOldBlocks, expectedNewBlocks, maxTimeIterations, timeStep)
}

func (teT *testEnv) sendChainFetchStateDiff(oldCommitment, newCommitment *state.L1Commitment, nodeID gpa.NodeID) <-chan *sm_inputs.ChainFetchStateDiffResults {
	input, responseCh := sm_inputs.NewChainFetchStateDiff(context.Background(), teT.bf.GetAliasOutput(oldCommitment), teT.bf.GetAliasOutput(newCommitment))
	teT.tc.WithInputs(map[gpa.NodeID]gpa.Input{nodeID: input}).RunAll()
	return responseCh
}

func (teT *testEnv) ensureCompletedChainFetchStateDiff(respChan <-chan *sm_inputs.ChainFetchStateDiffResults, expectedOldBlocks, expectedNewBlocks []state.Block, maxTimeIterations int, timeStep time.Duration) bool {
	return teT.ensureTrue("response from ChainFetchStateDiff", func() bool {
		select {
		case cfsdr := <-respChan:
			newStateTrieRoot := cfsdr.GetNewState().TrieRoot()
			lastNewBlockTrieRoot := expectedNewBlocks[len(expectedNewBlocks)-1].TrieRoot()
			teT.t.Logf("Checking trie roots: expected %s, obtained %s", lastNewBlockTrieRoot, newStateTrieRoot)
			require.True(teT.t, newStateTrieRoot.Equals(lastNewBlockTrieRoot))
			sm_gpa_utils.CheckStateInStore(teT.t, teT.bf.GetStore(), cfsdr.GetNewState())
			requireEqualsFun := func(expected, received []state.Block) {
				teT.t.Logf("\tExpected %v elements, obtained %v elements", len(expected), len(received))
				require.Equal(teT.t, len(expected), len(received))
				for i := range expected {
					teT.t.Logf("\tchecking %v-th element: expected %s, received %s", i, expected[i].L1Commitment(), received[i].L1Commitment())
					sm_gpa_utils.CheckBlocksEqual(teT.t, expected[i], received[i])
				}
			}
			teT.t.Log("Checking added blocks...")
			requireEqualsFun(expectedNewBlocks, cfsdr.GetAdded())
			teT.t.Log("Checking removed blocks...")
			requireEqualsFun(expectedOldBlocks, cfsdr.GetRemoved())
			return true
		default:
			return false
		}
	}, maxTimeIterations, timeStep)
}

// --------

func (teT *testEnv) ensureStoreContainsBlocksNoWait(nodeID gpa.NodeID, blocks []state.Block) bool {
	return teT.ensureTrue("store to contain blocks", func() bool {
		for _, block := range blocks {
			commitment := block.L1Commitment()
			teT.t.Logf("Checking block %s on node %s...", commitment, nodeID.ShortString())
			store, ok := teT.stores[nodeID]
			require.True(teT.t, ok)
			if store.HasTrieRoot(commitment.TrieRoot()) {
				teT.t.Logf("Node %s contains block %s", nodeID.ShortString(), commitment)
			} else {
				teT.t.Logf("Node %s does not contain block %s", nodeID.ShortString(), commitment)
				return false
			}
		}
		return true
	}, 1, 0*time.Second)
}

// --------

func (teT *testEnv) ensureTrue(title string, predicate func() bool, maxTimeIterations int, timeStep time.Duration) bool {
	if predicate() {
		return true
	}
	for i := 1; i < maxTimeIterations; i++ {
		teT.t.Logf("Waiting for %s iteration %v", title, i)
		teT.sendTimerTickToNodes(timeStep)
		if predicate() {
			return true
		}
	}
	return false
}

func (teT *testEnv) sendTimerTickToNodes(delay time.Duration) {
	now := teT.timeProvider.GetNow().Add(delay)
	teT.timeProvider.SetNow(now)
	teT.t.Logf("Time %v is sent to nodes %s", now, util.SliceShortString(teT.nodeIDs))
	teT.sendInputToNodes(func(_ gpa.NodeID) gpa.Input {
		return sm_inputs.NewStateManagerTimerTick(now)
	})
}

func (teT *testEnv) sendInputToNodes(makeInputFun func(gpa.NodeID) gpa.Input) {
	inputs := make(map[gpa.NodeID]gpa.Input)
	for _, nodeID := range teT.nodeIDs {
		inputs[nodeID] = makeInputFun(nodeID)
	}
	teT.tc.WithInputs(inputs).RunAll()
}

func mockStateManagerMetrics() *metrics.ChainStateManagerMetrics {
	return metrics.NewChainMetricsProvider().GetChainMetrics(isc.EmptyChainID()).StateManager
}
