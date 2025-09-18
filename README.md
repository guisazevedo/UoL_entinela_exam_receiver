# SENTINELA EXAM RECEIVER

## 📑 Table of Contents
1. [Project Overview](#project-overview)
2. [Features](#features)
3. [Architecture & Tech Stack](#architecture--tech-stack)
4. [Installation](#installation)
5. [Local Development](#local-development)
6. [Configuration](#configuration)
7. [Quality Assurance](#quality-assurance)
8. [Testing](#testing)
9. [Logging](#logging)
10. [CI/CD](#cicd)
11. [Project Structure](#project-structure)

---

## 1. 📋 Project Overview
Sentinela Exam Receiver is a backend service written in Rust for ingesting and processing medical exam payloads, integrating with GCP (Cloud Storage, Pub/Sub), and supporting modern DevOps and quality practices. It is suitable for PoC and production deployments.

> This service acts as the entrypoint for all exams into the system, following the Gateway component.

## 2. 🛠️ Features
- Receives and processes XRay and ECG exam payloads
- Integrates with Google Cloud Storage and Pub/Sub
- Modular service architecture for extensibility
- Structured logging for traceability
- Health check endpoint
- Dockerized for easy deployment
- SonarQube integration for code quality
- CI/CD pipeline with GitHub Actions

## 3. 🏗️ Architecture & Tech Stack
- **Language:** Rust
- **Framework:** Axum (async web server)
- **Cloud:** Google Cloud Platform (GCS, Pub/Sub)
- **Containerization:** Docker
- **Quality:** SonarQube
- **CI/CD:** GitHub Actions
- **Logging:** log crate
- **Other:** anyhow (error handling), Arc (thread safety)

## 4. ⚙️ Installation
- **Rust Toolchain:** Stable (recommended: latest stable, e.g. 1.70+)
- **Dependencies:**
  - Install via `cargo build`
  - See `Cargo.toml` for all dependencies
- **Docker:**
  - Install Docker Desktop (https://www.docker.com/products/docker-desktop)

## 5. 🧑‍💻 Local Development
- **Run with Cargo:**
  ```sh
  cargo watch -x clippy -x run
  ```
- **Build & Run Docker Locally:**
  ```sh
  docker build -t sentinela_exam_receiver .
  docker run --env-file .env -p 8080:8080 sentinela_exam_receiver
  ```

## 6. 📝 Configuration
- **Environment Variables:**
  - Use a `.env` file for local development
  - Required variables: GCP credentials, Pub/Sub topic, GCS bucket, etc
- **Config Profiles:**
  - Local, dev, prod supported 

## 7. 🧪 Quality Assurance
- **SonarQube:**
  - Integrated for static analysis and code quality
  - Run analysis via CI/CD pipeline

## 8. 🧪 Testing
- **Unit Tests:**
  - Run with `cargo test`
  - Focus on business logic; Rust's type system covers much of the boilerplate
  - Not aiming for 100% coverage; integration/E2E tests are handled separately
- **Integration/E2E:**
  - Managed outside this repo - in Postman

## 9. 📜 Logging
- Uses the `log` crate for structured logging
- Logs are essential for debugging, monitoring, and problem discovery
- All major processing steps and errors are logged

## 10. 🚀 CI/CD
- **GitHub Actions Workflow:**
  - Build, test, Docker image publish
  - SonarQube push
  - GCP deployment
- **Branch Policy:**
  - All changes via Pull Request (PR)
  - PRs require CODEOWNERS approval

## 11. 📂 Project Structure
- `src/` - Main source code
  - `main.rs` - Application entry point
  - `models/` - Data models (e.g., exam payloads)
  - `routes/` - HTTP route handlers
  - `services/` - Business logic/services (e.g., exam processing)
- `Dockerfile` - Container build instructions
- `.gitignore` / `.dockerignore` - Ignore rules for Git/Docker
- `Cargo.toml` / `Cargo.lock` - Rust dependencies
- `README.md` - Project documentation

---

© SwissAnalytica — All rights reserved.
