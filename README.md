# sentinela Exam Gateway  
Cloud Run Microservice to receive, validate, and process medical exams

---

This is a microservice of the sentinela diagnostic system.  
It acts as a RESTful service built using [Actix Web](https://actix.rs/), designed to receive medical exam data (e.g., X-rays, ECGs) from external hospitals and route them through a standardized validation, processing, and publishing pipeline.

---

## 🧱 Architecture Overview

- **Receives exams** from external clients via REST
- **Validates**, **transforms**, and **normalizes** incoming data
- **Saves** images/data to **Google Cloud Storage**
- **Publishes** metadata and references to **Google Cloud Pub/Sub**
- **Responds** only after all steps succeed or returns precise error information
- **Deployed via Docker** and **GitHub Actions** to Google Cloud Run to the dev and/or production environments

---

## ✅ Features

- RESTful API for multiple exam types (ECG, X-ray, CT, etc.)
- Rate limiting per hospital
- Virus and malware scanning
- Image validation (type, frontal detection, magic number check)
- Data transformation and resizing
- ECG timeseries normalization
- Cloud Storage export:
    - Images → PNG
    - ECG data → Parquet
- Pub/Sub publishing of processed exam metadata
- Modular and type-specific routing
- Full error tracking and response per step

---

## ⚙️ Technology Stack

- [Rust](https://www.rust-lang.org/)
- [Actix Web](https://actix.rs/)
- [Google Cloud Pub/Sub](https://cloud.google.com/pubsub)
- [Google Cloud Storage](https://cloud.google.com/storage)
- [Docker](https://www.docker.com/)
- [GitHub Actions](https://github.com/features/actions)

---

## 📌 Routes Overview

Each type of exam will be submitted through a dedicated REST endpoint, for example:
- POST: /exam/ecg (under construction)
- POST: /exam/xray (under construction)
- POST: /exam/ct (not implemented yet)
- POST: /exam/mri (not implemented yet)
- POST: /exam/ultrasound (not implemented yet)

These endpoints are individually validated and processed based on exam-specific rules.

---

## 🔁 Processing Pipeline

Each incoming request goes through the following stages:

1. **Rate limiting** (per hospital)
2. **Virus/Malware scan**
3. **Validation**
    - X-ray: image type, frontal detection, magic number
    - ECG: required fields, structure, time series validation
4. **Transformation**
    - X-ray: resize and convert image
    - ECG: normalize and reshape into Parquet
5. **Storage**
    - Upload PNG/Parquet to Google Cloud Storage specific bucket
6. **Publishing**
    - Publish exam metadata to Pub/Sub
7. **Response**
    - Returns `200 OK` only after successful publish and storage
    - Else returns structured error response with details

---

## 🧪 Local Development

To run locally and automatically lint during development:

```bash
cargo watch -x 'clippy'
```  

---

## ✅ Testing
 
Unit tests in respective modules.  
Run all tests:
```bash
cargo test
```
Run specific test:
```bash
cargo test <test_name>
```
To run all tests in a specific module, use:
```bash
cargo test <module_name>::
```

---

## 🚀 Deployment

Deployment is automated via GitHub Actions:
- Push to main runs all unit tests
- Builds Docker image
- Deploys to Google Cloud Run if tests pass

---

## 🐳 Docker

To build and run locally using Docker:

# ??REVIEW??
```bash 
docker .....
docker .....
```

## ❌ Error Handling

Each pipeline stage can return specific error responses:
- 429 – Rate limit exceeded
- 400 – Data validation failure
- 415 – Unsupported media type
- 422 – Transformation failure
- 500 – Cloud Storage or Pub/Sub failure

## 📄 License
???
