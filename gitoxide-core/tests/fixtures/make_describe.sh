#!/usr/bin/env bash
set -eu -o pipefail

# `git describe` names a commit after the closest tag, and accepts anything that peels to a commit as input:
# an annotated tag, a tag of a tag, the id of a tag object, a lightweight tag, a branch, or a revision of one of them.
# `tree-tag` is the negative case: it peels to a tree, which Git refuses to describe.
# The baselines record what Git prints, both for annotated tags only and for all tags,
# so tests can compare against Git instead of hardcoding names or abbreviations.
git init -q
git commit -q --allow-empty -m c1
git commit -q --allow-empty -m c2
git commit -q --allow-empty -m c3

# `nested` is an annotated tag of the annotated tag `v1`, i.e. a tag object that has to be peeled twice.
# Both name the same commit as the lightweight tag `light` and the branch `topic`. Git calls that commit `v1`,
# the annotated tag with the later date, whichever of these names is given as the revision.
GIT_COMMITTER_DATE="2022-01-02 00:00:00 +0000" git tag -a v1 -m "tag object" ":/c2"
GIT_COMMITTER_DATE="2022-01-01 00:00:00 +0000" git tag -a nested -m "tag object" v1
git tag light ":/c2"
git branch topic ":/c2"
git tag -a tree-tag -m "tag of a tree" 'HEAD^{tree}'
tag_id=$(git rev-parse v1)

specs=(v1 "$tag_id" nested light topic main HEAD ':/c2' 'v1^{}' 'v1^0' 'v1~0')
for spec in "${specs[@]}"; do
  annotated=$(git describe "$spec")
  printf '%s=%s\n' "$spec" "$annotated" >>describe.git
  all_tags=$(git describe --tags "$spec")
  printf '%s=%s\n' "$spec" "$all_tags" >>describe_tags.git
done

if git describe tree-tag >/dev/null 2>&1; then
  echo "Git must refuse to describe a tag of a tree" >&2
  exit 1
fi
