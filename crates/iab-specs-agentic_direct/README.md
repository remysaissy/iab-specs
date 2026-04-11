# iab-specs-agentic_direct

Agentic Direct v2.1 support for the [iab-specs](https://crates.io/crates/iab-specs) ecosystem.

## Overview

Implements the Agentic Direct v2.1 specification combining OpenDirect v2.1 with the A2A Protocol for autonomous agent-to-agent advertising transactions and JSON-RPC 2.0 message routing.

- **OpenDirect v2.1 entities** — Organization, Account, Product, Order, Line, Creative, Assignment, ChangeRequest, Placement
- **A2A Protocol** — AgentCard, Skill, A2ATask, A2AMessage, A2AArtifact
- **JSON-RPC 2.0** — Request, Response, Notification, Error, MCPTool
- **State machines** — Order, Line, and Task lifecycle management
- **Enumerations** — OrderStatus, LineStatus, TaskState, RateType, DeliveryType, and more

## License

Apache-2.0
