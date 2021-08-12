use crate::secret_channel::{
    storage_prefix_for_topic_pubkey, KeyPair, Peeler, PeelingReceiver, SecretMq,
};
use crate::std::fmt::Debug;
use crate::std::string::String;
use crate::system::System;
use crate::system::TransactionReceipt;

use super::TransactionStatus;
use crate::types::{deopaque_query, OpaqueError, OpaqueQuery, OpaqueReply};
use anyhow::{Context, Error, Result};
use core::str;
use parity_scale_codec::{Decode, Encode};
use phala_mq::{MessageOrigin, Sr25519MessageChannel as MessageChannel};
use phala_types::messaging::PushCommand;

use sp_core::H256;
use sp_runtime_interface::pass_by::PassByInner as _;

pub mod assets;
pub mod balances;
pub mod btc_lottery;
pub mod data_plaza;
// pub mod diem;
pub mod substrate_kitties;
pub mod web3analytics;
pub mod woothee;

pub use phala_types::contract::*;

// TODO.kevin: the wrapper is no longer needed.
#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Encode, Decode)]
pub struct AccountIdWrapper(pub chain::AccountId);

impl AccountIdWrapper {
    fn try_from(b: &[u8]) -> Result<Self> {
        let mut a = AccountIdWrapper::default();
        use core::convert::TryFrom;
        a.0 = sp_core::crypto::AccountId32::try_from(b)
            .map_err(|_| Error::msg("Failed to parse AccountId32"))?;
        Ok(a)
    }
    fn from_hex(s: &str) -> Result<Self> {
        let bytes = hex::decode(s)
            .map_err(Error::msg)
            .context("Failed to decode AccountId hex")?;
        Self::try_from(&bytes)
    }
    fn to_string(&self) -> String {
        hex::encode(&self.0)
    }
}

impl From<H256> for AccountIdWrapper {
    fn from(hash: H256) -> Self {
        Self((*hash.inner()).into())
    }
}

pub use support::*;
mod support {
    use core::convert::TryInto;

    use super::*;
    use crate::types::BlockInfo;

    pub struct ExecuteEnv<'a> {
        pub block: &'a BlockInfo<'a>,
        pub system: &'a mut System,
    }

    pub struct NativeContext<'a> {
        pub block: &'a BlockInfo<'a>,
        mq: &'a MessageChannel,
        secret_mq: SecretMq<'a>,
    }

    impl NativeContext<'_> {
        pub fn mq(&self) -> &MessageChannel {
            self.mq
        }
    }

    pub trait Contract {
        fn id(&self) -> ContractId;
        fn handle_query(
            &mut self,
            origin: Option<&chain::AccountId>,
            req: OpaqueQuery,
        ) -> Result<OpaqueReply, OpaqueError>;
        fn process_messages(&mut self, env: &mut ExecuteEnv);
    }

    pub trait NativeContract {
        type Cmd: Decode + Debug;
        type Event: Decode + Debug;
        type QReq: Decode + Debug;
        type QResp: Encode + Debug;

        fn id(&self) -> ContractId32;
        fn handle_command(
            &mut self,
            _context: &NativeContext,
            _origin: MessageOrigin,
            _cmd: PushCommand<Self::Cmd>,
        ) -> TransactionStatus {
            TransactionStatus::Ok
        }
        fn handle_event(
            &mut self,
            _context: &NativeContext,
            _origin: MessageOrigin,
            _event: Self::Event,
        ) {
        }
        fn handle_query(
            &mut self,
            origin: Option<&chain::AccountId>,
            req: Self::QReq,
        ) -> Self::QResp;
    }

    pub struct NativeCompatContract<
        Con,
        Cmd,
        CmdWrp,
        CmdPlr,
        Event,
        EventWrp,
        EventPlr,
        QReq,
        QResp,
    >
    where
        Cmd: Decode + Debug,
        CmdWrp: Decode + Debug,
        CmdPlr: Peeler<Wrp = CmdWrp, Msg = PushCommand<Cmd>>,
        Event: Decode + Debug,
        EventWrp: Decode + Debug,
        EventPlr: Peeler<Wrp = EventWrp, Msg = Event>,
        QReq: Decode + Debug,
        QResp: Encode + Debug,
        Con: NativeContract<Cmd = Cmd, Event = Event, QReq = QReq, QResp = QResp>,
    {
        contract: Con,
        send_mq: MessageChannel,
        cmd_rcv_mq: PeelingReceiver<PushCommand<Cmd>, CmdWrp, CmdPlr>,
        event_rcv_mq: PeelingReceiver<Event, EventWrp, EventPlr>,
        ecdh_key: KeyPair,
    }

    impl<Con, Cmd, CmdWrp, CmdPlr, Event, EventWrp, EventPlr, QReq, QResp>
        NativeCompatContract<Con, Cmd, CmdWrp, CmdPlr, Event, EventWrp, EventPlr, QReq, QResp>
    where
        Cmd: Decode + Debug,
        CmdWrp: Decode + Debug,
        CmdPlr: Peeler<Wrp = CmdWrp, Msg = PushCommand<Cmd>>,
        Event: Decode + Debug,
        EventWrp: Decode + Debug,
        EventPlr: Peeler<Wrp = EventWrp, Msg = Event>,
        QReq: Decode + Debug,
        QResp: Encode + Debug,
        Con: NativeContract<Cmd = Cmd, Event = Event, QReq = QReq, QResp = QResp>,
        PushCommand<Cmd>: Decode + Debug,
    {
        pub fn new(
            contract: Con,
            send_mq: MessageChannel,
            cmd_rcv_mq: PeelingReceiver<PushCommand<Cmd>, CmdWrp, CmdPlr>,
            event_rcv_mq: PeelingReceiver<Event, EventWrp, EventPlr>,
            ecdh_key: KeyPair,
        ) -> Self {
            NativeCompatContract {
                contract,
                send_mq,
                cmd_rcv_mq,
                event_rcv_mq,
                ecdh_key,
            }
        }
    }

    impl<Con, Cmd, CmdWrp, CmdPlr, Event, EventWrp, EventPlr, QReq, QResp> Contract
        for NativeCompatContract<Con, Cmd, CmdWrp, CmdPlr, Event, EventWrp, EventPlr, QReq, QResp>
    where
        Cmd: Decode + Debug + Send + Sync,
        CmdWrp: Decode + Debug + Send + Sync,
        CmdPlr: Peeler<Wrp = CmdWrp, Msg = PushCommand<Cmd>> + Send + Sync,
        Event: Decode + Debug + Send + Sync,
        EventWrp: Decode + Debug + Send + Sync,
        EventPlr: Peeler<Wrp = EventWrp, Msg = Event> + Send + Sync,
        QReq: Decode + Debug,
        QResp: Encode + Debug,
        Con: NativeContract<Cmd = Cmd, Event = Event, QReq = QReq, QResp = QResp> + Send + Sync,
        PushCommand<Cmd>: Decode + Debug,
    {
        fn id(&self) -> ContractId {
            id256(self.contract.id())
        }

        fn handle_query(
            &mut self,
            origin: Option<&runtime::AccountId>,
            req: OpaqueQuery,
        ) -> Result<OpaqueReply, OpaqueError> {
            let response = self.contract.handle_query(origin, deopaque_query(req)?);
            Ok(response.encode())
        }

        fn process_messages(&mut self, env: &mut ExecuteEnv) {
            let storage = env.block.storage;
            let key_map = |topic: &phala_mq::Path| {
                // TODO.kevin: query contract pubkey for contract topic's when the feature in GK is available.
                storage
                    .get(&storage_prefix_for_topic_pubkey(topic))
                    .map(|v| v.try_into().ok())
                    .flatten()
            };
            let secret_mq = SecretMq::new(&self.ecdh_key, &self.send_mq, &key_map);
            let context = NativeContext {
                block: env.block,
                mq: &self.send_mq,
                secret_mq,
            };
            loop {
                let ok = phala_mq::select! {
                    next_cmd = self.cmd_rcv_mq => match next_cmd {
                        Ok((_, cmd, origin)) => {
                            let cmd_number = cmd.number;
                            let status = self.contract.handle_command(&context, origin.clone(), cmd);
                            env.system.add_receipt(
                                cmd_number,
                                TransactionReceipt {
                                    account: origin,
                                    block_num: env.block.block_number,
                                    contract_id: self.id(),
                                    status,
                                },
                            );
                        }
                        Err(e) => {
                            error!("Read command failed: {:?}", e);
                        }
                    },
                    next_event = self.event_rcv_mq => match next_event {
                        Ok((_, event, origin)) => {
                            self.contract.handle_event(&context, origin, event);
                        }
                        Err(e) => {
                            error!("Read event failed: {:?}", e);
                        },
                    },
                };
                if ok.is_none() {
                    break;
                }
            }
        }
    }
}
