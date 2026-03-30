# 🧬 Rust AI Regex Assistant CLI

A dual-purpose **Command Line Regex Tool written in Rust** that uses the **Google Gemini API** to either generate complex regular expressions from plain English descriptions, or break down and explain confusing regex patterns into readable terms.

This project is part of my **Rust learning journey (Day 18 Project)**.

---

## 🚀 Features

* **Text-to-Regex:** Type what you want to match in plain English, get the exact regex.
* **Regex-to-Text:** Paste a complex regex, get a step-by-step explanation.
* Uses the Google Gemini 1.5 Flash model for fast responses.
* Secure API key management using `.env`.
* Interactive CLI menu system.

---

## 🛠 Built With

* **Rust**
* `tokio` (Async runtime)
* `reqwest` (HTTP requests)
* `serde_json` (JSON parsing)
* Google Gemini API

---

## 📂 Project Structure

```text
ai_regex_cli_18/
│
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── .env
```

---

## ⚙️ Installation

```bash
git clone [https://github.com/yourusername/rust-learning-journey.git](https://github.com/yourusername/rust-learning-journey.git)
cd ai_regex_cli_18
cargo build
```

---

## 🔑 Setup

Create a `.env` file in the root directory:

```env
GEMINI_API_KEY=your_api_key_here
```

---

## ▶️ Usage

```bash
cargo run
```

Follow the interactive prompts to either generate or explain a regex.

---

## 📸 Example

```text
======================================
 🧬 AI Regex Assistant CLI (Day 18) 
======================================
1. Generate Regex from English
2. Explain an existing Regex

Choose an option (1 or 2): 1

📝 Describe what you want to match: A valid hex color code like #FFF or #FFFFFF

🧠 Thinking...

✅ Generated Regex:

^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$
```

---

## 🧠 Concepts Practiced

* Interactive CLI input/output (`std::io`)
* Conditional prompt engineering
* Handling async HTTP requests
* API integration

---

## 📜 License

MIT License

---

## 👨‍💻 Author

**Khurram Rashid** B.Tech Computer Science Engineering  
Amity University Mumbai
```