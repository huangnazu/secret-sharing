//! # Secret Sharing
//! This tiny crate contains traits which provide generic APIs for secret sharing schemes.

pub trait SecretSharing<Secret, Share> {
    type Error;

    fn split(&self, secret: Secret) -> Vec<Share>;

    fn recover(&self, shares: &[Share]) -> Result<Secret, Self::Error>;
}

pub trait ThresholdSecretSharing<Secret, Share>: SecretSharing<Secret,Share> {
    fn threshold(&self) -> usize;

    fn num_shareholder(&self) -> usize;
}