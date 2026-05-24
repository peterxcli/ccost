# Security Policy

`ccost` reads local AI coding session logs, which may contain sensitive prompts, file paths, stack traces, and source context.

## Reporting a vulnerability

Please do not include secrets, private logs, or exploit details in a public issue.

If GitHub private vulnerability reporting is enabled for this repository, use that channel. Otherwise, open a public issue with a short non-sensitive summary and ask the maintainer to arrange a private disclosure channel.

## Supported versions

Security fixes are expected to target the latest released version.

## Local data

`ccost` should not upload session data or make network calls. If you find behavior that violates that expectation, report it as a security issue.
