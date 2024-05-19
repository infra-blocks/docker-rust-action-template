# Contribution Guidelines

## Development

### Setup

This project depends on the following tools:
- [docker](https://docs.docker.com/engine/install/) as the `docker compose` backend
- [docker compose](https://docs.docker.com/compose/install/) to build/run/test the repository's image.

## Releasing

The CI fully automates the release process. The only manual intervention required is to assign a semantic
versioning label to the pull request before merging (this is a required check). Upon merging, the
release process kicks off. It manages a set of semantic versioning git tags,
as described [here](https://github.com/infra-blocks/git-tag-semver-action).

Upon tagging the default branch, jobs to tag docker images with the same tags will kick off.
