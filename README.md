# docker-rust-action-template
[![Build](https://github.com/infrastructure-blocks/docker-rust-action-template/actions/workflows/build.yml/badge.svg)](https://github.com/infrastructure-blocks/docker-rust-action-template/actions/workflows/build.yml)
[![Release](https://github.com/infrastructure-blocks/docker-rust-action-template/actions/workflows/release.yml/badge.svg)](https://github.com/infrastructure-blocks/docker-rust-action-template/actions/workflows/release.yml)
[![Git Tag](https://github.com/infrastructure-blocks/docker-rust-action-template/actions/workflows/git-tag.yml/badge.svg)](https://github.com/infrastructure-blocks/docker-rust-action-template/actions/workflows/git-tag.yml)
[![Trigger Update From Template](https://github.com/infrastructure-blocks/docker-rust-action-template/actions/workflows/trigger-update-from-template.yml/badge.svg)](https://github.com/infrastructure-blocks/docker-rust-action-template/actions/workflows/trigger-update-from-template.yml)

[//]: # ([![Update From Template]&#40;https://github.com/infrastructure-blocks/docker-rust-action-template/actions/workflows/update-from-template.yml/badge.svg&#41;]&#40;https://github.com/infrastructure-blocks/docker-rust-action-template/actions/workflows/update-from-template.yml&#41;)

A template repository for GitHub Actions hosted as Docker images running Rust binaries.

## Instantiation checklist

- Do a global search & replace for `docker-rust-action-template` and replace it with the name of your repository
- Remove the [trigger update from template workflow](.github/workflows/trigger-update-from-template.yml)
- Remove the `Trigger Update From Template` status badge
- Uncomment the `Update From Template` status badge.
- Replace the self-test section of the [build workflow](.github/workflows/build.yml)
- Replace the summary and the action usage section in this readme.
- Prepare the [changelog](CHANGELOG.md) for the first version of the module that will be released.

## Inputs

|     Name      | Required | Description      |
|:-------------:|:--------:|------------------|
| example-input |   true   | A useless input. |

## Outputs

|      Name      | Description                    |
|:--------------:|--------------------------------|
| example-output | An equivalently useless output |

## Permissions

|     Scope     | Level | Reason   |
|:-------------:|:-----:|----------|
| pull-requests | read  | Because. |

## Usage

```yaml
name: Template Usage

on:
  push: ~

# The required permissions.
permissions:
  pull-requests: read

# The suggested concurrency controls.
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  example-job:
    runs-on: ubuntu-22.04
    steps:
      - uses: docker://public.ecr.aws/infrastructure-blocks/docker-rust-action-template:v1
```
