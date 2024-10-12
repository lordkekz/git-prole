use command_error::CommandExt;
use expect_test::expect;
use pretty_assertions::assert_eq;
use test_harness::GitProle;

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

    prole.assert_contents(&[(
        "puppy/README.md",
        expect![[r#"
            puppy doggy
        "#]],
    )]);

    // Last component of the path becomes the branch name.
    assert_eq!(prole.current_branch_in("puppy").unwrap(), "puppy");

    assert_eq!(
        prole.upstream_for_branch_in("puppy", "puppy").unwrap(),
        "main"
    );
}