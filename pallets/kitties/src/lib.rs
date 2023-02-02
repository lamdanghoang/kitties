#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::dispatch::fmt;
	use frame_support::inherent::Vec;
	use frame_support::pallet_prelude::*;
	use frame_support::print;
	use frame_support::traits::tokens::currency::Currency;
	// use frame_support::traits::Randomness;
	use frame_support::Printable;
	use frame_system::pallet_prelude::*;

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Default, Encode, Decode, TypeInfo, Clone)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: BalanceOf<T>,
		gender: Gender,
	}

	impl<T: Config> fmt::Debug for Kitty<T> {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Kitty")
				.field("dna", &self.dna)
				.field("owner", &self.owner)
				.field("price", &self.price)
				.field("gender", &self.gender)
				.finish()
		}
	}

	#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	// The #[pallet::without_storage_info] macro enables you to define pallet storage items that don't have a fixed size.
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<Self::AccountId>;
		// type KittyRandomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn total_kitties)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	// Store total amount of kitties
	pub type TotalKitties<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	// Store kitty's dna with its information
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	// Store kitties that users own
	pub type Owner<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters.
		KittyGenerated {
			dna: Vec<u8>,
			owner: T::AccountId,
		},
		KittyTransferedTo {
			from: T::AccountId,
			to: T::AccountId,
			dna: Vec<u8>,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		InvalidPrice,
		NoKitty,
		StorageOverflow,
		KittyNotOwned,
		InvalidAccount,
	}

	impl<T: Config> Printable for Error<T> {
		fn print(&self) {
			match self {
				Error::InvalidPrice => "Invalid Price".print(),
				Error::StorageOverflow => "Storage Overflow".print(),
				_ => "Invalid Error".print(),
			}
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// ensure!(price > 0, Error::<T>::InvalidPrice);
			if price == 0 {
				print(Error::<T>::InvalidPrice);
				Err(Error::<T>::InvalidPrice)?;
			}

			// Generate gender
			let gender = Self::gen_gender(dna.clone())?;

			// Generate kitty
			let kitty =
				Kitty::<T> { dna: dna.clone(), owner: who.clone(), price: price.into(), gender };

			log::warn!("Owner: {:?}", &kitty.owner);
			log::info!("{:?}", kitty);

			// Update storage.
			let mut current_id = TotalKitties::<T>::get();
			current_id += 1;
			<TotalKitties<T>>::put(current_id);

			<Kitties<T>>::insert(dna.clone(), kitty.clone());

			// match Owner::<T>::get(who.clone()) {
			// 	Some(mut dnas) => {
			// 		dnas.push(dna.clone());
			// 		Owner::<T>::insert(who.clone(), dnas);
			// 	},
			// 	None => {
			// 		let mut dnas = Vec::new();
			// 		dnas.push(dna.clone());
			// 		Owner::<T>::insert(who.clone(), dnas);
			// 	},
			// }

			let mut kitties_owned = Owner::<T>::get(&who).unwrap_or_default();
			kitties_owned.push(kitty.dna.clone());
			Owner::<T>::insert(&who, kitties_owned);

			// Emit an event.
			Self::deposit_event(Event::KittyGenerated { dna: dna.clone(), owner: who.clone() });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn transfer_kitty(
			origin: OriginFor<T>,
			dna: Vec<u8>,
			receiver: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Get all kitties of sender
			let mut sender_kitties = Owner::<T>::get(who.clone()).unwrap_or_default();

			// Check ownership of kitty that sender want to transfer
			ensure!(sender_kitties.contains(&dna), Error::<T>::KittyNotOwned);

			// Get all kitties of receiver
			let mut receiver_kitties = Owner::<T>::get(receiver.clone()).unwrap_or_default();

			// Transfer kitty from sender to receiver
			sender_kitties.retain(|_dna| _dna != &dna);
			receiver_kitties.push(dna.clone());

			// Update information of kitty
			let mut updated_kitty = Kitties::<T>::get(dna.clone()).unwrap();
			updated_kitty.owner = receiver.clone();

			// Update storage
			<Owner<T>>::insert(who.clone(), sender_kitties);
			Owner::<T>::insert(receiver.clone(), receiver_kitties);
			<Kitties<T>>::insert(dna.clone(), updated_kitty);

			// Emit an event
			Self::deposit_event(Event::KittyTransferedTo {
				from: who.clone(),
				to: receiver.clone(),
				dna: dna.clone(),
			});

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}

	impl<T> Pallet<T> {
		fn gen_gender(dna: Vec<u8>) -> Result<Gender, Error<T>> {
			let mut gen = Gender::Male;
			if dna.len() % 2 != 0 {
				gen = Gender::Female;
			}
			Ok(gen)
		}
	}
}
