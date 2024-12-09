pub trait StateManager {
    fn store_state(&self, workflow_id: &str, state: State) -> Result<()>;
    fn get_state(&self, workflow_id: &str) -> Result<State>;
}
