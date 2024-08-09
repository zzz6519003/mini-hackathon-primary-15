#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::storage]
    #[pallet::getter(fn get_balance)]
    pub type Balance<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a balance is set.
        BalanceSet(T::AccountId, u64),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Error returned when setting balance fails.
        SetBalanceFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn set_balance(origin: OriginFor<T>, account: T::AccountId, amount: u64) -> DispatchResult {
            let _sender = ensure_signed(origin)?;

            <Balance<T>>::insert(&account, amount);
            Self::deposit_event(Event::BalanceSet(account, amount));
            Ok(())
        }
    }
}
