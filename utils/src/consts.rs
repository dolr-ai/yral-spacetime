use candid::Principal;

pub(crate) const AUTH_JWT_ISS: &str = "https://auth.yral.com";

// TODO: Replace with the actual principal of SSR service
pub const YRAL_SSR_TRUSTED_PRINCIPAL: Principal = Principal::anonymous();

/// "hufxs-hfs2i-s5oi4-tjixu-zec4z-jjhdj-pegob-ykuje-5p4wa-joiru-2qe"
pub const OFFCHAIN_AGENT_TRUSTED_PRINCIPAL: Principal = Principal::from_slice(&[
    178, 210, 37, 215, 35, 147, 74, 47, 76, 144, 92, 202, 82, 113, 165, 228, 51, 131, 133, 81, 36,
    235, 249, 96, 37, 200, 141, 53, 2,
]);
