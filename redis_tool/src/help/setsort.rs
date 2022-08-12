#[derive(Debug, Clone)]
pub struct SetSortHelp {}

pub trait Help {
    fn help_zscan(&self);
    fn help_zunionstore(&self);
    fn help_zscore(&self);
    fn help_zrevrange(&self);
    fn help_zremrangebyscore(&self);
    fn help_zremrangebyrank(&self);
    fn help_zremrangebylex(&self);
    fn help_zrem(&self);
    fn help_zrank(&self);
    fn help_zrangebyscore(&self);
    fn help_zrangebylex(&self);
    fn help_zrange(&self);
    fn help_zlexcount(&self);
    fn help_zinterstore(&self);
    fn help_zincrby(&self);
    fn help_zcount(&self);
    fn help_zcard(&self);
    fn help_zadd(&self);
}

impl Help for SetSortHelp {
    fn help_zscan(&self) {}
    fn help_zunionstore(&self) {}
    fn help_zscore(&self) {}
    fn help_zrevrange(&self) {}
    fn help_zremrangebyscore(&self) {}
    fn help_zremrangebyrank(&self) {}
    fn help_zremrangebylex(&self) {}
    fn help_zrem(&self) {}
    fn help_zrank(&self) {}
    fn help_zrangebyscore(&self) {}
    fn help_zrangebylex(&self) {}
    fn help_zrange(&self) {}
    fn help_zlexcount(&self) {}
    fn help_zinterstore(&self) {}
    fn help_zincrby(&self) {}
    fn help_zcount(&self) {}
    fn help_zcard(&self) {}
    fn help_zadd(&self) {}
}
