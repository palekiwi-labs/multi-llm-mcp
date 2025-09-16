# Multi-LLM Agent Task Runner MCP

A Model Context Protocol (MCP) server that runs parallel agent tasks. Supports running multiple LLM agents concurrently for tasks like PR reviews.

## Features

- **PR Review Tool**: Runs multiple parallel agents to review pull requests
- **Legacy RSpec Support**: Maintains compatibility with RSpec test running
- **Parallel Processing**: Executes agent tasks concurrently using Tokio
- **File Output**: Agents generate output files with their results

## Tools

1. **pr_review**: Runs parallel PR review with multiple LLM agents (no arguments)
2. **run_rspec**: Runs RSpec tests for a specific file (legacy compatibility)
