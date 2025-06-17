use candid::Principal;

pub(crate) const AUTH_JWT_ISS: &str = "https://auth.yral.com";

/// "npgq4-7gexs-rbaoq-ixyja-cvnir-gtuw6-aazqb-ax3n3-gstsb-bvf5k-mqe"
pub const YRAL_SSR_TRUSTED_PRINCIPAL: Principal = Principal::from_slice(&[
    196, 188, 162, 16, 58, 8, 190, 18, 1, 85, 168, 137, 167, 75, 120, 0, 204, 2, 11, 237, 187, 52,
    167, 32, 134, 165, 234, 153, 2,
]);

/// "hufxs-hfs2i-s5oi4-tjixu-zec4z-jjhdj-pegob-ykuje-5p4wa-joiru-2qe"
pub const OFFCHAIN_AGENT_TRUSTED_PRINCIPAL: Principal = Principal::from_slice(&[
    178, 210, 37, 215, 35, 147, 74, 47, 76, 144, 92, 202, 82, 113, 165, 228, 51, 131, 133, 81, 36,
    235, 249, 96, 37, 200, 141, 53, 2,
]);

/// Number of seconds after which notifications are pruned (30 days).
pub const NOTIFICATION_PRUNE_AFTER_SECS: u64 = 30 * 24 * 3600;
