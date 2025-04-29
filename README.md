<a id="readme-top"></a>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h3 align="center">Pyresky</h3>

  <p align="center">
        Firesky.tv clone in Rust
    <br />
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#motivation">Motivation</a></li>
        <li><a href="#core-functionality">Core Functionality</a></li>
        <li><a href="#technical-highlights">Technical Highlights</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

PyreSky is a backend service built in Rust, designed to aggregate and broadcast social media feeds from the BlueSky network in real-time via websockets. It serves as an exploration into creating a high-performance alternative for observing BlueSky feed data, inspired by clients like firesky.tv.

### Motivation

While various clients exist for interacting with BlueSky (which uses the AT Protocol), this project was undertaken with a specific focus on backend performance and efficiency. The primary goal was to leverage Rust's capabilities for concurrency and speed to build a service that could:

1.  Maintain a persistent connection to the BlueSky API's event stream (jetstream).
2.  Process incoming feed data efficiently.
3.  Broadcast relevant updates simultaneously to a potentially large number of connected client applications via websockets with minimal latency.

The hypothesis was that Rust's performance characteristics and strong type system could enable the creation of a more robust and scalable backend compared to implementations in some other languages, particularly concerning managing many concurrent websocket connections.

### Core Functionality

*   **BlueSky API Connection:** Establishes an authenticated connection to the BlueSky API's jetstream endpoint (jetstream1.us-east.bsky.network/) to receive a real-time stream of repository events (posts, likes, reposts, etc.).
*   **Event Processing:** Parses the incoming stream of JSON messages from the jetstream, extracting relevant information about posts and other feed activities.
*   **WebSocket Broadcasting:** Runs a WebSocket server that allows client applications (e.g., custom frontends) to connect. PyreSky then pushes the processed feed updates out to all connected clients in near real-time.
*   **Efficiency:** Aims to minimize resource consumption (CPU, memory) while maximizing the speed at which updates are relayed from the BlueSky network to the end clients.

### Technical Highlights

*   **Language:** Built entirely in **Rust**, chosen for its performance, memory safety, and excellent concurrency support.
*   **Asynchronous Runtime:** Utilizes Rust's asynchronous programming features and the tokio runtime to efficiently handle network I/O for both the BlueSky API connection and the client-facing WebSocket server.
*   **WebSockets:** Employs the Rust WebSocket libraries `tokio-tungstenite` for connecting to the jetstream and `axum` for managing client connections and broadcasting messages.
*   **API Interaction:** Involves understanding and interacting with the intricacies of Bluesky's post object format.

This project serves as a practical application of Rust for building concurrent network services and demonstrates handling real-time data streams and API interactions.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.
* rust
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
### Installation

_Below is an example of how you can instruct your audience on installing and setting up your app. This template doesn't rely on any external dependencies or services._

1. Clone the repo
   ```sh
   git clone https://github.com/AndrewMcDonald-Dev/PyreSky
    ```
2. Run the server in release mode (recommended for production)
   ```sh
    cargo run --release
   ```
3. Access the server at http://localhost:8080

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

![Example screenshot](https://imgur.com/a/wHFedzO)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Andrew McDonald - AndrewMcDonald.Dev@gmail.com

Project Link: [https://github.com/AndrewMcDonald-Dev/PyreSky](https://github.com/AndrewMcDonald-Dev/PyreSky)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Firesky.tv](https://firesky.tv/)
* [Axum](https://github.com/tokio-rs/axum)
* [tera](https://github.com/Keats/tera)

<p align="right">(<a href="#readme-top">back to top</a>)</p>
