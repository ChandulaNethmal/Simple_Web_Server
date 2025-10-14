# Simple Rust Web Server 🌐
This project is an extension of the single-threaded web server implemented in Chapter 20 of The Rust Programming Language (The Book). It has been enhanced with basic routing and a proof-of-concept login feature.

## ✨ Features Implemented
- Thread Pool: Uses a custom ThreadPool to handle multiple concurrent requests, preventing the server from blocking.

- Basic Routing: Serves different HTML pages based on the requested URL path.

```
/ (Home)

/about

/contact

```

- Basic Login Functionality: Implements simple credential checking using POST requests and serves a protected page on success.

```
Login Path: /login

Success Path: /secret
```

Hardcoded Credentials: user / pass