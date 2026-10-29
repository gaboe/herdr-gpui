use super::*;
use crate::{
    preferences::AgentSort,
    sidebar::{
        agents::sorted_agents,
        search::{Device, Kind, search},
        workspaces::{
            displayed_sidebar_workspace_status, displayed_workspace_status, rail_workspace_status,
            workspace_status,
        },
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
    assert_eq!(
        workspace_status(workspace, &[], true),
        AgentStatus::Done,
        "a workspace without matching agents keeps its daemon status"
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
    for workspace in &mut snapshot.workspaces[3..6] {
        workspace.agent_status = AgentStatus::Done;
    }
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
fn expanded_sidebar_keeps_the_parent_status_when_inline_and_rollup_are_off() {
    let mut snapshot = layout_tests::snapshot(6);
    snapshot.workspaces[3].agent_status = AgentStatus::Idle;
    snapshot.workspaces[4].agent_status = AgentStatus::Blocked;
    snapshot.agents[0].workspace_id = snapshot.workspaces[4].workspace_id.clone();
    snapshot.agents[0].agent_status = AgentStatus::Working;
    let collapsed = HashSet::from([layout_tests::REPO_KEY.to_owned()]);
    let parent = &snapshot.workspaces[3];

    assert_eq!(
        displayed_sidebar_workspace_status(
            &snapshot.workspaces,
            &snapshot.agents,
            parent,
            &collapsed,
            false,
            false,
        ),
        AgentStatus::Idle
    );
    assert_eq!(
        displayed_sidebar_workspace_status(
            &snapshot.workspaces,
            &snapshot.agents,
            parent,
            &collapsed,
            true,
            false,
        ),
        AgentStatus::Blocked,
        "inline rows keep the legacy collapsed-group rollup"
    );
    assert_eq!(
        displayed_sidebar_workspace_status(
            &snapshot.workspaces,
            &snapshot.agents,
            parent,
            &collapsed,
            false,
            true,
        ),
        AgentStatus::Working,
        "the enabled rollup applies when inline rows are disabled"
    );
}

#[test]
fn compact_rail_rolls_up_collapsed_children_only_when_enabled() {
    let mut snapshot = layout_tests::snapshot(6);
    for workspace in &mut snapshot.workspaces[3..6] {
        workspace.agent_status = AgentStatus::Done;
    }
    let parent_id = snapshot.workspaces[3].workspace_id.clone();
    let child_id = snapshot.workspaces[4].workspace_id.clone();
    snapshot
        .agents
        .retain(|agent| agent.workspace_id != parent_id);
    snapshot.agents[0].workspace_id = child_id;
    snapshot.agents[0].agent_status = AgentStatus::Working;
    let collapsed = HashSet::from([layout_tests::REPO_KEY.to_owned()]);
    let parent = &snapshot.workspaces[3];

    assert_eq!(
        rail_workspace_status(
            &snapshot.workspaces,
            &snapshot.agents,
            parent,
            &collapsed,
            true,
        ),
        AgentStatus::Working
    );
    assert_eq!(
        rail_workspace_status(
            &snapshot.workspaces,
            &snapshot.agents,
            parent,
            &collapsed,
            false,
        ),
        AgentStatus::Done
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
