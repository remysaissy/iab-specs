//! Agentic Direct v2.1 Specification
//!
//! This module implements the complete Agentic Direct v2.1 specification,
//! combining OpenDirect v2.1 with autonomous-to-autonomous (A2A) protocol support
//! and JSON-RPC 2.0 message routing for agent-to-agent communication.
//!
//! # Architecture
//!
//! The module is organized into:
//!
//! - [`enums`] - Enumerations for transaction states, roles, and protocol identifiers
//! - [`entities`] - Core data structures for deals, creatives, and agent metadata
//! - [`a2a`] - A2A protocol messages and message exchange patterns
//! - [`jsonrpc`] - JSON-RPC 2.0 message framing and routing
//!
//! # Quick Start
//!
//! ## Creating an Order
//!
//! ```rust
//! use iab_specs_agentic_direct::v21::{Order, OrderStatus, can_transition_order};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let order = Order::builder()
//!     .name("Summer Campaign")
//!     .account_id("acc-001")
//!     .publisher_id("pub-001")
//!     .currency("USD")
//!     .status(OrderStatus::Draft)
//!     .build()?;
//!
//! assert_eq!(order.status, OrderStatus::Draft);
//! assert!(can_transition_order(&OrderStatus::Draft, &OrderStatus::PendingReview));
//! # Ok(())
//! # }
//! ```
//!
//! ## Creating an Agent Card
//!
//! ```rust
//! use iab_specs_agentic_direct::v21::{
//!     AgentCard, Skill, SkillInputMode, AgentCapabilities,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let card = AgentCard::builder()
//!     .name("Ad Negotiation Agent")
//!     .version("1.0.0")
//!     .protocol_version("0.3.0")
//!     .url("https://agent.example.com")
//!     .skills(vec![
//!         Skill::builder()
//!             .id("negotiate-order")
//!             .name("Order Negotiation")
//!             .description("Negotiate advertising orders")
//!             .input_modes(vec![SkillInputMode::Text, SkillInputMode::Data])
//!             .build()?,
//!     ])
//!     .capabilities(Some(AgentCapabilities::builder()
//!         .streaming(Some(true))
//!         .push_notifications(Some(true))
//!         .build()?))
//!     .build()?;
//!
//! // A2A types serialize with camelCase
//! let json = serde_json::to_string(&card)?;
//! assert!(json.contains("protocolVersion"));
//! # Ok(())
//! # }
//! ```
//!
//! ## Sending a JSON-RPC Request
//!
//! ```rust
//! use iab_specs_agentic_direct::v21::{JsonRpcRequest, JsonRpcResponse, JsonRpcId};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let request = JsonRpcRequest::builder()
//!     .jsonrpc("2.0")
//!     .method("agent/negotiate")
//!     .id(Some(JsonRpcId::String("req-1".into())))
//!     .params(Some(serde_json::json!({"deal_id": "d-100"})))
//!     .build()?;
//!
//! let response = JsonRpcResponse::builder()
//!     .jsonrpc("2.0")
//!     .id(JsonRpcId::String("req-1".into()))
//!     .result(Some(serde_json::json!({"status": "accepted"})))
//!     .build()?;
//!
//! assert_eq!(request.id.unwrap(), response.id);
//! # Ok(())
//! # }
//! ```
//!
//! # Extension Support
//!
//! All entity objects support custom extensions via the generic `Ext` parameter.
//! By default, extensions use [`DefaultExt`](crate::DefaultExt) which is `Vec<u8>` (opaque bytes).
//! Callers can use `serde_json::Value` or custom types as explicit type parameters.
//!
//! # Serialization Conventions
//!
//! - **OpenDirect entities** (Organization, Account, Order, Line, etc.) use `snake_case` JSON keys
//! - **A2A protocol types** (AgentCard, Skill, A2ATask, etc.) use `camelCase` JSON keys
//! - **JSON-RPC types** use the standard JSON-RPC 2.0 field names
//!
//! # Specification Reference
//!
//! This implementation follows the [Agentic Direct v2.1](https://github.com/IABTechLab/agentic-direct)
//! specification published by IAB Tech Lab, combining OpenDirect v2.1 with the A2A Protocol.

pub mod a2a;
pub mod entities;
pub mod enums;
pub mod jsonrpc;

pub use a2a::*;
pub use entities::*;
pub use enums::*;
pub use jsonrpc::*;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_opendirect_workflow() {
        let org = Organization::builder()
            .id("org-001")
            .name("Acme Advertising")
            .type_(Some(OrganizationType::Advertiser))
            .address(Some(
                Address::builder()
                    .city("New York")
                    .country("USA")
                    .build()
                    .unwrap(),
            ))
            .contacts(vec![
                Contact::builder()
                    .name("Jane Smith")
                    .email("jane@acme.com")
                    .role("Media Buyer")
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        assert_eq!(org.name, "Acme Advertising");
        assert_eq!(org.type_, Some(OrganizationType::Advertiser));

        let account = Account::builder()
            .id("acc-001")
            .advertiser_id(org.id.clone().unwrap())
            .buyer_id("buyer-001")
            .name("Acme Q3 Account")
            .status(Some(AccountStatus::Active))
            .build()
            .unwrap();

        assert_eq!(account.advertiser_id, "org-001");
        assert_eq!(account.status, Some(AccountStatus::Active));

        let product = Product::builder()
            .id("prod-001")
            .publisher_id("pub-001")
            .name("Premium Display")
            .description("Above-the-fold display inventory")
            .availability(Some(ProductAvailability::Available))
            .base_price(Some(10.0))
            .rate_type(Some(RateType::Cpm))
            .delivery_type(Some(DeliveryType::Guaranteed))
            .currency("USD")
            .targeting(Some(serde_json::json!({"geos": ["US", "CA"]})))
            .build()
            .unwrap();

        assert_eq!(product.name, "Premium Display");
        assert_eq!(product.base_price, Some(10.0));

        let order = Order::builder()
            .id("order-001")
            .name("Acme Summer Campaign")
            .account_id(account.id.clone().unwrap())
            .publisher_id("pub-001")
            .status(OrderStatus::Draft)
            .currency("USD")
            .budget(Some(50000.0))
            .start_date("2025-06-01")
            .end_date("2025-08-31")
            .build()
            .unwrap();

        assert_eq!(order.status, OrderStatus::Draft);

        assert!(can_transition_order(
            &OrderStatus::Draft,
            &OrderStatus::PendingReview
        ));
        assert!(!can_transition_order(
            &OrderStatus::Draft,
            &OrderStatus::InProgress
        ));

        let line = Line::builder()
            .id("line-001")
            .name("Display Banner 300x250")
            .order_id(order.id.clone().unwrap())
            .product_id(product.id.clone().unwrap())
            .status(LineStatus::Draft)
            .start_date("2025-06-01")
            .end_date("2025-08-31")
            .rate_type(RateType::Cpm)
            .rate(Some(10.0))
            .quantity(500000)
            .budget(Some(5000.0))
            .frequency_cap(Some(
                FrequencyCap::builder()
                    .count(3)
                    .period_seconds(86400)
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert_eq!(line.status, LineStatus::Draft);
        assert_eq!(line.quantity, 500000);

        assert!(can_transition_line(
            &LineStatus::Draft,
            &LineStatus::PendingReview
        ));
        assert!(!can_transition_line(
            &LineStatus::Draft,
            &LineStatus::Booked
        ));

        let creative = Creative::builder()
            .id("cre-001")
            .name("Summer Sale Banner")
            .account_id(account.id.clone().unwrap())
            .status(Some(CreativeStatus::Draft))
            .ad_format("display")
            .width(Some(300))
            .height(Some(250))
            .click_url("https://acme.com/summer-sale")
            .markup("<div>Summer Sale - 50% Off!</div>")
            .build()
            .unwrap();

        assert_eq!(creative.name, "Summer Sale Banner");
        assert_eq!(creative.width, Some(300));

        let assignment = Assignment::builder()
            .id("assign-001")
            .creative_id(creative.id.clone().unwrap())
            .line_id(line.id.clone().unwrap())
            .status(AssignmentStatus::Draft)
            .weight(Some(1.0))
            .build()
            .unwrap();

        assert_eq!(assignment.creative_id, "cre-001");
        assert_eq!(assignment.line_id, "line-001");
        assert_eq!(assignment.status, AssignmentStatus::Draft);

        let org_json = serde_json::to_string(&org).unwrap();
        let parsed_org: Organization = serde_json::from_str(&org_json).unwrap();
        assert_eq!(org, parsed_org);

        let order_json = serde_json::to_string(&order).unwrap();
        let parsed_order: Order = serde_json::from_str(&order_json).unwrap();
        assert_eq!(order, parsed_order);

        let line_json = serde_json::to_string(&line).unwrap();
        let parsed_line: Line = serde_json::from_str(&line_json).unwrap();
        assert_eq!(line, parsed_line);
    }

    #[test]
    fn test_agent_card_with_full_skills() {
        let card = AgentCard::builder()
            .name("AdTech Negotiation Agent")
            .description("Autonomous agent for advertising deal negotiation")
            .version("2.1.0")
            .protocol_version("0.3.0")
            .url("https://agent.adtech.example.com")
            .skills(vec![
                Skill::builder()
                    .id("negotiate-order")
                    .name("Order Negotiation")
                    .description("Negotiate advertising orders with publishers")
                    .tags(vec![
                        "advertising".to_string(),
                        "negotiation".to_string(),
                        "opendirect".to_string(),
                    ])
                    .examples(vec![
                        "Negotiate a $50K display campaign".to_string(),
                        "Request premium inventory at CPM $10".to_string(),
                    ])
                    .input_modes(vec![SkillInputMode::Text, SkillInputMode::Data])
                    .build()
                    .unwrap(),
                Skill::builder()
                    .id("manage-creative")
                    .name("Creative Management")
                    .description("Submit and manage ad creatives")
                    .tags(vec!["creative".to_string(), "display".to_string()])
                    .input_modes(vec![SkillInputMode::File, SkillInputMode::Data])
                    .build()
                    .unwrap(),
            ])
            .capabilities(Some(
                AgentCapabilities::builder()
                    .push_notifications(Some(true))
                    .streaming(Some(true))
                    .mcp_integration(Some(false))
                    .build()
                    .unwrap(),
            ))
            .additional_interfaces(vec![
                AgentInterface::builder()
                    .protocol(ProtocolType::Mcp)
                    .version("1.0")
                    .transport(TransportType::Sse)
                    .url("https://agent.adtech.example.com/mcp")
                    .build()
                    .unwrap(),
            ])
            .security_schemes(vec![
                SecurityScheme::builder()
                    .type_(SecuritySchemeType::OAuth2)
                    .description("OAuth2 client credentials")
                    .flows(Some(serde_json::json!({
                        "clientCredentials": {
                            "tokenUrl": "https://auth.example.com/token",
                            "scopes": {"negotiate": "Negotiate orders"}
                        }
                    })))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        assert_eq!(card.skills.len(), 2);
        assert_eq!(card.additional_interfaces.len(), 1);
        assert_eq!(card.security_schemes.len(), 1);

        let json = serde_json::to_string(&card).unwrap();
        assert!(
            json.contains("\"protocolVersion\""),
            "AgentCard should use camelCase"
        );
        assert!(
            json.contains("\"pushNotifications\""),
            "Capabilities should use camelCase"
        );
        assert!(
            json.contains("\"inputModes\""),
            "Skill should use camelCase"
        );
        assert!(
            json.contains("\"additionalInterfaces\""),
            "AgentCard should use camelCase"
        );
        assert!(
            json.contains("\"securitySchemes\""),
            "AgentCard should use camelCase"
        );

        let parsed: AgentCard = serde_json::from_str(&json).unwrap();
        assert_eq!(card, parsed);
    }

    #[test]
    fn test_jsonrpc_request_response_cycle() {
        let request = JsonRpcRequest::builder()
            .jsonrpc("2.0")
            .method("agent/negotiate")
            .id(Some(JsonRpcId::String("req-negotiate-001".into())))
            .params(Some(serde_json::json!({
                "order_id": "order-001",
                "action": "submit_for_review"
            })))
            .build()
            .unwrap();

        assert_eq!(request.method, "agent/negotiate");

        let success_response = JsonRpcResponse::builder()
            .jsonrpc("2.0")
            .id(request.id.clone().unwrap())
            .result(Some(serde_json::json!({
                "status": "accepted",
                "order_id": "order-001",
                "new_status": "pending_review"
            })))
            .build()
            .unwrap();

        assert_eq!(request.id.as_ref().unwrap().clone(), success_response.id);
        assert!(success_response.result.is_some());
        assert!(success_response.error.is_none());

        let error_response = JsonRpcResponse::builder()
            .jsonrpc("2.0")
            .id(request.id.clone().unwrap())
            .error(Some(
                JsonRpcError::builder()
                    .code(INVALID_PARAMS)
                    .message("Order not in valid state for review submission")
                    .data(Some(serde_json::json!({
                        "current_status": "in_progress",
                        "allowed_transitions": ["paused", "completed", "cancelled"]
                    })))
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        assert!(error_response.result.is_none());
        assert!(error_response.error.is_some());
        assert_eq!(error_response.error.as_ref().unwrap().code, INVALID_PARAMS);

        let notification = JsonRpcNotification::builder()
            .jsonrpc("2.0")
            .method("agent/statusUpdate")
            .params(Some(serde_json::json!({
                "order_id": "order-001",
                "status": "approved"
            })))
            .build()
            .unwrap();

        let notif_json = serde_json::to_string(&notification).unwrap();
        assert!(!notif_json.contains("\"id\""));

        let req_json = serde_json::to_string(&request).unwrap();
        let parsed_req: JsonRpcRequest = serde_json::from_str(&req_json).unwrap();
        assert_eq!(request, parsed_req);

        let resp_json = serde_json::to_string(&success_response).unwrap();
        let parsed_resp: JsonRpcResponse = serde_json::from_str(&resp_json).unwrap();
        assert_eq!(success_response, parsed_resp);

        let err_json = serde_json::to_string(&error_response).unwrap();
        let parsed_err: JsonRpcResponse = serde_json::from_str(&err_json).unwrap();
        assert_eq!(error_response, parsed_err);
    }

    #[test]
    fn test_state_machine_transitions_in_sequence() {
        let order_path = vec![
            (OrderStatus::Draft, OrderStatus::PendingReview),
            (OrderStatus::PendingReview, OrderStatus::Approved),
            (OrderStatus::Approved, OrderStatus::InProgress),
            (OrderStatus::InProgress, OrderStatus::Completed),
        ];
        for (from, to) in &order_path {
            assert!(
                can_transition_order(from, to),
                "Order: {:?} → {:?} should be valid",
                from,
                to
            );
        }

        let order_transitions = valid_order_transitions_from(&OrderStatus::Completed);
        assert!(
            order_transitions.is_empty(),
            "Completed orders should have no outgoing transitions"
        );

        let line_path = vec![
            (LineStatus::Draft, LineStatus::PendingReview),
            (LineStatus::PendingReview, LineStatus::Reserved),
            (LineStatus::Reserved, LineStatus::Booked),
            (LineStatus::Booked, LineStatus::InProgress),
            (LineStatus::InProgress, LineStatus::Completed),
        ];
        for (from, to) in &line_path {
            assert!(
                can_transition_line(from, to),
                "Line: {:?} → {:?} should be valid",
                from,
                to
            );
        }

        let line_transitions = valid_line_transitions_from(&LineStatus::Completed);
        assert!(
            line_transitions.is_empty(),
            "Completed lines should have no outgoing transitions"
        );

        let task_path = vec![
            (TaskState::Working, TaskState::InputRequired),
            (TaskState::InputRequired, TaskState::Working),
            (TaskState::Working, TaskState::Completed),
        ];
        for (from, to) in &task_path {
            assert!(
                can_transition_task(from, to),
                "Task: {:?} → {:?} should be valid",
                from,
                to
            );
        }

        for terminal in &[
            TaskState::Completed,
            TaskState::Failed,
            TaskState::Cancelled,
        ] {
            let transitions = valid_task_transitions_from(terminal);
            assert!(
                transitions.is_empty(),
                "Terminal task state {:?} should have no outgoing transitions",
                terminal
            );
        }
    }

    #[test]
    fn test_a2a_task_with_artifacts_workflow() {
        let task = A2ATask::builder()
            .id("task-negotiate-001")
            .state(TaskState::Working)
            .message("Processing order negotiation")
            .created_at("2025-06-01T10:00:00Z")
            .build()
            .unwrap();

        assert_eq!(task.state, TaskState::Working);

        assert!(can_transition_task(
            &TaskState::Working,
            &TaskState::Completed
        ));

        let completed_task = A2ATask::builder()
            .id("task-negotiate-001")
            .state(TaskState::Completed)
            .message("Order negotiation complete")
            .artifacts(vec![
                A2AArtifact::builder()
                    .name("negotiation_result.json")
                    .description("Final negotiated terms")
                    .parts(vec![
                        A2AArtifactPart::builder()
                            .type_("application/json")
                            .content(
                                serde_json::json!({
                                    "order_id": "order-001",
                                    "agreed_cpm": 8.50,
                                    "impressions": 1000000
                                })
                                .to_string(),
                            )
                            .build()
                            .unwrap(),
                    ])
                    .build()
                    .unwrap(),
                A2AArtifact::builder()
                    .name("audit_log.txt")
                    .parts(vec![
                        A2AArtifactPart::builder()
                            .type_("text/plain")
                            .content("Negotiation completed in 3 rounds")
                            .build()
                            .unwrap(),
                    ])
                    .build()
                    .unwrap(),
            ])
            .created_at("2025-06-01T10:00:00Z")
            .updated_at("2025-06-01T10:05:00Z")
            .build()
            .unwrap();

        assert_eq!(completed_task.state, TaskState::Completed);
        assert_eq!(completed_task.artifacts.len(), 2);

        let json = serde_json::to_string(&completed_task).unwrap();
        assert!(json.contains("\"createdAt\""));
        assert!(json.contains("\"updatedAt\""));
        assert!(!json.contains("\"created_at\""));

        let parsed: A2ATask = serde_json::from_str(&json).unwrap();
        assert_eq!(completed_task, parsed);
    }

    #[test]
    fn test_serde_case_conventions() {
        let order = Order::builder()
            .name("Test")
            .account_id("acc-1")
            .publisher_id("pub-1")
            .status(OrderStatus::Draft)
            .currency("USD")
            .start_date("2025-01-01")
            .end_date("2025-12-31")
            .build()
            .unwrap();

        let order_json = serde_json::to_string(&order).unwrap();
        assert!(
            order_json.contains("\"account_id\""),
            "OpenDirect entities should use snake_case"
        );
        assert!(
            order_json.contains("\"publisher_id\""),
            "OpenDirect entities should use snake_case"
        );
        assert!(
            order_json.contains("\"start_date\""),
            "OpenDirect entities should use snake_case"
        );

        let agent_card = AgentCard::builder()
            .name("Test Agent")
            .version("1.0")
            .protocol_version("0.3.0")
            .url("https://agent.example.com")
            .capabilities(Some(
                AgentCapabilities::builder()
                    .push_notifications(Some(true))
                    .mcp_integration(Some(false))
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        let card_json = serde_json::to_string(&agent_card).unwrap();
        assert!(
            card_json.contains("\"protocolVersion\""),
            "A2A types should use camelCase"
        );
        assert!(
            card_json.contains("\"pushNotifications\""),
            "A2A types should use camelCase"
        );
        assert!(
            card_json.contains("\"mcpIntegration\""),
            "A2A types should use camelCase"
        );
    }

    #[test]
    fn test_jsonrpc_wrapping_opendirect_entities() {
        let order = Order::builder()
            .id("order-wrap-001")
            .name("Wrapped Order")
            .account_id("acc-1")
            .publisher_id("pub-1")
            .status(OrderStatus::Draft)
            .currency("USD")
            .build()
            .unwrap();

        let order_json = serde_json::to_value(&order).unwrap();
        let request = JsonRpcRequest::builder()
            .jsonrpc("2.0")
            .method("order/create")
            .id(Some(JsonRpcId::Number(1)))
            .params(Some(order_json.clone()))
            .build()
            .unwrap();

        let params = request.params.as_ref().unwrap();
        assert_eq!(params["name"], "Wrapped Order");
        assert_eq!(params["status"], "draft");

        let response = JsonRpcResponse::builder()
            .jsonrpc("2.0")
            .id(JsonRpcId::Number(1))
            .result(Some(order_json))
            .build()
            .unwrap();

        let result_order: Order = serde_json::from_value(response.result.unwrap()).unwrap();
        assert_eq!(result_order, order);
    }

    #[test]
    fn test_change_request_and_placement_workflow() {
        let change_request = ChangeRequest::builder()
            .id("cr-001")
            .order_id("order-001")
            .change_type("date_shift")
            .description("Shift campaign start date by 2 weeks")
            .requested_changes(serde_json::json!({
                "field": "start_date",
                "old_value": "2025-06-01",
                "new_value": "2025-06-15"
            }))
            .status("pending")
            .build()
            .unwrap();

        assert_eq!(change_request.order_id, "order-001");
        assert_eq!(change_request.change_type, "date_shift");
        assert_eq!(change_request.status, "pending");

        let placement = Placement::builder()
            .id("pl-001")
            .line_id("line-001")
            .ad_unit_code("HEADER_LEADERBOARD_728x90")
            .overrides(Some(serde_json::json!({
                "viewability_threshold": 0.7,
                "position": "above_fold"
            })))
            .build()
            .unwrap();

        assert_eq!(placement.line_id, "line-001");
        assert_eq!(placement.ad_unit_code, "HEADER_LEADERBOARD_728x90");
        assert!(placement.overrides.is_some());

        let cr_json = serde_json::to_string(&change_request).unwrap();
        let parsed_cr: ChangeRequest = serde_json::from_str(&cr_json).unwrap();
        assert_eq!(change_request, parsed_cr);

        let pl_json = serde_json::to_string(&placement).unwrap();
        let parsed_pl: Placement = serde_json::from_str(&pl_json).unwrap();
        assert_eq!(placement, parsed_pl);
    }

    #[test]
    fn test_mcp_tool_and_task_history_workflow() {
        let tool = MCPTool::builder()
            .name("get_inventory")
            .description("Retrieve available advertising inventory")
            .input_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "publisher_id": {"type": "string"},
                    "ad_format": {"type": "string", "enum": ["display", "video", "native"]}
                },
                "required": ["publisher_id"]
            }))
            .build()
            .unwrap();

        assert_eq!(tool.name, "get_inventory");

        let tool_json = serde_json::to_string(&tool).unwrap();
        let parsed_tool: MCPTool = serde_json::from_str(&tool_json).unwrap();
        assert_eq!(tool, parsed_tool);

        let task = A2ATask::builder()
            .id("task-hist-001")
            .state(TaskState::Completed)
            .message("Negotiation complete")
            .history(vec![
                A2AMessage::builder()
                    .role("user")
                    .content("Please negotiate order-001 with budget $50K")
                    .timestamp("2025-06-01T10:00:00Z")
                    .build()
                    .unwrap(),
                A2AMessage::builder()
                    .role("agent")
                    .content("Analyzing publisher inventory and pricing...")
                    .timestamp("2025-06-01T10:00:05Z")
                    .build()
                    .unwrap(),
                A2AMessage::builder()
                    .role("agent")
                    .content("Negotiation complete. Agreed CPM: $8.50 for 1M impressions")
                    .timestamp("2025-06-01T10:05:00Z")
                    .build()
                    .unwrap(),
            ])
            .created_at("2025-06-01T10:00:00Z")
            .updated_at("2025-06-01T10:05:00Z")
            .build()
            .unwrap();

        assert_eq!(task.history.len(), 3);
        assert_eq!(task.history[0].role, "user");
        assert_eq!(task.history[2].role, "agent");

        let json = serde_json::to_string(&task).unwrap();
        assert!(json.contains("\"history\""));
        assert!(json.contains("\"role\""));
        assert!(json.contains("\"content\""));

        let parsed: A2ATask = serde_json::from_str(&json).unwrap();
        assert_eq!(task, parsed);
        assert_eq!(parsed.history.len(), 3);
    }
}
