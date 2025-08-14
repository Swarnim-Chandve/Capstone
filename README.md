# Capstone# Mirai - DAO Treasury Management Protocol

## 🛠️ Technology Stack

- **Blockchain**: Solana
- **Smart Contracts**: Rust + Anchor Framework
- **Testing**: TypeScript + Mocha + Anchor Test Framework
- **Development Tools**: Cargo, Anchor CLI, Solana CLI

## 📁 Project Structure

```
programs/mirai/
├── src/
│   ├── instructions/          # Program instructions
│   │   ├── init_dao.rs       # DAO initialization
│   │   ├── create_stream.rs  # Stream creation
│   │   ├── redeem_stream.rs  # Stream redemption
│   │   ├── create_vesting.rs # Vesting creation
│   │   └── claim_vesting.rs  # Vesting claims
│   ├── state/                 # Program state structures
│   │   ├── config.rs         # DAO configuration
│   │   ├── stream.rs         # Stream state management
│   │   └── vesting.rs        # Vesting state management
│   ├── errors/                # Custom error definitions
│   └── lib.rs                # Main program module
├── tests/                     # Test suite
└── Cargo.toml                # Rust dependencies
```



## 🏗️ Architecture and Working Diagram

[![Mirai Architecture Diagram](./diag.png)](./diag.png)

For a detailed visual representation of Mirai's modular architecture, marketplace integration, and system flow, please refer to our interactive diagram:

**[View Mirai Architecture Diagram](https://excalidraw.com/#json=7OVJn4MvJ8u_AfwBMSrxw,WNT-mnRvRhLTNm9VZtKUrA)**

## 🔧 Installation & Setup

### Prerequisites
- Rust (latest stable version)
- Solana CLI (latest version)
- Anchor CLI (latest version)
- Node.js and Yarn

### Setup Instructions
```bash
# Clone the repository
git clone https://github.com/Swarnim-Chandve/Capstone.git
cd Capstone

# Install dependencies
yarn install

# Build the program
anchor build

# Run tests
anchor test
```

## Devnet Deployment:

Signature: [5RP5KyR7CZ9pvHTbWS6FTLS1BcRXzSFFobnhMWXsjXF69NNCUCniB1fv7cPxHd7jwF7MVKqqQLbDZVkWMAm2MKGq](https://explorer.solana.com/tx/5RP5KyR7CZ9pvHTbWS6FTLS1BcRXzSFFobnhMWXsjXF69NNCUCniB1fv7cPxHd7jwF7MVKqqQLbDZVkWMAm2MKGq?cluster=devnet)
   

## 🧪 Testing

The project includes comprehensive test coverage with 28 test cases covering:

- ✅ DAO initialization and governance
- ✅ Stream creation and redemption
- ✅ Vesting management and claims
- ✅ Treasury analytics and tracking
- ✅ Error handling and validation
- ✅ Payment categorization
- ✅ Stream status management

Run the test suite:
```bash
anchor test
```

## 📊 Smart Contract Architecture

### Core Components

#### DAO Configuration
- Treasury mint management
- Governance settings
- Authority controls
- Payment category definitions

#### Stream Management
- Time-based token unlocking
- Linear distribution algorithms
- Status tracking and completion
- Treasury integration

#### Vesting System
- Linear and cliff vesting support
- Claimable amount calculations
- Progress tracking
- Status management (Active, Paused, Completed, Cancelled)

### Security Features
- **PDA Validation**: Program Derived Address verification
- **Authority Checks**: Multi-level authorization controls
- **Input Validation**: Comprehensive parameter validation
- **Error Handling**: Custom error types with descriptive messages

## 🔮 Future Enhancements

- **Multi-token Support**: Extend beyond SPL tokens
- **Advanced Analytics**: Enhanced reporting and visualization
- **Mobile Interface**: React Native or mobile web app
- **Integration APIs**: REST/GraphQL endpoints for external access
- **Governance Extensions**: Additional voting and proposal mechanisms

## 📝 License

This project is developed as part of the Turbine Cohort capstone project.

## 🤝 Contributing

This is a capstone project, but feedback and suggestions are welcome through GitHub issues.

---

**Mirai** - Building the future of DAO treasury management on Solana 🚀
