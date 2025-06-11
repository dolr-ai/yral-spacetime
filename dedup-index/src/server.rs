use spacetimedb::{ReducerContext, Table, Timestamp, TryInsertError};

#[spacetimedb::table(name = dedup_index, public)]
pub struct VideoHash {
    #[unique]
    video_id: String,
    #[index(btree)]
    hash: String,
    created_at: Timestamp,
}

#[spacetimedb::table(name = unique_hash, public)]
pub struct UniqueHash {
    #[primary_key]
    hash: String,
    video_id: String,
}

#[spacetimedb::reducer]
pub fn add(
    ctx: &ReducerContext,
    hash: &str,
    video_id: &str,
    created_at: Timestamp,
) -> utils::Result<()> {
    utils::validate_sender_identity(ctx, utils::consts::OFFCHAIN_AGENT_TRUSTED_PRINCIPAL)?;

    ctx.db.dedup_index().insert(VideoHash {
        hash: hash.to_string(),
        video_id: video_id.to_string(),
        created_at,
    });

    let res = ctx.db.unique_hash().try_insert(UniqueHash {
        hash: hash.to_string(),
        video_id: video_id.to_string(),
    });

    match res {
        Ok(..) | Err(TryInsertError::UniqueConstraintViolation(..)) => Ok(()),
        _ => {
            unreachable!("This should never happen")
        }
    }
}
