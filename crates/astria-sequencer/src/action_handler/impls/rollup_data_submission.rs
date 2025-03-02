use astria_core::protocol::transaction::v1::action::RollupDataSubmission;
use astria_eyre::eyre::{
    ensure,
    Result,
};
use async_trait::async_trait;
use cnidarium::StateWrite;
use tracing::{
    instrument,
    Level,
};

use crate::action_handler::ActionHandler;

#[async_trait]
impl ActionHandler for RollupDataSubmission {
    #[instrument(skip_all, err(level = Level::DEBUG))]
    async fn check_stateless(&self) -> Result<()> {
        // TODO: do we want to place a maximum on the size of the data?
        // https://github.com/astriaorg/astria/issues/222
        ensure!(
            !self.data.is_empty(),
            "cannot have empty data for sequence action"
        );
        Ok(())
    }

    async fn check_and_execute<S: StateWrite>(&self, _state: S) -> Result<()> {
        Ok(())
    }
}
