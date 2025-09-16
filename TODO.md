# Multi-LLM Agent Task Runner MCP - TODO

## ✅ Completed Features

### Core Infrastructure
- [x] Convert test runner to agent task runner architecture
- [x] Add parallel task processing using tokio and futures
- [x] Implement UUID-based unique file naming
- [x] Create agent output directory management

### PR Review Tool
- [x] Implement `pr_review` tool with no arguments
- [x] Create parallel agent simulation (claude and gpt4)
- [x] Add 1-2 second delay simulation for realistic agent work
- [x] Generate dummy PR review content in output files
- [x] Return file paths in tool response

### Legacy Support
- [x] Maintain compatibility with `run_rspec` tool
- [x] Keep existing file path validation and parsing logic

## 🚧 Future Enhancements

### High Priority
- [ ] Add real LLM agent integration (replace dummy content)
- [ ] Implement configurable agent types and models
- [ ] Add PR context fetching (from GitHub API or git)

### Medium Priority  
- [ ] Add more agent task types (code review, security scan, etc.)
- [ ] Implement result aggregation and summary generation
- [ ] Add configurable output formats (JSON, markdown, etc.)

### Low Priority
- [ ] Add agent task queuing and rate limiting
- [ ] Implement persistent task history and caching
- [ ] Add web UI for task monitoring

## 🐛 Known Issues
- [ ] Build system needs nix environment activation
- [ ] Server binary name needs update in flake.nix