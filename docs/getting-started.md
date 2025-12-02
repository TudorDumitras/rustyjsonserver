# Getting Started

This guide walks you through setting up **RustyJSONServer**, creating your first config file, and running a mock API in minutes.

---

## 1. [Install RustyJSONServer](./installation.md)

---

## 2. Create a simple config

Create **`config.json`**:

```json
{
  "port": 8080,
  "resources": [
    {
      "path": "hello",
      "methods": [
        {
          "method": "GET",
          "response": {
            "status": 200,
            "body": "Hello World!"
          }
        }
      ]
    }
  ]
}
```

---

## 3. Start the server

* As Binary: `rjserver.exe serve --config config.json`

* As Docker image: build image `docker build -t <IMAGE_NAME> .` , run image  `docker run --rm -p 8080:8080 <IMAGE_NAME>`

Access:

```
GET http://localhost:8080/hello
```

---

## 4. Hot Reload

RustyJSONServer reloads automatically when JSON or script files change. (unless --no-watch flag is used)

---
