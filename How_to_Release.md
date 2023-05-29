# Creating a new release

This is a guide how to create a new ruff release

1. Update the version with `rg 0.0.269 --files-with-matches | xargs sed -i 's/0.0.269/0.0.270/g'`
2. Update [BREAKING_CHANGES.md](BREAKING_CHANGES.md)
3. Create a PR with the version and BREAKING_CHANGES.md updated
4. Merge the PR
5. Run the release workflow with the tag (without starting `v`) as input. Make sure main has your merged PR as last commit
6. The release workflow will do the following:
   1. Build all the assets. If this fails (even though we tested in step 4), we haven’t tagged or uploaded anything, you can restart after pushing a fix
   2. Upload to pypi
   3. Create a git tag (from pyproject.toml) and push git tag. We create the git tag only here because we can't change it ([#4468](https://github.com/charliermarsh/ruff/issues/4468)), so we want to make sure everything up to and including publishing to pypi worked
   4. Attach artifacts to draft GitHub release
   5. Trigger downstream repositories. This can fail without causing fallout, it is possible (if inconvenient) to trigger the downstream jobs manually
7. Create release notes in GitHub UI (https://github.com/charliermarsh/ruff/releases/new)
8. If needed, [update the schemastore](https://github.com/charliermarsh/ruff/blob/main/scripts/update_schemastore.py)
9. If needed, update ruff-lsp and ruff-vscode
