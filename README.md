# Snyk Zed Extension

This extension aims to add Snyk LSP support to the Zed IDE.
This is not an officially supported extension and simply a community effort, it is mainly a pet project for the time being.

## Current Status
The LSP extension in its current state can be imported into Zed as a local extension for development purposes. The LSP starts up and posts logs yet the initialization parameters are not properly set. This causes OAuth failures due to incorrectly passed tokens.

## Roadmap
- [x] Download uncompressed LSP binary
- [x] Provide LSP to the Zed extension API
- [ ] WIP: Define initialization options
- [ ] WIP: Define settings
- [ ] Inject initialization options into Zed API extension
- [ ] Build custom workflows
- [ ] Slash commands for fancy AI things? 
- [ ] Publish a PR to the Zed Repo to make the first version available
