use command_error::CommandExt;
use expect_test::expect;
use test_harness::GitProle;
use test_harness::WorktreeState;

#[test]
fn add_by_path() {
    let prole = GitProle::new().unwrap();
    prole.setup_worktree_repo("my-repo").unwrap();

    prole
        .cd_cmd("my-repo/main")
        // Weird But Okay
        .args(["add", "../../puppy"])
        .status_checked()
        .unwrap();

    prole
        .repo_state("my-repo")
        .worktrees([
            WorktreeState::new_bare(),
            WorktreeState::new("main").branch("main"),
            WorktreeState::new("../puppy")
                .branch("puppy")
                .upstream("main")
                .file(
                    "README.md",
                    expect![[r#"
                        puppy doggy
                    "#]],
                ),
        ])
        .assert();
}
