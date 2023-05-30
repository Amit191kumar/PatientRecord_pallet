// lib.rs

use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

type PatientId<T> = <T as system::Trait>::AccountId;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct PatientRecord<AccountId> {
    name: Vec<u8>,
    age: u8,
    gender: Gender,
    // Include other fields relevant to patient records
    // For example, medical history, medications, etc.
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Other,
}

decl_storage! {
    trait Store for Module<T: Trait> as PatientRecordModule {
        PatientRecords get(fn patient_record): map hasher(twox_64_concat) PatientId<T> => Option<PatientRecord<PatientId<T>>>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        NewRecordCreated(AccountId),
        RecordUpdated(AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        RecordNotFound,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn create_record(origin, name: Vec<u8>, age: u8, gender: Gender) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!<PatientRecords<T>>::contains_key(&sender), Error::<T>::RecordNotFound);

            let new_record = PatientRecord {
                name,
                age,
                gender,
            };

            <PatientRecords<T>>::insert(&sender, new_record.clone());

            Self::deposit_event(RawEvent::NewRecordCreated(sender));

            Ok(())
        }

        #[weight = 10_000]
        pub fn update_record(origin, name: Vec<u8>, age: u8, gender: Gender) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            let mut patient_record = <PatientRecords<T>>::get(&sender).ok_or(Error::<T>::RecordNotFound)?;

            patient_record.name = name;
            patient_record.age = age;
            patient_record.gender = gender;

            <PatientRecords<T>>::insert(&sender, patient_record.clone());

            Self::deposit_event(RawEvent::RecordUpdated(sender));

            Ok(())
        }
    }
}
