#[derive(Debug, Clone)]
pub struct SetHelp {}

//todo: 
pub trait Help {
    fn help_sadd(&self);
    fn help_scard(&self);
    fn help_sdiff(&self);
    fn help_sdiffstore(&self);
    fn help_sinter(&self);
    fn help_sinterstore(&self);
    fn help_sscan(&self);
    fn help_sunionstore(&self);
    fn help_sunion(&self);
    fn help_srem(&self);
    fn help_srandmember(&self);
    fn help_spop(&self);
    fn help_smove(&self);
    fn help_smembers(&self);
    fn help_sismember(&self);
}

impl Help for SetHelp {
    fn help_sadd(&self) {}
    fn help_scard(&self) {}
    fn help_sdiff(&self) {}
    fn help_sdiffstore(&self) {}
    fn help_sinter(&self) {}
    fn help_sinterstore(&self) {}
    fn help_sscan(&self) {}
    fn help_sunionstore(&self) {}
    fn help_sunion(&self) {}
    fn help_srem(&self) {}
    fn help_srandmember(&self) {}
    fn help_spop(&self) {}
    fn help_smove(&self) {}
    fn help_smembers(&self) {}
    fn help_sismember(&self) {}
}
