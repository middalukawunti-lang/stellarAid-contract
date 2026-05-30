use soroban_sdk::{Env, Vec};
use crate::types::MilestoneData;
use crate::storage::{get_campaign, get_milestone};

/// Issue #200 â€“ `get_all_milestones` view function
///
/// Returns all milestones in index order.
/// No authentication required.
/// Handles campaigns with 1â€“5 milestones.
pub fn get_all_milestones(env: &Env) -> Vec<MilestoneData> {
    let campaign = get_campaign(env).expect("campaign not initialized");
    let mut milestones = Vec::new(env);
    for i in 0..campaign.milestone_count {
        if let Some(m) = get_milestone(env, i) {
            milestones.push_back(m);
        }
    }
    milestones
}
