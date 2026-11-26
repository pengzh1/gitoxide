use gitoxide_core::repository::commit;
use gix_testtools::TestResult;

fn options(all_tags: bool) -> commit::describe::Options {
    commit::describe::Options {
        all_tags,
        all_refs: false,
        first_parent: false,
        always: false,
        long_format: false,
        statistics: false,
        max_candidates: 10,
        dirty_suffix: None,
    }
}

fn describe(repo: &gix::Repository, rev_spec: &str, all_tags: bool) -> gix::Result<String> {
    let mut out = Vec::new();
    commit::describe(
        repo.clone(),
        Some(rev_spec),
        &mut out,
        &mut Vec::new(),
        options(all_tags),
    )?;
    Ok(String::from_utf8(out).expect("describe only writes names and hashes"))
}

fn open_fixture() -> TestResult<(gix::Repository, std::path::PathBuf)> {
    let fixture = gix_testtools::scripted_fixture_read_only("make_describe.sh")?;
    let repo = gix::open_opts(&fixture, gix::open::Options::isolated())?;
    Ok((repo, fixture))
}

fn assert_matches_git(baseline: &str, all_tags: bool) -> TestResult {
    let (repo, fixture) = open_fixture()?;
    for line in std::fs::read_to_string(fixture.join(baseline))?.lines() {
        let (rev_spec, expected) = line.split_once('=').expect("baseline lines are `<revision>=<name>`");
        let actual = describe(&repo, rev_spec, all_tags)
            .unwrap_or_else(|err| panic!("`{rev_spec}` must be describable like in Git, but failed with: {err}"));
        assert_eq!(
            actual,
            format!("{expected}\n"),
            "`{rev_spec}` is named like Git names it, tag objects included"
        );
    }
    Ok(())
}

#[test]
fn annotated_tags_accept_any_commitish_like_git() -> TestResult {
    assert_matches_git("describe.git", false)
}

#[test]
fn all_tags_accept_any_commitish_like_git() -> TestResult {
    assert_matches_git("describe_tags.git", true)
}

#[test]
fn a_tag_of_a_tree_is_rejected_like_in_git() -> TestResult {
    let (repo, _) = open_fixture()?;
    for all_tags in [false, true] {
        describe(&repo, "tree-tag", all_tags)
            .expect_err("Git refuses to describe a tag that peels to a tree, whatever names are allowed");
    }
    Ok(())
}
