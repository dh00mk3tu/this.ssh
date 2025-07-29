### Building From Source

To build this.ssh from source, you will need to have the following libs installed:

- Node.js (v22.0.0 or later)
- npm (v9.0.0 or later)
- TypeScript (v5.0.0 or later)
- Rust Compiler (v1.88.0 or later)
- Cargo (v1.88.0 or later)
- Tauri (v2.6.2, not tested on version v2.7.0 or later)

Clone the repository:

```bash
git clone git@github.com:dh00mk3tu/this.ssh.git
```

Navigate to the project directory:

```bash
cd this.ssh
```

Change branch to `master`:
Master branch has the latest stable code.

```bash
git checkout master
```

Install the dependencies:

```bash
npm install
```

Build the project:

```bash
npm run tauri build
```

#### Running the Project Locally

To run and to test the project locally, you need to run both the Tauri and the NUXT development servers.
Ideally I run one terminal and split it into two panes, one for each server (tmux), or tabs.

Start the Tauri development server:

```bash
cargo tauri dev
```

Start the NUXT development server:

```bash
npm run dev
```

The application will fire a native window and the application will work within that window only. The application will not work in a browser, it is a native application amd Tauri APIs do not work in a browser environment.
