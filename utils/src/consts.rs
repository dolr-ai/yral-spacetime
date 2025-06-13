use candid::Principal;

pub(crate) const AUTH_JWT_ISS: &str = "https://auth.yral.com";
// TODO: Replace with the actual principal of SSR service
pub const YRAL_SSR_TRUSTED_PRINCIPAL: Principal = Principal::anonymous();

pub const OFFCHAIN_AGENT_TRUSTED_PRINCIPAL: &str =
    "hufxs-hfs2i-s5oi4-tjixu-zec4z-jjhdj-pegob-ykuje-5p4wa-joiru-2qe";
