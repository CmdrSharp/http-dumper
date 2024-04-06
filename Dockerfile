################################
#### Runtime
FROM alpine:3.19 as runtime

# User Input
ENV RUST_LOG 'http_dumper=debug,tower_http=debug'
ENV HTTP_DUMPER_ARGS ''

WORKDIR /app

# Create the non-root user
RUN addgroup -S dumper && adduser -S dumper -G dumper -D

# Don't touch these
ENV LC_COLLATE en_US.UTF-8
ENV LC_CTYPE UTF-8
ENV LC_MESSAGES en_US.UTF-8
ENV LC_MONETARY en_US.UTF-8
ENV LC_NUMERIC en_US.UTF-8
ENV LC_TIME en_US.UTF-8
ENV LC_ALL en_US.UTF-8
ENV LANG en_US.UTF-8

# Install CURL, used for health checking
RUN apk add --update --no-cache curl

# Copy the binary
COPY target/x86_64-unknown-linux-musl/release/http_dumper /usr/local/bin/http_dumper
RUN chmod +x /usr/local/bin/http_dumper
RUN chown dumper:dumper /usr/local/bin/http_dumper

# Run as non-root
USER http_dumper
CMD ["/usr/local/bin/http_dumper"]
