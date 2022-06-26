use crate::prelude::*;

const MAX_LOG_LENGTH: usize = 6;

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
    event_log.messages.push_front(want_log.log_entry.clone());
    if event_log.messages.len() > MAX_LOG_LENGTH {
        // clamp size
        event_log.messages.pop_back();
    }

    // We have handled this message
    commands.remove(*entity);
}
