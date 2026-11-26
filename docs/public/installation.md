---
title: Installation
description: Install the rulelink command on macOS, Linux or Windows and check that it runs.
---

# Installation

This page shows how to install the `rulelink` command. Pick the section for your operating system. Each one is a single command.

> **No release yet.** The first release of Rulelink is not published. Until it is, the commands below fail because the files they download do not exist yet. They are the commands you will run once the first release is out.

## Before you start

- macOS, Linux or Windows on a 64-bit machine. Apple Silicon and Intel Macs, and Linux on x86-64 or ARM64, are supported.
- For the npm option only: Node.js with `npm`.

## macOS and Linux

Run this in a terminal:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/E-Jacques/rulelink/releases/latest/download/rulelink-cli-installer.sh | sh
```

## Windows

Run this in PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/E-Jacques/rulelink/releases/latest/download/rulelink-cli-installer.ps1 | iex"
```

## npm

On any system with Node.js, run:

```bash
npm install -g @rulelink-cli/rulelink-cli
```

## Check the installation

Open a new terminal and run:

```bash
rulelink --version
```

You see the version number you installed. If the terminal says the command is not found, close it and open a new one so it picks up the new location.

## Rulelink Cloud

Rulelink Cloud is coming soon.

## Next steps

- Read the [Introduction](introduction.md) to see how Rulelink works.
- Create your project files with [`rulelink init`](cli/init.md).
- See what exists today and what is announced on the [Roadmap](roadmap.md).
