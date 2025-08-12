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

## Features

### SSH Key Management

- **View SSH Keys**: Scan and display all SSH keys in your `~/.ssh/` directory
- **SSH Agent Integration**: Show which keys are currently loaded in your SSH agent
- **Key Status**: Display whether each key is active (loaded) or inactive
- **Copy Public Keys**: Copy public key content to clipboard for easy sharing

### SSH Key Creation

- **Create New Keys**: Generate new SSH keys directly from the application
- **Multiple Key Types**: Support for RSA, Ed25519, and ECDSA key types
- **Configurable Key Sizes**:
  - RSA: 2048 or 4096 bits
  - ECDSA: 256, 384, or 521 bits
  - Ed25519: 256 bits (fixed)
- **Optional Passphrase**: Add passphrase protection to your keys
- **Smart Naming**: Automatic filename generation based on key type and email

### Key Types Supported

- **RSA**: Traditional RSA keys with configurable bit lengths
- **Ed25519**: Modern, secure keys (recommended for new deployments)
- **ECDSA**: Elliptic curve keys with various curve sizes
