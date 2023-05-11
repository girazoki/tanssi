//! # Collator Assignment Pallet
//!
//! This pallet stores the AuthorityId -> AccountID mapping for two sessions

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use {
    frame_support::pallet_prelude::*,
    sp_runtime::{
        traits::{AtLeast32BitUnsigned, CheckedSub},
        RuntimeAppPublic,
    },
    sp_std::{collections::btree_map::BTreeMap, prelude::*, vec},
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type SessionIndex: parity_scale_codec::FullCodec + TypeInfo + Copy + AtLeast32BitUnsigned;

        // Sessions after which keys should be removed
        type SessionRemovalBoundary: Get<Self::SessionIndex>;

        type AuthorityId: Member
            + Parameter
            + Ord
            + RuntimeAppPublic
            + MaybeSerializeDeserialize
            + MaxEncodedLen;
    }

    #[pallet::storage]
    #[pallet::getter(fn authority_id_mapping)]
    pub(super) type AuthorityIdMapping<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::SessionIndex,
        BTreeMap<T::AuthorityId, T::AccountId>,
        OptionQuery,
    >;

    impl<T: Config> Pallet<T> {
        pub fn initializer_on_new_session(
            session_index: &T::SessionIndex,
            authorities: Vec<(T::AccountId, T::AuthorityId)>,
        ) {
            // Remove only if the checked sub does not saturate
            if let Some(session_index_to_remove) =
                session_index.checked_sub(&T::SessionRemovalBoundary::get())
            {
                AuthorityIdMapping::<T>::remove(session_index_to_remove)
            }

            let mut assignation = BTreeMap::new();
            for (account, key) in authorities {
                assignation.insert(key, account);
            }
            AuthorityIdMapping::<T>::insert(session_index, assignation);
        }
    }
}
