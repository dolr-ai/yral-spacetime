use spacetimedb::{ReducerContext, Table, Timestamp};

#[spacetimedb::table(name = dedup_index, public)]
pub struct VideoHash {
    #[unique]
    video_id: String,
    #[index(btree)]
    hash: String,
    created_at: Timestamp,
}

#[spacetimedb::reducer]
pub fn add(ctx: &ReducerContext, hash: String, video_id: String, created_at: Timestamp) {
    ctx.db.dedup_index().insert(VideoHash {
        hash,
        video_id,
        created_at,
    });
}
