use bitcoin::util::psbt::PartiallySignedTransaction as Psbt;
use std::convert::From;

use iced::{Command, Element, Subscription};

use crate::{
    app::{
        context::Context,
        error::Error,
        message::{Message, SpendTxMessage},
        state::{
            cmd::{
                broadcast_spend_tx, delete_spend_tx, list_spend_txs, list_vaults, update_spend_tx,
            },
            sign::{Signer, SpendTransactionTarget},
            State,
        },
        view::spend_transaction::{
            SpendTransactionBroadcastView, SpendTransactionDeleteView,
            SpendTransactionListItemView, SpendTransactionSharePsbtView, SpendTransactionSignView,
            SpendTransactionView,
        },
    },
    revaultd::model,
};

#[derive(Debug)]
pub struct SpendTransactionState {
    pub psbt: Psbt,
    cpfp_index: usize,
    change_index: Option<usize>,

    deposit_outpoints: Vec<String>,
    deposits: Vec<model::Vault>,
    warning: Option<Error>,

    action: SpendTransactionAction,

    view: SpendTransactionView,
}

impl SpendTransactionState {
    pub fn new(psbt: Psbt) -> Self {
        Self {
            cpfp_index: 0,
            change_index: None,
            psbt,
            deposit_outpoints: Vec::new(),
            deposits: Vec::new(),
            action: SpendTransactionAction::new(),
            warning: None,
            view: SpendTransactionView::new(),
        }
    }
}

impl State for SpendTransactionState {
    fn update(&mut self, ctx: &Context, message: Message) -> Command<Message> {
        match message {
            Message::SpendTx(SpendTxMessage::Inputs(res)) => match res {
                Ok(vaults) => {
                    self.deposits = Vec::new();
                    // we keep the order of the deposit_outpoints.
                    for deposit in vaults.into_iter() {
                        for outpoint in &self.deposit_outpoints {
                            if deposit.outpoint() == *outpoint {
                                self.deposits.push(deposit.clone());
                            }
                        }
                    }
                }
                Err(e) => self.warning = Error::from(e).into(),
            },
            Message::SpendTx(SpendTxMessage::SpendTransactions(res)) => match res {
                Ok(txs) => {
                    for tx in txs {
                        if tx.psbt.global.unsigned_tx.txid() == self.psbt.global.unsigned_tx.txid()
                        {
                            self.deposit_outpoints = tx.deposit_outpoints;
                            self.psbt = tx.psbt;
                            self.cpfp_index = tx.cpfp_index;
                            self.change_index = tx.change_index;
                            return Command::perform(
                                list_vaults(
                                    ctx.revaultd.clone(),
                                    None,
                                    Some(self.deposit_outpoints.clone()),
                                ),
                                |res| Message::SpendTx(SpendTxMessage::Inputs(res)),
                            );
                        }
                    }
                }
                Err(e) => self.warning = Error::from(e).into(),
            },
            Message::SpendTx(msg) => {
                return self
                    .action
                    .update(ctx, &self.deposits, &mut self.psbt, msg)
                    .map(Message::SpendTx);
            }
            _ => {}
        };
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if let SpendTransactionAction::Sign { signer, .. } = &self.action {
            signer
                .subscription()
                .map(|msg| Message::SpendTx(SpendTxMessage::Sign(msg)))
        } else {
            Subscription::none()
        }
    }

    fn view(&mut self, ctx: &Context) -> Element<Message> {
        let show_delete_button = !matches!(self.action, SpendTransactionAction::Delete { .. });
        self.view.view(
            ctx,
            &self.psbt,
            self.cpfp_index,
            self.change_index,
            &self.deposits,
            self.action.view(ctx, &self.psbt),
            self.warning.as_ref(),
            show_delete_button,
        )
    }

    fn load(&self, ctx: &Context) -> Command<Message> {
        Command::perform(list_spend_txs(ctx.revaultd.clone(), None), |res| {
            Message::SpendTx(SpendTxMessage::SpendTransactions(res))
        })
    }
}

#[derive(Debug)]
pub enum SpendTransactionAction {
    SharePsbt {
        psbt_input: String,
        processing: bool,
        success: bool,
        warning: Option<Error>,
        view: SpendTransactionSharePsbtView,
    },
    Sign {
        warning: Option<Error>,
        processing: bool,
        signer: Signer<SpendTransactionTarget>,
        view: SpendTransactionSignView,
    },
    Broadcast {
        processing: bool,
        success: bool,
        warning: Option<Error>,
        view: SpendTransactionBroadcastView,
    },
    Delete {
        processing: bool,
        success: bool,
        warning: Option<Error>,
        view: SpendTransactionDeleteView,
    },
}

impl SpendTransactionAction {
    fn new() -> Self {
        Self::SharePsbt {
            psbt_input: "".to_string(),
            processing: false,
            success: false,
            warning: None,
            view: SpendTransactionSharePsbtView::new(),
        }
    }
    fn update(
        &mut self,
        ctx: &Context,
        deposits: &Vec<model::Vault>,
        psbt: &mut Psbt,
        message: SpendTxMessage,
    ) -> Command<SpendTxMessage> {
        match message {
            SpendTxMessage::Delete => {
                if let Self::Delete { processing, .. } = self {
                    *processing = true;
                    return Command::perform(
                        delete_spend_tx(
                            ctx.revaultd.clone(),
                            psbt.global.unsigned_tx.txid().to_string(),
                        ),
                        SpendTxMessage::Deleted,
                    );
                }
            }
            SpendTxMessage::Deleted(res) => {
                if let Self::Delete {
                    processing,
                    success,
                    warning,
                    ..
                } = self
                {
                    *processing = false;
                    match res {
                        Ok(()) => *success = true,
                        Err(e) => *warning = Error::from(e).into(),
                    };
                }
            }
            SpendTxMessage::SelectShare => {
                *self = Self::new();
            }
            SpendTxMessage::SelectDelete => {
                *self = Self::Delete {
                    processing: false,
                    success: false,
                    warning: None,
                    view: SpendTransactionDeleteView::new(),
                };
            }
            SpendTxMessage::SelectSign => {
                *self = Self::Sign {
                    processing: false,
                    warning: None,
                    signer: Signer::new(SpendTransactionTarget::new(
                        ctx.revaultd.config.manager_config.as_ref().unwrap().xpub,
                        &deposits
                            .iter()
                            .map(|deposit| deposit.derivation_index)
                            .collect(),
                        psbt.clone(),
                    )),
                    view: SpendTransactionSignView::new(),
                };
            }
            SpendTxMessage::Sign(msg) => {
                if let Self::Sign {
                    signer, processing, ..
                } = self
                {
                    let cmd = signer.update(msg);
                    if signer.signed() && !*processing {
                        *psbt = signer.target.spend_tx.clone();
                        *processing = true;
                        return Command::perform(
                            update_spend_tx(ctx.revaultd.clone(), signer.target.spend_tx.clone()),
                            SpendTxMessage::Signed,
                        );
                    }
                    return cmd.map(SpendTxMessage::Sign);
                }
            }
            SpendTxMessage::Signed(res) => {
                if let Self::Sign {
                    warning,
                    signer,
                    processing,
                    ..
                } = self
                {
                    *processing = false;
                    match res {
                        Ok(_) => {
                            // During this step state has a generated psbt
                            // and signer has a signed psbt.
                            *warning = None;
                            *psbt = signer.target.spend_tx.clone();
                        }

                        Err(e) => *warning = Some(Error::RevaultDError(e)),
                    }
                }
            }
            SpendTxMessage::SelectBroadcast => {
                *self = Self::Broadcast {
                    processing: false,
                    success: false,
                    warning: None,
                    view: SpendTransactionBroadcastView::new(),
                };
            }
            SpendTxMessage::Broadcast => {
                if let Self::Broadcast { processing, .. } = self {
                    *processing = true;
                    return Command::perform(
                        broadcast_spend_tx(
                            ctx.revaultd.clone(),
                            psbt.global.unsigned_tx.txid().to_string(),
                        ),
                        SpendTxMessage::Broadcasted,
                    );
                }
            }
            SpendTxMessage::Broadcasted(res) => {
                if let Self::Broadcast {
                    processing,
                    success,
                    warning,
                    ..
                } = self
                {
                    *processing = false;
                    match res {
                        Ok(()) => *success = true,
                        Err(e) => *warning = Error::from(e).into(),
                    };
                }
            }
            SpendTxMessage::PsbtEdited(input) => {
                if let Self::SharePsbt {
                    psbt_input,
                    processing,
                    success,
                    ..
                } = self
                {
                    *success = false;
                    if !*processing {
                        *psbt_input = input;
                    }
                }
            }
            SpendTxMessage::Update => {
                if let Self::SharePsbt {
                    psbt_input,
                    processing,
                    warning,
                    ..
                } = self
                {
                    let p: Option<Psbt> = bitcoin::base64::decode(&psbt_input)
                        .ok()
                        .and_then(|bytes| bitcoin::consensus::encode::deserialize(&bytes).ok());
                    if let Some(p) = p {
                        if p.global.unsigned_tx.txid() != psbt.global.unsigned_tx.txid() {
                            *warning = Error::UnexpectedError(
                                "Entered PSBT is for a different transaction".to_string(),
                            )
                            .into();
                        } else {
                            *processing = true;
                            return Command::perform(
                                update_spend_tx(ctx.revaultd.clone(), p),
                                SpendTxMessage::Updated,
                            );
                        }
                    } else {
                        *warning =
                            Error::UnexpectedError("Please enter a valid psbt".to_string()).into();
                    }
                }
            }
            SpendTxMessage::Updated(res) => {
                if let Self::SharePsbt {
                    psbt_input,
                    processing,
                    success,
                    warning,
                    ..
                } = self
                {
                    match res {
                        Ok(()) => {
                            *success = true;
                            *psbt = bitcoin::consensus::encode::deserialize(
                                &bitcoin::base64::decode(&psbt_input)
                                    .expect("psbt was successfully updated with the given input"),
                            )
                            .expect("psbt was successfully updated with the given input");
                            *psbt_input = "".to_string();
                        }
                        Err(e) => *warning = Error::from(e).into(),
                    };
                    *processing = false;
                }
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&mut self, ctx: &Context, psbt: &Psbt) -> Element<Message> {
        match self {
            Self::Sign {
                signer,
                warning,
                view,
                ..
            } => view.view(
                signer
                    .view(ctx)
                    .map(|msg| Message::SpendTx(SpendTxMessage::Sign(msg))),
                warning.as_ref(),
            ),
            Self::SharePsbt {
                view,
                psbt_input,
                processing,
                success,
                warning,
                ..
            } => view.view(&psbt_input, &processing, &success, psbt, warning.as_ref()),
            Self::Broadcast {
                view,
                processing,
                success,
                warning,
            } => view.view(&processing, &success, warning.as_ref()),
            Self::Delete {
                view,
                processing,
                success,
                warning,
            } => view.view(&processing, &success, warning.as_ref()),
        }
    }
}

#[derive(Debug)]
pub struct SpendTransactionListItem {
    pub tx: model::SpendTx,

    spend_amount: u64,
    fees: u64,

    view: SpendTransactionListItemView,
}

impl SpendTransactionListItem {
    pub fn new(tx: model::SpendTx, vaults_amount: u64) -> Self {
        let spend_amount = tx
            .psbt
            .global
            .unsigned_tx
            .output
            .iter()
            .enumerate()
            .filter(|(i, _)| Some(i) != tx.change_index.as_ref() && i != &tx.cpfp_index)
            .fold(0, |acc, (_, output)| acc + output.value);
        let change_amount = if let Some(i) = tx.change_index {
            tx.psbt.global.unsigned_tx.output[i].value
        } else {
            0
        };

        let fees = if vaults_amount == 0 {
            // Vaults are still loading
            0
        } else {
            vaults_amount - spend_amount - change_amount
        };
        Self {
            tx,
            spend_amount,
            fees,
            view: SpendTransactionListItemView::new(),
        }
    }

    pub fn view(&mut self, ctx: &Context) -> Element<SpendTxMessage> {
        self.view.view(ctx, &self.tx, self.spend_amount, self.fees)
    }
}
