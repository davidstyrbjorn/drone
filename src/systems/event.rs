use crate::prelude::*;

// Will run for each entity with WantsToLog component
#[system(for_each)]
#[read_component(WantsToLog)]
pub fn event(
    entity: &Entity,
    want_log: &mut WantsToLog,
    #[resource] event_log: &mut EventLog,
    commands: &mut CommandBuffer,
) {
    // Add event to event log
    event_log.messages.push(want_log.log_entry.clone());

    // We have handled this message
    commands.remove(*entity);
}
