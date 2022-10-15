// // // Copyright 2020 IOTA Stiftung
// // // SPDX-License-Identifier: Apache-2.0

// use crate::wasp_client;

use crate::*;
use keypair::*;
use wasmlib::*;

pub trait IEventHandler {
    fn call_handler(&self, topic: &str, params: &[&str]);
}

pub struct WasmClientContext {
    pub chain_id: ScChainID,
    pub err: String,
    pub event_done: bool, // FIXME this should be channel
    pub event_handlers: Vec<Box<dyn IEventHandler>>,
    pub key_pair: KeyPair,
    pub req_id: ScRequestID,
    pub sc_name: String,
    pub sc_hname: ScHname,
    pub svc_client: WasmClientService, //TODO Maybe use trait instead of struct
}

impl WasmClientContext {
    // FIXME use 'dyn IClientService' for 'svc_client'
    pub fn new(
        svc_client: WasmClientService,
        chain_id: &wasmlib::ScChainID,
        sc_name: &str,
    ) -> WasmClientContext {
        WasmClientContext {
            svc_client: svc_client,
            sc_name: sc_name.to_string(),
            sc_hname: ScHname::new(sc_name),
            chain_id: chain_id.clone(),
            err: String::new(),
            event_done: false,
            event_handlers: Vec::new(),
            key_pair: KeyPair::default(),
            req_id: ScRequestID::default(),
        }
    }

    pub fn default() -> WasmClientContext {
        WasmClientContext {
            svc_client: WasmClientService::default(),
            sc_name: String::new(),
            sc_hname: ScHname::default(),
            chain_id: ScChainID::default(),
            err: String::new(),
            event_done: false,
            event_handlers: Vec::new(),
            key_pair: KeyPair::default(),
            req_id: ScRequestID::default(),
        }
    }

    pub fn current_chain_id(&self) -> ScChainID {
        return self.chain_id;
    }

    pub fn init_func_call_context(&self) {
        todo!()
        // connect_host(self);
    }

    pub fn init_view_call_context(&self, contract_hname: ScHname) -> ScHname {
        todo!()
        // connect_host(self);
        // return self.scHname;
    }

    pub fn register(&self, handler: &dyn IEventHandler) -> Result<(), &'static str> {
        todo!()
        // for h in self.eventHandlers {
        // 	if h == handler {
        // 		return nil;
        // 	}
        // }
        // self.eventHandlers = append(self.eventHandlers, handler);
        // if len(self.eventHandlers) > 1 {
        // 	return nil;
        // }
        // return self.startEventHandlers();
    }

    // overrides default contract name
    pub fn service_contract_name(&self, contract_name: &str) {
        todo!()
        // self.scHname = NewScHname(contractName);
    }

    pub fn sign_requests(&self, key_pair: &keypair::KeyPair) {
        todo!()
        // self.keyPair = keyPair;
    }

    pub fn unregister(&self, handler: &dyn IEventHandler) {
        todo!()
        // for h in self.eventHandlers {
        // 	if h == handler {
        // 		self.eventHandlers = append(self.eventHandlers[:i], self.event_handlers[i+1:]...);
        // 		if len(self.eventHandlers) == 0 {
        // 			self.stopEventHandlers();
        // 		}
        // 		return;
        // 	}
        // }
    }

    pub fn wait_request(&self, req_ids: &[&ScRequestID]) -> Result<(), String> {
        todo!()
        // let requestID = self.ReqID;
        // if len(reqID) == 1 {
        // 	requestID = reqID[0];
        // }
        // return self.svcClient.WaitUntilRequestProcessed(self.chainID, requestID, 1*time.Minute);
    }

    pub fn start_event_handlers(&self) -> Result<(), String> {
        todo!()
        // let chMsg = make(chan []string, 20);
        // self.eventDone = make(chan: bool);
        // let err = self.svcClient.SubscribeEvents(chMsg, self.eventDone);
        // if err != nil {
        // 	return err;
        // }
        // go pub fn() {
        // 	for {
        // 		for let msgSplit = range chMsg {
        // 			let event = strings.Join(msgSplit, " ");
        // 			fmt.Printf("%self\n", event);
        // 			if msgSplit[0] == "vmmsg" {
        // 				let msg = strings.Split(msgSplit[3], "|");
        // 				let topic = msg[0];
        // 				let params = msg[1:];
        // 				for let _,  handler = range self.eventHandlers {
        // 					handler.CallHandler(topic, params);
        // 				}
        // 			}
        // 		}
        // 	}
        // }()
        // return nil;
    }

    pub fn stop_event_handlers(&self) {
        todo!()
        // if len(self.eventHandlers) > 0 {
        // 	self.eventDone <- true;
        // }
    }
}
