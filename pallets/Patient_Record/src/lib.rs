#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
// use frame_support::traits::TypeInfo;
// use frame_support::inherent::Vec;
// #[cfg(test)]
// mod mock;

// #[cfg(test)]

// mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::{OptionQuery, ValueQuery, *};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	// use frame_support::traits::TypeInfo;
	use frame_support::pallet_prelude::TypeInfo;

	// use frame_system::Config as SystemConfig;

	// use frame_system::Trait as SystemTrait;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	pub type PatientId<T> = <T as frame_system::Config>::AccountId;

	// #[derive(Encode, Decode,  Clone, PartialEq)]
	// #[derive(Debug, Default)]
	// #[derive(Debug)]
	// pub struct Patient_Record<AccountId> {
	// 	gender: Gender,
	// 	account_id: AccountId,

	// }

	#[derive(
		Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug, TypeInfo, MaxEncodedLen,
	)]
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	pub enum Gender {
		Male,
		Female,
		Other,
	}

	#[derive(Encode, Decode, TypeInfo)]
	pub struct Result {
		patient_id: u32,
		name: Vec<u8>,
		age: u32,
		gender: Gender,
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn patient_record_storage)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	// pub type PatientRecord<T> = StorageMap<_, Blake2_128Concat, Vec<u8>, u32,gender: Gender, ValueQuery>;
	// pub type PatientRecord<T> = StorageMap<_ ,Blake2_128Concat, T::AccountId, PatientRecord<T::AccountId>>;
	pub type PatientRecordStorage<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Result, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	// #[derive(Debug)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		PatientRecordStored {
			patient_id: u32,
			name: Vec<u8>,
			age: u32,
			gender: Gender,
			who: T::AccountId,
		},
		// RecordUpdated{ patient_id: u8,name: Vec<u8>, age: u32,gender: Gender, sender: T::AccountId },
		RecordUpdated {
			patient_id: u32,
			name: Vec<u8>,
			age: u32,
			gender: Gender,
			sender: T::AccountId,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		RecordNotFound,
		// [("Patient record already exists")],
		PatientRecordExists,
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
		pub fn add_patient_record(
			origin: OriginFor<T>,
			patient_id: u32,
			name: Vec<u8>,
			age: u32,
			gender: Gender,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Retrieve the patient record associated with the patient_id
			// let patient_record_exists = PatientRecordStorage::<T>::Result{ patient_id, name, age, gender };
			// // // Check if patient record already exists with the same patient_id
			// ensure!(
			// 	!<PatientRecordStorage<T>>::contains_key(&Result.patient_id.into()),
			// 	Error::<T>::PatientRecordExists
			// );

			let res = Result { patient_id, name: name.clone(), age, gender: gender.clone() };

			// ensure!(!<PatientRecordStorage<T>>::contains_key(&patient_id),Error::<T>::PatientRecordExists);

			// Update storage.
			PatientRecordStorage::<T>::insert(&who, &res);

			// Emit an event.
			Self::deposit_event(Event::PatientRecordStored {
				patient_id,
				name: name.clone(),
				age,
				gender,
				who,
			});
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update_record(
			origin: OriginFor<T>,
			patient_id: u32,
			name: Vec<u8>,
			age: u32,
			gender: Gender,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let mut patient_record =
				<PatientRecordStorage<T>>::get(&sender).ok_or(Error::<T>::RecordNotFound)?;
			patient_record.patient_id = patient_id;
			patient_record.name = name.clone();
			patient_record.age = age;
			patient_record.gender = gender.clone();

			<PatientRecordStorage<T>>::insert(&sender, patient_record);

			Self::deposit_event(Event::RecordUpdated {
				patient_id,
				name: name.clone(),
				age,
				gender,
				sender,
			});

			Ok(())
		}
	}
}
