package wasptest

import (
	"fmt"
	"github.com/iotaledger/goshimmer/dapps/valuetransfers/packages/address"
	nodeapi "github.com/iotaledger/goshimmer/packages/waspconn/apilib"
	waspapi "github.com/iotaledger/wasp/packages/apilib"
	"github.com/iotaledger/wasp/tools/cluster"
)

const goshimmer = "127.0.0.1:8080"

func CreateOrigin1SC(clu *cluster.Cluster) error {
	fmt.Printf("------------------------------   Test 3: create origin of 1 SC\n")

	// selecting first committee node to post the batch
	waspNodeNetAddr := clu.Config.Nodes[clu.SmartContractConfig[0].CommitteeNodes[0]].NetAddress
	waspNodeApiPort := clu.Config.Nodes[clu.SmartContractConfig[0].CommitteeNodes[0]].ApiPort
	waspNodeUrl := fmt.Sprintf("%s:%d", waspNodeNetAddr, waspNodeApiPort)

	sc := clu.SmartContractConfig[0]
	tx, batch, err := cluster.CreateOriginData(goshimmer, &sc)
	if err != nil {
		return err
	}
	err = nodeapi.PostTransaction(goshimmer, tx.Transaction)
	if err != nil {
		return err
	}
	fmt.Printf("[cluster] posted node origin tx to Goshimmer: addr: %s, txid: %s\n", sc.Address, tx.ID().String())

	addr, err := address.FromBase58(sc.Address)
	if err != nil {
		return err
	}

	err = waspapi.PostOriginBatch(waspNodeUrl, &addr, batch)
	if err != nil {
		return err
	}
	fmt.Printf("[cluster] posted origin batch to Wasp node %s\n", waspNodeUrl)
	return nil
}
