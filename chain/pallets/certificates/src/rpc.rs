use codec::Codec;
use jsonrpsee::{
    core::RpcResult,
    proc_macros::rpc,
    types::error::{CallError, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

use sp_std::vec::Vec;

#[rpc(client, server)]
pub trait CertificatesApi<BlockHash, AccountId, CertificateId, Certificate> {
    #[method(name = "certificates_getAccountCertificates")]
    fn get_account_certificates(
        &self,
        account: AccountId,
        at: Option<BlockHash>,
    ) -> RpcResult<Vec<Certificate>>;

    #[method(name = "certificates_isCertificateValid")]
    fn is_certificate_valid(
        &self,
        cert_id: CertificateId,
        at: Option<BlockHash>,
    ) -> RpcResult<bool>;
}

/// A struct that implements the `CertificatesApi`.
pub struct Certificates<C, Block> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, Block> Certificates<C, Block> {
    /// Create new `Certificates` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

/// Error type of this RPC api.
pub enum Error {
    /// The call to runtime failed.
    RuntimeError,
}

impl From<Error> for i32 {
    fn from(e: Error) -> i32 {
        match e {
            Error::RuntimeError => 1,
        }
    }
}

impl<C, Block, AccountId, CertificateId, Certificate>
    CertificatesApiServer<<Block as BlockT>::Hash, AccountId, CertificateId, Certificate>
    for Certificates<C, Block>
where
    Block: BlockT,
    C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
    C::Api: CertificatesRuntimeApi<Block, AccountId, CertificateId, Certificate>,
    AccountId: Codec + Send + Sync + 'static,
    CertificateId: Codec + Send + Sync + 'static,
    Certificate: Codec + Send + Sync + 'static,
{
    fn get_account_certificates(
        &self,
        account: AccountId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Vec<Certificate>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        api.get_account_certificates(&at, account)
            .map_err(|e| {
                CallError::Custom(ErrorObject::owned(
                    Error::RuntimeError.into(),
                    "Unable to get account certificates.",
                    Some(format!("{:?}", e)),
                ))
                .into()
            })
    }

    fn is_certificate_valid(
        &self,
        cert_id: CertificateId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<bool> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        api.is_certificate_valid(&at, cert_id)
            .map_err(|e| {
                CallError::Custom(ErrorObject::owned(
                    Error::RuntimeError.into(),
                    "Unable to check certificate validity.",
                    Some(format!("{:?}", e)),
                ))
                .into()
            })
    }
}

#[sp_api::api]
pub trait CertificatesRuntimeApi<Block: BlockT, AccountId, CertificateId, Certificate> {
    fn get_account_certificates(account: AccountId) -> Vec<Certificate>;
    fn is_certificate_valid(cert_id: CertificateId) -> bool;
} 