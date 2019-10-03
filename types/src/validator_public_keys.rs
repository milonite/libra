// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    account_address::AccountAddress,
    proto::validator_public_keys::ValidatorPublicKeys as ProtoValidatorPublicKeys,
};
use canonical_serialization::{
    CanonicalDeserialize, CanonicalDeserializer, CanonicalSerialize, CanonicalSerializer,
};
use crypto::{ed25519::*, traits::ValidKey, x25519::X25519StaticPublicKey};
use failure::Result;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;
use proto_conv::{FromProto, IntoProto};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt};

/// After executing a special transaction that sets the validators that should be used for the
/// next epoch, consensus and networking get the new list of validators.  Consensus will have a
/// public key to validate signed messages and networking will have a TBD public key for
/// creating secure channels of communication between validators.  The validators and their
/// public keys may or may not change between epochs.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct ValidatorPublicKeys {
    // Hash value of the current public key of the account address
    account_address: AccountAddress,
    // This key can validate messages sent from this validator
    consensus_public_key: Ed25519PublicKey,
    // This key can validate signed messages at the network layer
    network_signing_public_key: Ed25519PublicKey,
    // This key establishes the corresponding PrivateKey holder's eligibility to join the p2p
    // network
    network_identity_public_key: X25519StaticPublicKey,
}

impl fmt::Display for ValidatorPublicKeys {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "account_address: {}", self.account_address.short_str())
    }
}

impl ValidatorPublicKeys {
    pub fn new(
        account_address: AccountAddress,
        consensus_public_key: Ed25519PublicKey,
        network_signing_public_key: Ed25519PublicKey,
        network_identity_public_key: X25519StaticPublicKey,
    ) -> Self {
        ValidatorPublicKeys {
            account_address,
            consensus_public_key,
            network_signing_public_key,
            network_identity_public_key,
        }
    }

    /// Returns the id of this validator (hash of the current public key of the
    /// validator associated account address)
    pub fn account_address(&self) -> &AccountAddress {
        &self.account_address
    }

    /// Returns the key for validating signed messages from this validator
    pub fn consensus_public_key(&self) -> &Ed25519PublicKey {
        &self.consensus_public_key
    }

    /// Returns the key for validating signed messages at the network layers
    pub fn network_signing_public_key(&self) -> &Ed25519PublicKey {
        &self.network_signing_public_key
    }

    /// Returns the key that establishes a validator's identity in the p2p network
    pub fn network_identity_public_key(&self) -> &X25519StaticPublicKey {
        &self.network_identity_public_key
    }
}

impl FromProto for ValidatorPublicKeys {
    type ProtoType = ProtoValidatorPublicKeys;

    fn from_proto(object: Self::ProtoType) -> Result<Self> {
        let account_address = AccountAddress::from_proto(object.get_account_address().to_vec())?;
        let consensus_public_key = Ed25519PublicKey::try_from(object.get_consensus_public_key())?;
        let network_signing_public_key =
            Ed25519PublicKey::try_from(object.get_network_signing_public_key())?;
        let network_identity_public_key =
            X25519StaticPublicKey::try_from(object.get_network_identity_public_key())?;
        Ok(Self::new(
            account_address,
            consensus_public_key,
            network_signing_public_key,
            network_identity_public_key,
        ))
    }
}

impl IntoProto for ValidatorPublicKeys {
    type ProtoType = ProtoValidatorPublicKeys;

    fn into_proto(self) -> Self::ProtoType {
        let mut proto = Self::ProtoType::new();
        proto.set_account_address(AccountAddress::into_proto(self.account_address));
        proto.set_consensus_public_key(
            Ed25519PublicKey::to_bytes(&self.consensus_public_key).to_vec(),
        );
        proto.set_network_signing_public_key(
            Ed25519PublicKey::to_bytes(&self.network_signing_public_key).to_vec(),
        );
        proto.set_network_identity_public_key(
            X25519StaticPublicKey::to_bytes(&self.network_identity_public_key).to_vec(),
        );
        proto
    }
}

impl TryFrom<crate::proto::types::ValidatorPublicKeys> for ValidatorPublicKeys {
    type Error = failure::Error;

    fn try_from(proto: crate::proto::types::ValidatorPublicKeys) -> Result<Self> {
        let account_address = AccountAddress::try_from(proto.account_address)?;
        let consensus_public_key = Ed25519PublicKey::try_from(&proto.consensus_public_key[..])?;
        let network_signing_public_key =
            Ed25519PublicKey::try_from(&proto.network_signing_public_key[..])?;
        let network_identity_public_key =
            X25519StaticPublicKey::try_from(&proto.network_identity_public_key[..])?;
        Ok(Self::new(
            account_address,
            consensus_public_key,
            network_signing_public_key,
            network_identity_public_key,
        ))
    }
}

impl From<ValidatorPublicKeys> for crate::proto::types::ValidatorPublicKeys {
    fn from(keys: ValidatorPublicKeys) -> Self {
        Self {
            account_address: keys.account_address.to_vec(),
            consensus_public_key: keys.consensus_public_key.to_bytes().to_vec(),
            network_signing_public_key: keys.network_signing_public_key.to_bytes().to_vec(),
            network_identity_public_key: keys.network_identity_public_key.to_bytes().to_vec(),
        }
    }
}

impl CanonicalSerialize for ValidatorPublicKeys {
    fn serialize(&self, serializer: &mut impl CanonicalSerializer) -> Result<()> {
        serializer
            .encode_struct(&self.account_address)?
            .encode_struct(&self.consensus_public_key)?
            .encode_struct(&self.network_signing_public_key)?
            .encode_struct(&self.network_identity_public_key)?;
        Ok(())
    }
}

impl CanonicalDeserialize for ValidatorPublicKeys {
    fn deserialize(deserializer: &mut impl CanonicalDeserializer) -> Result<Self> {
        let account_address: AccountAddress = deserializer.decode_struct()?;
        let consensus_public_key: Ed25519PublicKey = deserializer.decode_struct()?;
        let network_signing_public_key: Ed25519PublicKey = deserializer.decode_struct()?;
        let network_identity_public_key: X25519StaticPublicKey = deserializer.decode_struct()?;
        Ok(ValidatorPublicKeys::new(
            account_address,
            consensus_public_key,
            network_signing_public_key,
            network_identity_public_key,
        ))
    }
}
