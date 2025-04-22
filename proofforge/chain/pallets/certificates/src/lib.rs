#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "std")]
pub mod rpc;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::StorageVersion,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;
    use sp_runtime::traits::StaticLookup;
    use codec::{Encode, Decode, MaxEncodedLen};
    use scale_info::TypeInfo;

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Certificate metadata structure
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Certificate<T: Config> {
        /// Certificate ID
        pub id: T::CertificateId,
        /// Certificate owner
        pub owner: T::AccountId,
        /// Certificate issuer
        pub issuer: T::AccountId,
        /// Certificate metadata (stored as JSON string)
        pub metadata: BoundedVec<u8, T::MaxMetadataLength>,
        /// Certificate issuance time
        pub issued_at: T::BlockNumber,
        /// Certificate revocation status
        pub revoked: bool,
        /// Certificate expiry time (0 if no expiry)
        pub expires_at: T::BlockNumber,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// The certificate ID type
        type CertificateId: Member + Parameter + MaxEncodedLen + Copy + From<u32> + Into<u32>;
        
        /// Maximum metadata length
        #[pallet::constant]
        type MaxMetadataLength: Get<u32>;
        
        /// The origin which may issue certificates
        type IssuerOrigin: EnsureOrigin<Self::RuntimeOrigin>;
    }

    #[pallet::storage]
    #[pallet::getter(fn certificates)]
    pub type Certificates<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::CertificateId,
        Certificate<T>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn certificate_count)]
    pub type CertificateCount<T: Config> = StorageValue<_, T::CertificateId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn account_certificates)]
    pub type AccountCertificates<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<T::CertificateId, T::MaxMetadataLength>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A certificate was issued
        CertificateIssued {
            id: T::CertificateId,
            owner: T::AccountId,
            issuer: T::AccountId,
        },
        /// A certificate was revoked
        CertificateRevoked {
            id: T::CertificateId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Certificate already exists
        CertificateAlreadyExists,
        /// Certificate does not exist
        CertificateNotFound,
        /// Certificate is already revoked
        CertificateAlreadyRevoked,
        /// Certificate does not belong to the account
        NotCertificateOwner,
        /// Certificate metadata too long
        MetadataTooLong,
        /// Account certificates list is full
        TooManyCertificates,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Issue a new certificate to an account
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn issue_cert(
            origin: OriginFor<T>,
            recipient: <T::Lookup as StaticLookup>::Source,
            metadata: Vec<u8>,
            expires_at: T::BlockNumber,
        ) -> DispatchResult {
            let issuer = T::IssuerOrigin::ensure_origin(origin)?;
            let recipient = T::Lookup::lookup(recipient)?;
            
            // Validate metadata length
            let bounded_metadata = BoundedVec::<u8, T::MaxMetadataLength>::try_from(metadata)
                .map_err(|_| Error::<T>::MetadataTooLong)?;
            
            // Generate a new certificate ID
            let id = Self::next_certificate_id()?;
            
            // Create a new certificate
            let cert = Certificate {
                id,
                owner: recipient.clone(),
                issuer: issuer.clone(),
                metadata: bounded_metadata,
                issued_at: <frame_system::Pallet<T>>::block_number(),
                revoked: false,
                expires_at,
            };
            
            // Store the certificate
            <Certificates<T>>::insert(id, cert);
            
            // Update account certificates
            <AccountCertificates<T>>::try_mutate(&recipient, |certs| {
                certs.try_push(id).map_err(|_| Error::<T>::TooManyCertificates)
            })?;
            
            // Emit event
            Self::deposit_event(Event::CertificateIssued {
                id,
                owner: recipient,
                issuer,
            });
            
            Ok(())
        }
        
        /// Revoke a certificate
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn revoke_cert(
            origin: OriginFor<T>,
            cert_id: T::CertificateId,
        ) -> DispatchResult {
            let issuer = T::IssuerOrigin::ensure_origin(origin)?;
            
            // Ensure certificate exists
            <Certificates<T>>::try_mutate(cert_id, |cert_opt| {
                let cert = cert_opt.as_mut().ok_or(Error::<T>::CertificateNotFound)?;
                
                // Ensure certificate is not already revoked
                ensure!(!cert.revoked, Error::<T>::CertificateAlreadyRevoked);
                
                // Ensure the caller is the issuer
                ensure!(cert.issuer == issuer, Error::<T>::NotCertificateOwner);
                
                // Revoke the certificate
                cert.revoked = true;
                
                // Emit event
                Self::deposit_event(Event::CertificateRevoked { id: cert_id });
                
                Ok(())
            })
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get the next certificate ID
        fn next_certificate_id() -> Result<T::CertificateId, DispatchError> {
            <CertificateCount<T>>::try_mutate(|id| {
                let current_id = *id;
                *id = id.checked_add(&T::CertificateId::from(1)).ok_or(Error::<T>::CertificateAlreadyExists)?;
                Ok(current_id)
            })
        }
        
        /// Get all certificates for an account
        pub fn get_account_certificates(account: &T::AccountId) -> Vec<Certificate<T>> {
            <AccountCertificates<T>>::get(account)
                .iter()
                .filter_map(|id| <Certificates<T>>::get(id))
                .collect()
        }
        
        /// Check if a certificate is valid
        pub fn is_certificate_valid(cert_id: T::CertificateId) -> bool {
            if let Some(cert) = <Certificates<T>>::get(cert_id) {
                let current_block = <frame_system::Pallet<T>>::block_number();
                !cert.revoked && (cert.expires_at == T::BlockNumber::from(0) || cert.expires_at > current_block)
            } else {
                false
            }
        }
    }
} 