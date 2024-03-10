# Simple Dynamic Update Server for Tauri Written in Rust 🦀 + Rocket 🚀

[Tauri Dynamic Update Server](https://github.com/pAkalpa/tauri-dynamic-update-server) is a minimalistic server that can be used to serve updates to Tauri applications dynamically.

# 🧩 Develop

To download and run this rust/rocket project locally, the only prerequisite is rust with the `cargo` package manager.
Clone this repo, install the dependencies (all local), and run the development server:

```bash
git clone https://github.com/pAkalpa/tauri-dynamic-update-server.git
cd tauri-dynamic-update-server
cargo install
cargo run

# You will see something like:
#
#   >> port: 8000
#   ...
#   >> cli colors: true
#Routes:
#   >> (index) GET /
#   >> (get_update_data) GET /<target>?<version>&<arch>
#Fairings:
#   >> Add CORS headers to responses (response)
#   >> Shield (liftoff, response, singleton)
#Shield:
#   >> X-Frame-Options: SAMEORIGIN
#   >> X-Content-Type-Options: nosniff
#   >> Permissions-Policy: interest-cohort=()
#Rocket has launched from http://127.0.0.1:8000

```

The development api will be running on `http://localhost:8000`.

## 🛠️ Deploy from source

The _production_ build of the api is optimized for performance and is performed by the `cargo build --release` command,
after installing the required dependencies.

```bash
# .. repeat the steps above up to `npm install`, then:
cargo build --release
```

The app will be running on the specified port, e.g. `http://localhost:8000`.

## 🐳 Deploy with Docker

Build and run:

```bash
docker build -t dynamic-update-server .
docker run -e github_token=mygithubtoken -e github_repo_owner=mygithubusername -e github_repo_name=mytauriappreponame -d -p 8000:8000 dynamic-update-server
``` 

Or run the [published](https://hub.docker.com/r/pasinduakalpa/dynamicus) container:

- manually: `docker run -e github_token=mygithubtoken -e github_repo_owner=mygithubusername -e github_repo_name=mytauriappreponame -d -p 8000:8000 pasinduakalpa/dynamicus:latest`

---

2024 · [Pasindu Akalpa](https://www.github.com/pAkalpa) · License: [MIT](LICENSE) · Made with 💜
