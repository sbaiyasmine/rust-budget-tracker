# Rust Budget Tracker

A simple Command Line Interface (CLI) application written in Rust to manage multiple budgets and track financial transactions.

## Overview
Rust Budget Tracker helps you:
- Create, rename and delete multiple budgets.
- Add, edit and remove transactions (expenses and incomes).
- View all transactions for a budget and see the remaining balance.
- Handle invalid inputs with clear error messages.

## Features
- Multiple named budgets stored in memory.
- Transaction operations: add / modify / delete / list.
- Balance calculation per budget.
- Interactive CLI using `dialoguer` (optional TUI later).

## Tech Stack
- Rust + Cargo
- Libraries: dialoguer, console, tui (optional), rusqlite (for future persistence)

## Quick Start

1. Clone the repository:
```bash
git clone https://github.com/sbaiyasmine/rust-budget-tracker.git
cd rust-budget-tracker
