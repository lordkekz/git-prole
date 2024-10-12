use command_error::CommandExt;
use miette::IntoDiagnostic;
use test_harness::setup_repo_multiple_remotes;
use test_harness::GitProle;

#[test]
fn config_remotes() -> miette::Result<()> {
    let prole = GitProle::new()?;

    setup_repo_multiple_remotes(&prole, "my-remotes/my-repo", "my-repo")?;

    prole.write_config(
        r#"
        remotes = [
            "a"
        ]
        "#,
    )?;

    prole
        .cd_cmd("my-repo")
        .arg("convert")
        .status_checked()
        .into_diagnostic()?;

    assert_eq!(prole.current_branch_in("my-repo/main")?, "main");
    assert_eq!(prole.current_branch_in("my-repo/a")?, "a");
    assert_eq!(prole.upstream_for_branch_in("my-repo/a", "a")?, "a/a");

    Ok(())
}
