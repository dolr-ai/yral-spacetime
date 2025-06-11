use candid::Principal;

pub(crate) const AUTH_JWT_ISS: &str = "https://auth.yral.com";
// TODO: Replace with the actual principal of SSR service
pub const YRAL_SSR_TRUSTED_PRINCIPAL: Principal = Principal::anonymous();

pub const OFFCHAIN_AGENT_TRUSTED_PRINCIPAL: &str =
    "o3hhp-ur64t-rf42j-pgbx3-5ahu7-kbema-zhnie-uemnz-tzswh-bpgha-bqe";
