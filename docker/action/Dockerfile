ARG RUST_VERSION

FROM public.ecr.aws/infra-blocks/docker-rust-builder-base:${RUST_VERSION} as builder

FROM public.ecr.aws/infra-blocks/docker-rust-runtime-base:${RUST_VERSION}
COPY --from=builder /usr/local/cargo/bin/docker-rust-action-template /usr/local/bin/docker-rust-action-template
ENTRYPOINT ["docker-rust-action-template"]
