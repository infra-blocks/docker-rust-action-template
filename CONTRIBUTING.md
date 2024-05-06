# Contribution Guidelines

## Development

### Setup

This project depends on the following tools:
- [docker](https://docs.docker.com/engine/install/) as the `docker compose` backend
- [docker compose](https://docs.docker.com/compose/install/linux/) to build/run/test the repository's image.

### Release Flow

This project's release cycle is fully automated. It leverages the [git-tag-from-semver-increment-workflow](https://github.com/infrastructure-blocks/git-tag-from-semver-increment-workflow)
to accomplish this. Refer to its documentation for usage information.

## Releasing

The CI fully automates the release process. The only manual intervention required is to assign a semantic
versioning label to the pull request before merging (this is a required check). Upon merging, the
release process kicks off. It manages a set of semantic versioning git tags,
as described [here](https://github.com/infrastructure-blocks/git-tag-semver-action).

Upon tagging the default branch, jobs to tag docker images with the same tags will kick off.
