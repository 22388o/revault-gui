use bitcoin::{
    secp256k1,
    util::{bip32::ExtendedPubKey, psbt::PartiallySignedTransaction as Psbt},
};
use revaultd::revault_tx::miniscript::DescriptorPublicKey;
use std::{str::FromStr, sync::Arc, time::Duration};
use tokio::sync::Mutex;

use iced::{time, Command, Element, Subscription};

use revault_hwi::{Channel, Error};

use crate::{
    app::{context::Context, message::SignMessage, view::sign::SignerView},
    daemon::client::Client,
};

#[derive(Debug)]
pub struct RevocationTransactionsTarget {
    pub cancel_tx: Psbt,
    pub emergency_tx: Psbt,
    pub emergency_unvault_tx: Psbt,
}

impl RevocationTransactionsTarget {
    pub fn new(
        xpub: ExtendedPubKey,
        derivation_index: u32,
        mut cancel_tx: Psbt,
        mut emergency_tx: Psbt,
        mut emergency_unvault_tx: Psbt,
    ) -> Self {
        let xpub = DescriptorPublicKey::from_str(&format!("{}/*", xpub)).unwrap();
        let xpub = xpub.derive(derivation_index);
        let curve = secp256k1::Secp256k1::verification_only();

        let pubkey = xpub.derive_public_key(&curve).unwrap();
        let fingerprint = xpub.master_fingerprint();
        let derivation_path = xpub.full_derivation_path();

        cancel_tx.inputs.iter_mut().for_each(|input| {
            input
                .bip32_derivation
                .insert(pubkey, (fingerprint, derivation_path.clone()));
        });
        emergency_tx.inputs.iter_mut().for_each(|input| {
            input
                .bip32_derivation
                .insert(pubkey, (fingerprint, derivation_path.clone()));
        });
        emergency_unvault_tx.inputs.iter_mut().for_each(|input| {
            input
                .bip32_derivation
                .insert(pubkey, (fingerprint, derivation_path.clone()));
        });

        Self {
            cancel_tx,
            emergency_tx,
            emergency_unvault_tx,
        }
    }
}

#[derive(Debug)]
pub struct UnvaultTransactionTarget {
    pub unvault_tx: Psbt,
}

impl UnvaultTransactionTarget {
    pub fn new(xpub: ExtendedPubKey, derivation_index: u32, mut unvault_tx: Psbt) -> Self {
        let xpub = DescriptorPublicKey::from_str(&format!("{}/*", xpub)).unwrap();
        let xpub = xpub.derive(derivation_index);
        let curve = secp256k1::Secp256k1::verification_only();

        let pubkey = xpub.derive_public_key(&curve).unwrap();
        let fingerprint = xpub.master_fingerprint();
        let derivation_path = xpub.full_derivation_path();

        unvault_tx.inputs.iter_mut().for_each(|input| {
            input
                .bip32_derivation
                .insert(pubkey, (fingerprint, derivation_path.clone()));
        });

        Self { unvault_tx }
    }
}

#[derive(Debug)]
pub struct SpendTransactionTarget {
    pub spend_tx: Psbt,
}

impl SpendTransactionTarget {
    pub fn new(xpub: ExtendedPubKey, derivation_indexes: &Vec<u32>, mut spend_tx: Psbt) -> Self {
        let xpub = DescriptorPublicKey::from_str(&format!("{}/*", xpub)).unwrap();
        let curve = secp256k1::Secp256k1::verification_only();

        let derived_keys: Vec<DescriptorPublicKey> = derivation_indexes
            .iter()
            .map(|index| xpub.clone().derive(*index))
            .collect();

        spend_tx
            .inputs
            .iter_mut()
            .enumerate()
            .for_each(|(i, input)| {
                let key = &derived_keys[i];
                input.bip32_derivation.insert(
                    key.derive_public_key(&curve).unwrap(),
                    (key.master_fingerprint(), key.full_derivation_path()),
                );
            });

        Self { spend_tx }
    }
}

#[derive(Debug)]
pub struct Signer<T> {
    channel: Option<Arc<Mutex<Channel>>>,
    processing: bool,
    signed: bool,
    error: Option<Error>,

    pub target: T,

    view: SignerView,
}

impl<T> Signer<T> {
    pub fn new(target: T) -> Self {
        Signer {
            channel: None,
            processing: false,
            signed: false,
            error: None,
            target,
            view: SignerView::new(),
        }
    }

    pub fn signed(&self) -> bool {
        self.signed
    }

    pub fn check_channel(&mut self) -> Command<SignMessage> {
        if let Some(channel) = &self.channel {
            Command::perform(ping(channel.clone()), SignMessage::Ping)
        } else {
            Command::perform(Channel::try_connect(), |res| {
                SignMessage::Connected(res.map(|channel| Arc::new(Mutex::new(channel))))
            })
        }
    }

    pub fn update_channel(&mut self, message: SignMessage) -> Command<SignMessage> {
        match message {
            SignMessage::Ping(res) => {
                if res.is_err() {
                    self.channel = None;
                }
            }
            SignMessage::CheckConnection => return self.check_channel(),
            SignMessage::Connected(Ok(channel)) => self.channel = Some(channel),
            _ => {}
        };
        Command::none()
    }

    pub fn subscription(&self) -> Subscription<SignMessage> {
        if !self.signed && !self.processing {
            time::every(Duration::from_secs(1)).map(|_| SignMessage::CheckConnection)
        } else {
            Subscription::none()
        }
    }

    pub fn view<C: Client>(&mut self, ctx: &Context<C>) -> Element<SignMessage> {
        self.view
            .view(ctx, self.channel.is_some(), self.processing, self.signed)
    }
}

pub async fn ping(channel: Arc<Mutex<Channel>>) -> Result<(), Error> {
    channel.clone().lock().await.ping().await
}

impl Signer<SpendTransactionTarget> {
    pub fn update(&mut self, message: SignMessage) -> Command<SignMessage> {
        match message {
            SignMessage::SelectSign => {
                if let Some(channel) = &self.channel {
                    self.processing = true;
                    return Command::perform(
                        sign_spend_tx(channel.clone(), self.target.spend_tx.clone()),
                        SignMessage::Signed,
                    );
                }
            }
            SignMessage::Signed(res) => {
                self.processing = false;
                match res {
                    Ok(txs) => {
                        self.signed = true;
                        let txs = *txs;
                        if let Some(tx) = txs.into_iter().find(|psbt| {
                            psbt.global.unsigned_tx.txid()
                                == self.target.spend_tx.global.unsigned_tx.txid()
                        }) {
                            self.target.spend_tx = tx;
                        }
                    }
                    Err(e) => {
                        tracing::info!("{:?}", e);
                        self.error = Some(e);
                    }
                }
            }
            _ => return self.update_channel(message),
        };
        Command::none()
    }
}

pub async fn sign_spend_tx(
    channel: Arc<Mutex<Channel>>,
    spend_tx: Psbt,
) -> Result<Box<Vec<Psbt>>, Error> {
    channel.clone().lock().await.sign_spend_tx(spend_tx).await
}

impl Signer<UnvaultTransactionTarget> {
    pub fn update(&mut self, message: SignMessage) -> Command<SignMessage> {
        match message {
            SignMessage::SelectSign => {
                if let Some(channel) = &self.channel {
                    self.processing = true;
                    return Command::perform(
                        sign_unvault_tx(channel.clone(), self.target.unvault_tx.clone()),
                        SignMessage::Signed,
                    );
                }
            }
            SignMessage::Signed(res) => {
                self.processing = false;
                match res {
                    Ok(txs) => {
                        self.signed = true;
                        let txs = *txs;
                        if let Some(tx) = txs.into_iter().find(|psbt| {
                            psbt.global.unsigned_tx.txid()
                                == self.target.unvault_tx.global.unsigned_tx.txid()
                        }) {
                            self.target.unvault_tx = tx;
                        }
                    }
                    Err(e) => self.error = Some(e),
                }
            }
            _ => return self.update_channel(message),
        };
        Command::none()
    }
}

pub async fn sign_unvault_tx(
    channel: Arc<Mutex<Channel>>,
    unvault_tx: Psbt,
) -> Result<Box<Vec<Psbt>>, Error> {
    channel
        .clone()
        .lock()
        .await
        .sign_unvault_tx(unvault_tx)
        .await
}

impl Signer<RevocationTransactionsTarget> {
    pub fn update(&mut self, message: SignMessage) -> Command<SignMessage> {
        match message {
            SignMessage::SelectSign => {
                if let Some(channel) = &self.channel {
                    self.processing = true;
                    return Command::perform(
                        sign_revocation_txs(
                            channel.clone(),
                            self.target.emergency_tx.clone(),
                            self.target.emergency_unvault_tx.clone(),
                            self.target.cancel_tx.clone(),
                        ),
                        SignMessage::Signed,
                    );
                }
            }
            SignMessage::Signed(res) => {
                self.processing = false;
                match res {
                    Ok(txs) => {
                        self.signed = true;
                        let txs = *txs;
                        if let Some(tx) = txs.iter().find(|psbt| {
                            psbt.global.unsigned_tx.txid()
                                == self.target.cancel_tx.global.unsigned_tx.txid()
                        }) {
                            self.target.cancel_tx = tx.clone();
                        }
                        if let Some(tx) = txs.iter().find(|psbt| {
                            psbt.global.unsigned_tx.txid()
                                == self.target.emergency_tx.global.unsigned_tx.txid()
                        }) {
                            self.target.emergency_tx = tx.clone();
                        }
                        if let Some(tx) = txs.into_iter().find(|psbt| {
                            psbt.global.unsigned_tx.txid()
                                == self.target.emergency_unvault_tx.global.unsigned_tx.txid()
                        }) {
                            self.target.emergency_unvault_tx = tx;
                        }
                    }
                    Err(e) => self.error = Some(e),
                }
            }
            _ => return self.update_channel(message),
        };
        Command::none()
    }
}

pub async fn sign_revocation_txs(
    channel: Arc<Mutex<Channel>>,
    emergency_tx: Psbt,
    emergency_unvault_tx: Psbt,
    cancel_tx: Psbt,
) -> Result<Box<Vec<Psbt>>, Error> {
    channel
        .clone()
        .lock()
        .await
        .sign_revocation_txs(emergency_tx, emergency_unvault_tx, cancel_tx)
        .await
}
