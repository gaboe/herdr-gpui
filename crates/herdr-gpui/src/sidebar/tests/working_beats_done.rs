use super::*;
use crate::{
    preferences::AgentSort,
    sidebar::{
        agents::sorted_agents,
        search::{Device, Kind, search},
        workspaces::{displayed_workspace_status, workspace_status},
    },
};
use std::collections::HashSet;

fn mixed_snapshot() -> ClientShellSnapshot {
    let mut snapshot = layout_tests::snapshot(1);
    snapshot.workspaces[0].agent_status = AgentStatus::Done;
    snapshot.agents[0].workspace_id = "w0".into();
    snapshot.agents[0].agent_status = AgentStatus::Done;
    let mut working = snapshot.agents[0].clone();
    working.pane_id = "working".into();
    working.agent_status = AgentStatus::Working;
    snapshot.agents.push(working);
    snapshot
}

#[test]
fn workspace_status_keeps_legacy_default_and_can_prefer_working() {
    let snapshot = mixed_snapshot();
    let workspace = &snapshot.workspaces[0];
    assert_eq!(
        workspace_status(workspace, &snapshot.agents, false),
        AgentStatus::Done
    );
    assert_eq!(
        workspace_status(workspace, &snapshot.agents, true),
        AgentStatus::Working
    );
    let mut blocked = snapshot.agents.clone();
    blocked[0].agent_status = AgentStatus::Blocked;
    assert_eq!(
        workspace_status(workspace, &blocked, true),
        AgentStatus::Blocked
    );
}

#[test]
fn collapsed_worktree_and_priority_order_prefer_working_when_enabled() {
    let mut snapshot = mixed_snapshot();
    assert_eq!(
        sorted_agents(&snapshot, AgentSort::Priority, true)
            .first()
            .map(|agent| agent.agent_status),
        Some(AgentStatus::Working)
    );

    snapshot = layout_tests::snapshot(6);
    snapshot.workspaces[3].agent_status = AgentStatus::Done;
    snapshot.workspaces[4].agent_status = AgentStatus::Done;
    let child_id = snapshot.workspaces[4].workspace_id.clone();
    snapshot.agents[0].workspace_id = child_id;
    snapshot.agents[0].agent_status = AgentStatus::Working;
    let collapsed = HashSet::from([layout_tests::REPO_KEY.to_owned()]);
    let parent = &snapshot.workspaces[3];
    assert_eq!(
        displayed_workspace_status(
            &snapshot.workspaces,
            &snapshot.agents,
            parent,
            &collapsed,
            true,
        ),
        AgentStatus::Working
    );
}

#[test]
fn workspace_search_uses_the_configured_status() {
    let snapshot = mixed_snapshot();
    let device = Device {
        id: "local",
        label: "MacBook Pro",
        workspaces: &snapshot.workspaces,
        agents: &snapshot.agents,
        working_beats_done: true,
        notes: None,
    };
    let hit = search("herdr", &[device])
        .into_iter()
        .find(|hit| hit.kind == Kind::Worktree)
        .unwrap();
    assert_eq!(hit.status, Some(AgentStatus::Working));
}
