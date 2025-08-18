# this.ssh Documentation

## Project Overview

This is a desktop application built with **Tauri** (Rust backend + NUXT frontend) that provides a user-friendly interface for managing SSH keys on your system. The application allows users to view, create, and remove SSH keys while providing real-time status information about loaded keys in the SSH agent.

## Architecture

### Backend (Rust/Tauri)

- **Framework**: Tauri with Rust
- **Purpose**: System-level SSH operations and file management
- **Key Functions**:
  - SSH key discovery and parsing
  - Key creation and removal
  - SSH agent status monitoring
  - File system operations

### Frontend (Nuxt)

- **Framework**: Nuxt with Composition API
- **Styling**: Tailwind CSS with custom CSS variables
- **State Management**: Nuxt reactive system with custom composables
- **Components**: Modular, reusable components for different UI elements

## Core Features

### 1. SSH Key Discovery

- **Function**: `get_ssh_keys()`
- **Purpose**: Scans `~/.ssh/` directory for public key files
- **Output**: Returns structured data including:
  - Filename (for key removal)
  - Key information (type, size, email, fingerprint)
  - Error handling for corrupted or invalid keys

### 2. SSH Agent Monitoring

- **Function**: `is_ssh_agent_running()`
- **Purpose**: Checks if SSH agent is active
- **Function**: `get_loaded_ssh_agent_keys()`
- **Purpose**: Lists currently loaded keys in SSH agent
- **Output**: Shows which keys are actively available for SSH connections

### 3. SSH Key Creation

- **Function**: `create_ssh_key()`
- **Parameters**:
  - Email address
  - Key type (RSA, ED25519, etc.)
  - Key size (bits)
  - Optional passphrase
- **Features**:
  - Automatic filename generation
  - Duplicate key prevention
  - Directory creation if needed

### 4. SSH Key Removal

- **Function**: `remove_ssh_key()`
- **Parameters**: Filename (without extension)
- **Safety**: Removes both private and public key files
- **Confirmation**: User confirmation modal before deletion

## Data Structures

### SSHKeyInfo (Backend)

```rust
struct SSHKeyInfo {
    filename: String,    // Key filename (e.g., "id_ed25519_example")
    key_info: String,    // ssh-keygen -lv output
}
```

### Key (Frontend)

```typescript
interface Key {
  keyPid: string; // Key fingerprint/size
  publicKey: string; // Public key hash
  email: string; // Associated email
  isActive: boolean; // Whether key is loaded in agent
  keyType: string; // Key algorithm (ED25519, RSA, etc.)
  filename: string; // Filename for removal operations
}
```

### State Management

```typescript
interface State {
  status: "success" | "failed" | "loading" | "idle";
  message?: string;
  data?: object;
}
```

## Component Architecture

### Core Components

#### 1. Main.vue

- **Purpose**: Application root component
- **Features**: Debug mode toggle, main layout management

#### 2. Keys.vue

- **Purpose**: Main SSH key management interface
- **Features**:
  - Key list display
  - Status management
  - Error handling
  - Debug information display

#### 3. KeyCard.vue

- **Purpose**: Individual key display card
- **Features**:
  - Key information display
  - Active status indicator
  - Action menu integration

#### 4. KeyCardMenu.vue

- **Purpose**: Context menu for key actions
- **Features**:
  - Copy public key
  - Remove key option

#### 5. CreateKeyModal.vue

- **Purpose**: SSH key creation interface
- **Features**:
  - Form validation
  - Key type selection
  - Passphrase input
  - Error handling

#### 6. RemoveKeyConfirmationModal.vue

- **Purpose**: Key removal confirmation
- **Features**:
  - Key details display
  - Warning messages
  - Confirmation buttons

#### 7. Modal.vue

- **Purpose**: Reusable modal system
- **Features**:
  - Slot-based architecture
  - Responsive design
  - Backdrop handling

## Key Workflows

### 1. Application Startup

1. Component mounts
2. `fetchSSHKeys()` called automatically
3. `getLoadedSSHAgentKeys()` called automatically
4. UI updates with current key status

### 2. SSH Key Discovery

1. Backend scans `~/.ssh/` directory
2. Identifies `.pub` files
3. Runs `ssh-keygen -lv` on each file
4. Parses output for key information
5. Returns structured data to frontend

### 3. Key Creation

1. User opens CreateKeyModal
2. Fills in key details (email, type, size, passphrase)
3. Frontend validates input
4. Backend executes `ssh-keygen` command
5. Success/error feedback to user
6. Key list refreshes automatically

### 4. Key Removal

1. User clicks remove button on KeyCard
2. RemoveKeyConfirmationModal opens
3. User confirms deletion
4. Backend removes key files using filename
5. Success feedback and modal closes
6. Key list refreshes automatically

### 5. Status Monitoring

1. Frontend polls for SSH agent status
2. Compares available keys vs. loaded keys
3. Updates UI to show active/inactive status
4. Real-time status indicators

## Error Handling

### Backend Errors

- **File system errors**: Directory access, file permissions
- **Command execution errors**: `ssh-keygen` failures
- **Validation errors**: Invalid parameters, duplicate keys

### Frontend Errors

- **Network errors**: Tauri command failures
- **Parsing errors**: Invalid key data format
- **User input errors**: Form validation failures

### Error Recovery

- **Automatic retries**: Key list refresh on errors
- **User feedback**: Clear error messages
- **Graceful degradation**: Partial functionality when possible

## Security Considerations

### Key Storage

- Keys stored in standard `~/.ssh/` location
- Follows system SSH conventions
- No additional encryption beyond system defaults

### Key Operations

- **Creation**: Uses system `ssh-keygen` command
- **Removal**: Direct file deletion (irreversible)
- **Display**: Shows public key information only

### Agent Integration

- Monitors SSH agent status
- No direct agent manipulation
- Read-only access to loaded keys

## Development Setup

### Prerequisites

- Rust toolchain
- Node.js and npm
- Tauri CLI
- SSH tools (`ssh-keygen`, `ssh-add`)

### Build Commands

```bash
# Install dependencies
npm install

# Development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Project Structure

```
thisdotssh/
├── src-tauri/          # Rust backend
│   ├── src/lib.rs      # Main backend logic
│   └── Cargo.toml      # Rust dependencies
├── components/          # Vue components
├── types/              # TypeScript type definitions
├── composables/        # Vue composables
└── assets/             # CSS and static assets
```

## Testing

### Manual Testing

1. **Key Discovery**: Verify existing keys are detected
2. **Key Creation**: Test key generation with different parameters
3. **Key Removal**: Test deletion with confirmation
4. **Status Updates**: Verify active/inactive status accuracy

### Edge Cases

- Empty `.ssh` directory
- Corrupted key files
- Missing SSH tools
- Permission issues

## Future Enhancements

### Potential Features

- **Key Import**: Import existing keys from other locations
- **Key Backup**: Export/backup functionality
- **Advanced Key Types**: Support for more SSH key algorithms
- **Key Rotation**: Automated key replacement workflows
- **Server Integration**: Direct server key deployment

### Technical Improvements

- **Performance**: Optimize key scanning for large directories
- **Caching**: Implement key data caching
- **Real-time Updates**: File system watching for changes
- **Cross-platform**: Enhanced Windows/macOS support

## Troubleshooting

### Common Issues

#### Keys Not Showing

- Check `~/.ssh/` directory permissions
- Verify SSH tools are installed
- Check console for error messages

#### Key Creation Fails

- Verify email format
- Check key type/size compatibility
- Ensure sufficient disk space

#### Key Removal Fails

- Verify filename is correct
- Check file permissions
- Ensure key is not in use

#### Status Not Updating

- Verify SSH agent is running
- Check `SSH_AUTH_SOCK` environment variable
- Restart SSH agent if needed

### Debug Mode

Enable debug mode in the application to see:

- Raw key data from backend
- Parsing results
- Component state information
- Error details

## Contributing

### Code Style

- **Rust**: Follow Rust formatting guidelines
- **Vue/Nuxt**: Use Composition API, TypeScript
- **CSS**: Tailwind classes with custom variables

### Testing

- Test all key operations manually
- Verify error handling scenarios
- Check responsive design on different screen sizes

### Documentation

- Update this file for new features
- Document API changes
- Include usage examples

## License

Apache-2.0 License

## Support

You can reach out to me on discord @dk3tm or email me at cyberhybird@hotmail.com
