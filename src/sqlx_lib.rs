use async_trait::async_trait;
use chrono::NaiveDateTime;
use serenity::all::UserId;
use sqlx::{any::AnyQueryResult, prelude::FromRow, Database, Pool};

use crate::level_up_xp;

#[async_trait]
pub trait LevelsManager<Db: Database> {
    async fn leaderboard(
        pool: &Pool<Db>,
        users: &[i64],
        page: i64,
    ) -> sqlx::Result<Vec<LeaderboardRow>>;

    async fn get_user_rank(
        pool: &Pool<Db>,
        user_id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<i64>>;

    async fn rank_row(
        pool: &Pool<Db>,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<RankRow>>;

    async fn xp_row(pool: &Pool<Db>, id: impl Into<UserId> + Send) -> sqlx::Result<Option<XpRow>>;

    async fn level_state_row(
        pool: &Pool<Db>,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<LevelStateRow>>;

    async fn save_level_state_row(
        pool: &Pool<Db>,
        row: LevelStateRow,
    ) -> sqlx::Result<AnyQueryResult>;
}

pub trait LevelsRow {
    fn user_id(&self) -> UserId;

    fn xp(&self) -> i32;

    fn level(&self) -> i32;

    fn total_xp(&self) -> i64;

    fn message_count(&self) -> i64;

    fn last_xp(&self) -> NaiveDateTime;
}

#[derive(FromRow)]
pub struct LeaderboardRow {
    pub id: i64,
    pub xp: i32,
    pub level: i32,
    pub message_count: i64,
}

impl LevelsRow for LeaderboardRow {
    fn user_id(&self) -> UserId {
        UserId::new(self.id as u64)
    }

    fn xp(&self) -> i32 {
        self.xp
    }

    fn level(&self) -> i32 {
        self.level
    }

    fn total_xp(&self) -> i64 {
        unimplemented!()
    }

    fn message_count(&self) -> i64 {
        self.message_count
    }

    fn last_xp(&self) -> NaiveDateTime {
        unimplemented!()
    }
}

#[derive(FromRow)]
pub struct RankRow {
    pub xp: i32,
    pub level: i32,
}

impl Default for RankRow {
    fn default() -> Self {
        Self { xp: 0, level: 1 }
    }
}

impl LevelsRow for RankRow {
    fn user_id(&self) -> serenity::all::UserId {
        unimplemented!()
    }

    fn xp(&self) -> i32 {
        self.xp
    }

    fn level(&self) -> i32 {
        self.level
    }

    fn total_xp(&self) -> i64 {
        unimplemented!()
    }

    fn message_count(&self) -> i64 {
        unimplemented!()
    }

    fn last_xp(&self) -> NaiveDateTime {
        unimplemented!()
    }
}

#[derive(FromRow)]
pub struct XpRow {
    pub xp: i32,
    pub level: i32,
    pub total_xp: i64,
}

impl Default for XpRow {
    fn default() -> Self {
        Self {
            xp: 0,
            level: 1,
            total_xp: 0,
        }
    }
}

impl LevelsRow for XpRow {
    fn user_id(&self) -> UserId {
        unimplemented!()
    }

    fn xp(&self) -> i32 {
        self.xp
    }

    fn level(&self) -> i32 {
        self.level
    }

    fn total_xp(&self) -> i64 {
        self.total_xp
    }

    fn message_count(&self) -> i64 {
        unimplemented!()
    }

    fn last_xp(&self) -> NaiveDateTime {
        unimplemented!()
    }
}

#[derive(FromRow)]
pub struct LevelStateRow {
    pub xp: i32,
    pub level: i32,
    pub total_xp: i64,
    pub last_xp: NaiveDateTime,
}

impl LevelStateRow {
    pub fn update_level(&mut self) -> Option<i32> {
        let next_level_xp = level_up_xp(self.level());

        let rand_xp = rand::random_range(15..25);
        self.total_xp += rand_xp as i64;
        self.xp += rand_xp;

        if self.xp >= next_level_xp {
            self.xp -= next_level_xp;
            self.level += 1;
            return Some(self.level);
        };

        None
    }
}

impl Default for LevelStateRow {
    fn default() -> Self {
        Self {
            xp: 0,
            level: 1,
            total_xp: 0,
            last_xp: NaiveDateTime::default(),
        }
    }
}

impl LevelsRow for LevelStateRow {
    fn user_id(&self) -> UserId {
        unimplemented!()
    }

    fn xp(&self) -> i32 {
        self.xp
    }

    fn level(&self) -> i32 {
        self.level
    }

    fn total_xp(&self) -> i64 {
        self.total_xp
    }

    fn message_count(&self) -> i64 {
        unimplemented!()
    }

    fn last_xp(&self) -> NaiveDateTime {
        self.last_xp
    }
}
